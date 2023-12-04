use bakefile::{parse_recipe_from_path, Baker};


fn main() {
    let recipe = match parse_recipe_from_path("Bakefile") {
        Ok(recipe) => recipe,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    Baker::new(false).perform(recipe);
}
