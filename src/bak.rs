pub use crate::ing::{Instruction, Recipe};
pub use crate::execute::{Bash, Sh};
pub use crate::execute::Shell;
pub use std::process::{Command, Output};
use sanitation::SString;
use std::io::{self, Write};

pub struct Baker {
    cwd: Option<String>,
    safe: bool,
    instructions: Vec<String>,
}

impl Baker {
    pub fn new(cwd: Option<String>, safe: bool, instructions: Vec<String>) -> Baker {
        Baker{
            cwd: cwd,
            safe: safe,
            instructions: instructions,
        }
    }
    pub fn perform(&self, recipe: Recipe) {
        // let mut performed = Vec::<String>::new();
        if self.instructions.len() == 0 {
            self.execute_instruction(&recipe.main_instruction().unwrap())
        } else {
            for label in &self.instructions {
                for instruction in recipe.get_instructions(label) {
                    self.execute_instruction(&instruction)
                }
            }
        }
    }
    pub fn execute_instruction(&self, instruction: &Instruction) {
        for step in instruction.steps() {
            let output = Sh::new(self.cwd.clone()).execute(&step).expect(&format!("failed execute step: {:?}", step));
            if self.safe {
                let stdout = SString::new(&output.stdout);
                let stderr = SString::new(&output.stderr);
                println!("{}", stdout.soft_word());
                eprintln!("{}", stderr.soft_word());
            } else {
                io::stdout().write_all(&output.stdout).expect(&format!("failed to write the output of {:?} to stdout", &step));
                io::stderr().write_all(&output.stderr).expect(&format!("failed to write the output of {:?} to stderr", &step));
            }
            match output.status.code() {
                Some(code) => {
                    if code != 0 {
                        std::process::exit(code);
                    }
                },
                None => {
                    std::process::exit(3);
                }
            }
        }
    }
}
