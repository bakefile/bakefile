pub use crate::ing::Recipe;
pub use crate::execute::{Bash, Sh};
pub use crate::execute::Shell;
use sanitation::SString;
pub use std::process::{Command, Output};
use std::io::{self, Write};

pub struct Baker {
    safe: bool,
}

impl Baker {
    pub fn new(safe: bool) -> Baker {
        Baker{safe:safe}
    }
    pub fn perform(&self, recipe: Recipe) {
        let instruction = recipe.main_instruction().expect(&format!("Baker cannot perform recipe {}", recipe));
        for step in instruction.steps() {
            let output = Sh::new(None).execute(&step).expect(&format!("failed execute step: {:?}", step));
            if self.safe {
                let stdout = SString::new(&output.stdout);
                let stderr = SString::new(&output.stderr);
                println!("{}", stdout.soft_word());
                eprintln!("{}", stderr.soft_word());
            } else {
                io::stdout().write_all(&output.stdout).expect(&format!("failed to write to stdout the output of {:?}", &step));
                io::stderr().write_all(&output.stderr).expect(&format!("failed to write to stdout the output of {:?}", &step));
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
