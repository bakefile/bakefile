use shells::sh;
pub use crate::ing::Recipe;
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
        let instruction = recipe.main_instruction().expect(&format!("Baker cannot perform recipe {}", recipe));
        for step in instruction.steps() {
            let (code, stdout, stderr) = sh!("{}", step);
            println!("{}", stdout);
            eprintln!("{}", stderr);
            if code != 0 {
                std::process::exit(code);
            } else {
                std::process::exit(3);
            }
        }
    }
}
