pub use std::process::{Command, Output};


pub trait Shell<'a> {
    fn new(cwd: Option<String>) -> Self where Self: Sized;
    fn command(&self) -> String {
        self.get_path()
    }
    fn get_path(&self) -> String;
    fn exec_params(&self) -> Vec<String> {
        vec!["-c".to_string()]
    }
    fn execute(&self, shell_command: &str) -> Result<Output, std::io::Error> {
        let mut args = Vec::new();
        args.extend(self.exec_params());
        args.push(format!("'{}'", shell_command));
        let mut cmd = Command::new(&self.command());
        Ok(cmd.args(args).output()?)
    }
}

#[derive(Debug, Clone)]
pub struct Bash {
    cwd: Option<String>
}

impl Shell<'_> for Bash {
    fn new(cwd: Option<String>) -> Bash {
        Bash {
            cwd: cwd
        }
    }
    fn get_path(&self) -> String {
        format!("bash")
    }

}

#[derive(Debug, Clone)]
pub struct Sh {
    cwd: Option<String>
}

impl Shell<'_> for Sh {
    fn new(cwd: Option<String>) -> Sh {
        Sh {
            cwd: cwd
        }
    }
    fn get_path(&self) -> String {
        format!("sh")
    }

}
