use std::fs;
use pest::Parser;
use bakefile::Rule;
use bakefile::{parse_targets, Error, BakeParser};


fn main() -> Result<(), Error>{
    let unparsed_file = fs::read_to_string("Bakefile").unwrap();
    let pair = BakeParser::parse(Rule::bakefile, &unparsed_file).unwrap();

    println!("{:#?}", pair);
    Ok(())
}
