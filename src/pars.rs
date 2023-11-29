use crate::ing::{Instruction, Recipe};

fn comment_start(c: char) -> bool {
    return c == '₽'
}
fn comment_end(c: char) -> bool {
    return c == '\n'
}

#[allow(unused)]
pub fn parse_recipe(data: &str) -> Recipe {
    let mut recipe = Recipe::blank();
    let mut instruction = Instruction::new("");
    let mut target_name = String::new();
    let mut shell_command = String::new();
    let mut indent = 0;
    let mut lineno = 0;
    let mut lpos = 0;
    let mut pos = 0;
    let mut incomment = false;

    for c in data.chars() {
        pos += 1;
        lpos += 1;

        if comment_start(c) { // rubble signs currently turns on comment thus invalidating (almost) any other change in state
            incomment = true;
        } else if comment_end(c) {
            incomment = false;
        }
        if incomment {
            continue;
        }

        match c {
            ':' => {
                match indent {
                    0 => {
                        instruction.set_label(&target_name);
                        target_name.clear();
                    },
                    _ => {
                        shell_command.push(c)
                    },
                }
                continue;
            },
            ' ' => {
                indent += 1;
                continue;
            },
            '\n' => {
                lineno += 1;
                lpos = 0;
                indent = 0;
                incomment = false;

                // eprintln!("\x1b[1;35;8;208m{:?}\x1b[0m", shell_command);
                if !shell_command.is_empty() {
                    instruction.add_action(&shell_command);
                    shell_command.clear();
                }
            },
            _ => {
                match indent {
                    0 => {
                        target_name.push(c);
                    },
                    4 => {
                        shell_command.push(c)
                    },
                    _ => todo!()
                }
            }
        }
    }
    if !shell_command.is_empty() {
        instruction.add_action(&shell_command);
    }
    recipe.add_instruction(instruction);
    recipe
}



#[cfg(test)]
mod unit_tests {
    use crate::pars::parse_recipe;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};


    #[test]
    fn test_target_name() {
        let input = "foo:";
        let recipe = parse_recipe(&input);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &[], &[])));
    }

    #[test]
    fn test_target_and_command() {
        let input = "foo:
    bar";
        let recipe = parse_recipe(&input);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
    }

    #[test]
    fn test_comment_rubble_noneffective_at_shell_command_level() {
        let input = "
foo:
    bar
    ₽echo dobrie
";
        let recipe = parse_recipe(&input);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
    }

    #[test]
    fn test_comment_rubble_noneffective_at_target_level() {
        let input0 = "₽echo dobrie

foo:
    bar
";
        let recipe = parse_recipe(&input0);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
        let input1 = "

₽echo dobrie
foo:
    bar
";
        let recipe = parse_recipe(&input1);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));
    }

    #[test]
    fn test_target_and_2_commands() {
        let input = "foo:
    bar
    baz
";
        let recipe = parse_recipe(&input);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar", "baz"], &[])));
    }


}
