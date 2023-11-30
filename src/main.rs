use std::fs;
use bakefile::{parse_recipe, Error};


fn main() -> Result<(), Error>{
    let unparsed_file = fs::read_to_string("Bakefile").unwrap();
    let recipe = parse_recipe(&unparsed_file)?;

    println!("{:#?}", recipe);
    Ok(())
}
