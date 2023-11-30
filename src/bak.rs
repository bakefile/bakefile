pub use crate::Recipe;
use sanitation::SString;
use std::collections::HashMap;
pub use std::process::{Command, Output};
// use std::env::current_dir;

// pub trait Shell<'a>: Display {
pub trait Shell<'a> {
    fn new() -> Self where Self: Sized;
    fn command(&self) -> String {
        "/usr/bin/env".to_string()
    }
    fn get_path(&self) -> String;
    fn exec_params(&self) -> Vec<String> {
        vec!["-c".to_string()]
    }
    fn spawn(&self, shell_command: &str) -> Result<Output, std::io::Error> {
        let (mut cmd, args) = self.engage(shell_command);
        let env : HashMap<String, String> = std::env::vars().collect();
        Ok(cmd.args(args).envs(&env).output()?)
    }
    fn engage(&self, shell_command: &str) -> (Command, Vec<String>) {
        let shell_executable_path = self.get_path();
        let mut args: Vec<String> = if shell_executable_path.len() > 0 {
            vec![shell_executable_path]
        } else {
            Vec::new()
        };
        args.extend(self.exec_params());
        args.push(format!("'{}'", shell_command));

        (Command::new(&self.command()), args)
    }
}

#[derive(Debug, Clone)]
pub struct Baker {
    // cwd: String,
}

impl Shell<'_> for Baker {
    fn new() -> Baker {
        Baker {
            // cwd: format!("{}", current_dir().expect("could not retrieve the current working directory of the current process").display())
        }
    }
    fn get_path(&self) -> String {
        format!("bash")
    }
}

pub fn shell<S: for <'a> Shell<'a>>(cmd: &str) -> Output {
    let shell = S::new();
    shell.spawn(cmd).expect(&format!("could not execute shell command {:?}", cmd))
}

impl Baker {
    pub fn perform(recipe: Recipe) {
        for (_, instructions) in recipe.instructions() {
            for instruction in instructions {
                for step in instruction.steps() {
                    let output = shell::<Baker>(&step);
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
