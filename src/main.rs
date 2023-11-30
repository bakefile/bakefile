use std::fs;
use bakefile::{parse_recipe, Baker};


fn main() {
    let unparsed_file = fs::read_to_string("Bakefile").unwrap();
    let recipe = match parse_recipe(&unparsed_file) {
        Ok(recipe) => recipe,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    Baker::perform(recipe);
}
