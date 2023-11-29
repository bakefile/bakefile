use crate::ing::{Instruction, Recipe};

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

    for c in data.chars() {
        pos += 1;
        lpos += 1;
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
                // eprintln!("\x1b[1;35;8;208m{:?}\x1b[0m", shell_command);
                if !shell_command.is_empty() {
                    instruction.add_action(&shell_command);
                }
            },
            '$' |  '€' |  '₢' |  '₽' |  '₰' |  '₤' |  '¢' |  '#' |  '₳' |  '₷' |  '₸' |  '₪' |  '﷼' |  '௹' |  '૱' |  '৳' |  '₦' |  '₴' |  '₭' |  '₱' |  '₮' |  '₺' |  '₩' |  '฿' |  '₶' |  '₯' |  '₧' |  '₣' |  '₠' |  '₥' |  '¥' => {
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


// [
//     36,
//     8368,
//     8364,
//     8354,
//     8381,
//     8356,
//     162,
//     35,
//     8371,
//     8375,
//     8376,
//     8362,
//     65020,
//     3065,
//     2801,
//     2547,
//     8358,
//     8372,
//     8365,
//     8369,
//     8366,
//     8378,
//     8361,
//     3647,
//     8374,
//     8367,
//     8359,
//     8355,
//     8352,
//     8357,
//     165,
// ]


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

}
