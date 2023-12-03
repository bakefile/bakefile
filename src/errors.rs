use crate::knead::Rule;

#[derive(Debug)]
pub enum Error {
    RuleParsingError(pest::error::Error<Rule>),
    RecipeParsingError(String),
    UnstructedRecipe(String),
    IOError(std::io::Error),
}
impl std::error::Error for Error {}

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Error::RuleParsingError(e)
    }
}
impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::RecipeParsingError(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RuleParsingError(e) => match e {
                pest::error::Error { variant, .. } => {
                    write!(f, "RuleParsingError: {}", variant)
                }
            },
            Error::RecipeParsingError(e) => write!(f, "RecipeParsingError: {}", e),
            Error::UnstructedRecipe(e) => write!(f, "UnstructedRecipe: {}", e),
            Error::IOError(e) => write!(f, "IOError: {}", e),
        }
    }
}
