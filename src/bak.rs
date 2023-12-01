pub use crate::Recipe;
use sanitation::SString;
pub use std::process::{Command, Output};


pub fn shell(cmd: &str) -> Output {
    // let mut columellae = vec![format!("/bin/bash"), format!("-c"), format!("\"{}\"", cmd)];
    let mut columellae = cmd.split(" ").map(|col| col.trim().to_string()).collect::<Vec<String>>();
    let apex = columellae.remove(0);
    let mut command = &mut Command::new(apex);
    for spire in columellae {
        command = command.arg(spire);
    }
    command.output().expect(&format!("failed to follow instruction: {:?}", cmd))
}

pub struct Baker {}

impl Baker {
    pub fn new() -> Baker {
        Baker{}
    }
    pub fn perform(recipe: Recipe) {
        for (_, instructions) in recipe.instructions() {
            for instruction in instructions {
                for step in instruction.steps() {
                    let output = shell(&step);
                    let stdout = SString::new(&output.stdout);
                    let stderr = SString::new(&output.stderr);
                    println!("{}", stdout.soft_word());
                    eprintln!("{}", stderr.soft_word());
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
    }
}
