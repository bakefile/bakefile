use clap::Parser;
use bakefile::{parse_recipe_from_path, Baker};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short = 'f', long, help = "path to a Bakefile", default_value = "Bakefile")]
    pub bakefile: String,

    #[arg(short, long, help = "current working directory")]
    pub cwd: Option<String>,

    #[arg(short, long, help = "toggle safe output")]
    pub safe: bool,

    #[arg(help = "specify instructions to follow")]
    pub instructions: Vec<String>,
}


fn main() {
    let params = Cli::parse();
    let recipe = match parse_recipe_from_path(&params.bakefile) {
        Ok(recipe) => recipe,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    Baker::new(params.cwd, params.safe, params.instructions).perform(recipe);
}
