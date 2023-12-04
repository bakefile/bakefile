use std::fs;
use crate::ing::{Instruction, Recipe};
use crate::errors::Error;

fn comment_start(c: char) -> bool {
    match c  {
        '₽' | '₪' | '₠' | '₤' | '₦' | '€' | '₢' | '₧' => true,
        _ => false
    }
}

fn new_line(c: char) -> bool {
    return c == '\n'
}


pub fn parse_recipe_from_path(path: &str) -> Result<Recipe, Error> {
    let unparsed_file = fs::read_to_string(path).expect(&format!("failed to read path {}", path));
    parse_recipe(&unparsed_file)
}


pub fn parse_recipe(data: &str) -> Result<Recipe, Error> {
    let mut recipe = Recipe::blank();
    let mut instruction = Instruction::new("");
    let mut target_name = String::new();
    let mut current_dependency = String::new();
    let mut shell_command = String::new();
    let mut inshell = false;
    let mut dependency = false;
    let mut indent = 0;
    let mut lineno = 1;
    let mut lpos = 1;
    let mut pos = 0;
    let mut incomment = false;
    let indentation = 4;
    for c in data.chars() {
        pos += 1;

        if comment_start(c) {
            incomment = true;
            continue;
        } else if new_line(c) {
            if dependency {
                if !current_dependency.is_empty() {
                    instruction.add_dependency(&current_dependency);
                    current_dependency.clear();
                }
            } else if inshell {
                if !shell_command.is_empty() {
                    instruction.add_action(&shell_command);
                    shell_command.clear();
                }
            }
            lineno += 1;
            lpos = 0;
            indent = 0;
            incomment = false;
            inshell = false;
            dependency = false;

            continue;
        }
        if incomment {
            continue;
        }
        lpos += 1;

        match c {
            ':' => {
                match indent {
                    0 => {
                        instruction.set_label(&target_name);
                        target_name.clear();
                        dependency = true;
                    },
                    i => if i == indentation {
                        shell_command.push(c)
                    },
                }
                continue;
            },
            ' ' => {
                if dependency {
                    if current_dependency.len() > 0 {
                        instruction.add_dependency(&current_dependency);
                        current_dependency.clear();
                    }
                    continue
                }
                if !inshell && !incomment {
                    indent += 1;
                } else {
                    shell_command.push(c)
                }
                continue;
            },
            '\r' => {
                lpos = 0;
            },
            _ => {
                if dependency {
                    current_dependency.push(c);
                    continue;
                } else if inshell {
                    shell_command.push(c);
                    continue;
                }
                match indent {
                    0 => {
                        target_name.push(c);
                    },
                    4 => {
                        inshell = true;
                        shell_command.push(c)
                    },
                    _ => {
                        if comment_start(c) {
                            incomment = true;
                        } else if new_line(c) {
                            incomment = false;
                        } else {
                            return Err(Error::RecipeParsingError(format!("unhandled symbol: {:?} at {}:{}:{}", c, lineno, lpos, pos)))
                            // continue;
                        }
                    }
                }
            }
        }
    }
    if !incomment {
    if dependency {
        if !current_dependency.is_empty() {
            instruction.add_dependency(&current_dependency);
            current_dependency.clear();
        }
    } else if inshell {
        if !shell_command.is_empty() {
            instruction.add_action(&shell_command);
            shell_command.clear();
        }
    }
    }
    recipe.add_instruction(instruction);
    Ok(recipe)
}



