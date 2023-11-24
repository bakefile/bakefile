use pest::Parser;
use pest_derive::Parser;
use crate::ing::{Instruction, Recipe};

#[derive(Parser)]
#[grammar = "bake.pest"]
pub struct BakeParser;

#[derive(Debug)]
pub enum Error {
    RuleParsingError(pest::error::Error<Rule>),
    IOError(std::io::Error),
}
impl std::error::Error for Error {}

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Error::RuleParsingError(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RuleParsingError(e) => match e {
                pest::error::Error { variant, .. } => {
                    write!(f, "RuleParsingError: {}", variant)
                }
            },
            Error::IOError(e) => write!(f, "IOError: {}", e),
        }
    }
}

pub fn parse_targets(data: &str) -> Recipe {
    let pair = BakeParser::parse(Rule::bakefile, data).unwrap().next().unwrap();
    // println!("\x1b[1;38;5;154m{:#?}\x1b[0m", pair);

    let mut recipe = Recipe::blank();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::target => {
                let mut current_instruction = Instruction::of_dependencies("", &[]);
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::target_name => {
                            if current_instruction.name().len() == 0 {
                                current_instruction.set_label(pair.as_str().trim());
                            }
                        }
                        Rule::deps => {
                            for pair in pair.into_inner() {
                                match pair.as_rule() {
                                    Rule::target_name => {
                                        for dep in pair.as_str().split(" ").map(|x| x.trim()).filter(|x| x.len() > 0) {
                                            current_instruction.add_dependency(dep);
                                        }
                                    }
                                    _ => {
                                        println!(
                                            "{:?}\x1b[1;38;5;154m{:?}\x1b[0m",
                                            pair.as_rule(),
                                            pair.as_str().trim().split(" ").collect::<Vec<&str>>()
                                        );

                                    }
                                }
                            }
                        }
                        Rule::subshell => {
                            // println!("\x1b[1;38;5;202m{}\x1b[0m", pair.as_str());
                            let tagliatti = pair
                                .as_str()
                                .split("\n")
                                .map(|mirt| mirt.trim())
                                .collect::<Vec<&str>>();

                            current_instruction.set_action(&tagliatti);
                            recipe.add_instruction(current_instruction.clone());
                            current_instruction = Instruction::of_dependencies("", &[]);
                        }

                        _ => {
                            println!(
                                "{:?}\x1b[1;38;5;154m{:?}\x1b[0m",
                                pair.as_rule(),
                                pair.as_str().trim().split(" ").collect::<Vec<&str>>()
                            );

                        }
                    }
                }
            }
            fback => {
                if fback != Rule::EOI {
                    println!(
                        "{:?}\x1b[1;38;5;154m{:?}\x1b[0m",
                        pair.as_rule(),
                        pair.as_str().trim().split(" ").collect::<Vec<&str>>()
                    );
                }
            }
        }
    }
    recipe
}
#[cfg(test)]
mod functional_tests {
    use crate::parse_targets;
    use std::fs;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};


    #[test]
    fn test_simple_0_comments_1_target_2_subshells() {
        let unparsed_file = fs::read_to_string("tests/simple/Bakefile.0c1t2s").unwrap();
        let recipe = parse_targets(&unparsed_file);

        let mut lecipe = Recipe::blank();
        lecipe.add_instruction(Instruction::with_dependencies("all", &["echo \"hello world\"", "echo \"hallö welt\" > /dev/random"], &[]));
        assert_equal!(recipe, lecipe);

    }

    #[test]
    fn test_simple_0_comments_3_target_3_subshells() {
        let unparsed_file = fs::read_to_string("tests/simple/Bakefile.0c3t3s").unwrap();
        let recipe = parse_targets(&unparsed_file);
        let mut lecipe = Recipe::blank();

        lecipe.add_instruction(Instruction::with_dependencies("hw", &[], &["en", "de"]));
        lecipe.add_instruction(Instruction::with_dependencies("en", &["echo \"hello world\""], &[]));
        lecipe.add_instruction(Instruction::with_dependencies("de", &["echo \"hallö welt\" > /dev/random"], &[]));
        assert_equal!(recipe, lecipe);
    }
}
