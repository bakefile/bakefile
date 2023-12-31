use std::fs;
use crate::ing::{Instruction, Recipe};
use crate::errors::Error;

fn comment_start(c: char) -> bool {
    match c  {
        '#' => true,
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
    let indentation = 6;
    let ac = data.chars();
    for c in ac {
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
                    instruction.add_action(&shell_command.trim());
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
            '\t' => {
                continue
            },
            ':' => {
                match indent {
                    0 => {
                        instruction.set_label(&target_name);
                        target_name.clear();
                        dependency = true;
                    },
                    i => if i == indentation {
                        shell_command.push(c)
                    } else {
                        return Err(Error::RecipeParsingError(format!("invalid character {:?} at {}:{}:{}", c, lineno, lpos, pos)))
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
                } else if !incomment {
                    shell_command.push(c);
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
                    f => {
                        if comment_start(c) {
                            incomment = true;
                        } else if new_line(c) {
                            incomment = false;
                        } else if f == indentation {
                            inshell = true;
                            shell_command.push(c);
                        } else {
                            if indent != indentation {
                                return Err(Error::RecipeParsingError(format!("got {} spaces instead of 6 at {}:{}:{}", indent, lineno, lpos, pos)))
                            } else {
                                return Err(Error::RecipeParsingError(format!("unhandled symbol: {:?} at line {}:{}:{}", c, lineno, lpos, pos)))
                            }
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
            instruction.add_action(&shell_command.trim());
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
    fn test_comment_seems_to_use_hash_symbol_for_some_apparent_reason() -> Result<(), Error>  {
        let input = "# comment
foo:
      bar
      # comment
";
        let recipe = parse_recipe(&input)?;

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
        Ok(())
    }
}


#[cfg(test)]
mod functional_tests {
    use crate::pars::parse_recipe_from_path;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};
    use crate::errors::{Error};

    #[test]
    fn test_parse_repo_bakefile()  -> Result<(), Error> {
        let recipe = parse_recipe_from_path("Bakefile")?;

        assert_equal!(recipe, Recipe::with_instructions(vec![
            Instruction::with_dependencies("all", &[
                "cargo test",
            ], &[]),
        ]));
        Ok(())
    }
    #[test]
    fn test_parse_test_bakefile_0c1t2s()  -> Result<(), Error> {
        let recipe = parse_recipe_from_path("tests/simple/Bakefile.0c1t2s")?;

        assert_equal!(recipe, Recipe::with_instructions(vec![
            Instruction::with_dependencies("all", &[
                "echo \"hello world\"",
                "echo \"hallö welt\" > /dev/random",
            ], &[]),
        ]));
        Ok(())
    }
    // #[test]
    // fn test_parse_test_bakefile_0c3t3s()  -> Result<(), Error> {
    //     let recipe = parse_recipe_from_path("tests/simple/Bakefile.0c3t3s")?;

    //     assert_equal!(recipe, Recipe::with_instructions(vec![

    //         Instruction::with_dependencies("hw", &[
    //         ], &["en", "de"]),

    //         Instruction::with_dependencies("en", &[
    //             "echo \"hello world\"",
    //         ], &[]),

    //         Instruction::with_dependencies("de", &[
    //             "echo \"hallö welt\" > /dev/random",
    //         ], &[]),

    //     ]));
    //     Ok(())
    // }
    // #[test]
    // fn test_parse_test_bakefile_3c3t3s()  -> Result<(), Error> {
    //     let recipe = parse_recipe_from_path("tests/simple/Bakefile.3c3t3s")?;

    //     assert_equal!(recipe, Recipe::with_instructions(vec![
    //         Instruction::with_dependencies("hw", &[
    //         ], &["en", "de"]),
    //         Instruction::with_dependencies("en", &[
    //             "echo \"hello world\"",
    //         ], &[]),
    //         Instruction::with_dependencies("de", &[
    //             "echo \"hallö welt\" > /dev/random",
    //         ], &[]),
    //     ]));
    //     Ok(())
    // }
}