#[cfg(test)]
mod unit_tests {
    use crate::pars::parse_recipe;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};
    use crate::errors::{Error};


    #[test]
    fn test_target_name() -> Result<(), Error> {
        let input = "foo:";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &[], &[])));
        Ok(())
    }

    #[test]
    fn test_target_and_command() -> Result<(), Error>  {
        let input = "foo:
    bar";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));

        Ok(())
    }
    #[test]
    fn test_target_and_2_commands()  -> Result<(), Error> {
        let input = "foo:
    bar
    baz
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar", "baz"], &[])));
        Ok(())
    }

    #[test]
    fn test_target_and_2_dependencies()  -> Result<(), Error> {
        let input = "foo: bar baz";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &[], &["bar", "baz"])));
        Ok(())
    }


}
#[cfg(test)]
mod comment_tests {
    use crate::pars::parse_recipe;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};
    use crate::errors::{Error};

    #[test]
    fn test_comment_ruble_noneffective_at_shell_command_level() -> Result<(), Error>  {
        let input = "
foo:
    bar
    ₽echo dobrie
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
        Ok(())
    }

    #[test]
    fn test_comment_ruble_noneffective_at_target_level()  -> Result<(), Error> {
        let input0 = "₽echo dobrie

foo:
    bar
";
        let recipe = parse_recipe(&input0)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
        let input1 = "

₽echo dobrie
foo:
    bar
";
        let recipe = parse_recipe(&input1)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
        Ok(())
    }

    #[test]
    fn test_target_and_comment_symbol_newsheqel()  -> Result<(), Error> {
        let input = "₪ noop
foo:
    bar
    ₪ noop
    baz

₪ noop
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar", "baz"], &[])));
        Ok(())
    }


    #[test]
    fn test_target_and_comment_symbol_naira()  -> Result<(), Error> {
        let input = "₦éééééé
foo:
    bar
    ₦ééééééééééé
    baz

₦ééééééééééé
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar", "baz"], &[])));
        Ok(())
    }

   #[test]
    fn test_target_and_comment_symbol_euro_currencies()  -> Result<(), Error> {
        let input = "₠€
foo:
    bar
    €₠€₠€₠
    baz

€₠₠€€₠₠€
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar", "baz"], &[])));
        Ok(())
    }

   #[test]
    fn test_target_failed_currency_merely_allowed_for_comment()  -> Result<(), Error> {
        let input = "₢AB

₢ 0 0
purpose:
    ₢ I'm Mr. Meeseeks look at me!
    seek-attention
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("purpose", &["seek-attention"], &[])));
        Ok(())
    }

   #[test]
    fn test_target_pesos()  -> Result<(), Error> {
        let input = "₢₱

brush-teeth:
    rinse
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("brush-teeth", &[ "rinse"], &[])));
        Ok(())
    }

//    #[test]
//     fn test_comment_is_not_dependency()  -> Result<(), Error> {
//         let input = "₢₱
//
// sweet-tooth: ₱aste!
// ";
//         let recipe = parse_recipe(&input)?;
//
//         assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("sweet-tooth", &[], &[])));
//         Ok(())
//     }
//
//     #[test]
//     fn test_spaces_and_comments()  -> Result<(), Error> {
//         let input = "₢ ₱
//
// brush-teeth: disinfect
//     ₱aste, remember the paste!
//     brush
//     rinse
//     spit
// ";
//         let recipe = parse_recipe(&input)?;
//
//         assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("brush-teeth", &[ "brush", "rinse", "spit"], &["disinfect"])));
//         Ok(())
//     }
}


#[cfg(test)]
mod functional_tests {
    use std::fs;
    use crate::pars::parse_recipe;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};
    use crate::errors::{Error};

    #[test]
    fn test_parse_repo_bakefile()  -> Result<(), Error> {
        let unparsed_file = fs::read_to_string("Bakefile").unwrap();
        let recipe = parse_recipe(&unparsed_file)?;

        assert_equal!(recipe, Recipe::with_instructions(vec![
            Instruction::with_dependencies("all", &[
                "cargo test",
            ], &[]),
        ]));
        Ok(())
    }
    // #[test]
    // fn test_parse_test_bakefile_0c1t2s()  -> Result<(), Error> {
    //     let unparsed_file = fs::read_to_string("tests/simple/Bakefile.0c1t2s").unwrap();
    //     let recipe = parse_recipe(&unparsed_file)?;

    //     assert_equal!(recipe, Recipe::with_instructions(vec![
    //         Instruction::with_dependencies("all", &[], &["test"]),
    //         Instruction::with_dependencies("test", &[
    //             "cargo test",
    //         ], &[]),
    //     ]));
    //     Ok(())
    // }
}
