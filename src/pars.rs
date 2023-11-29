use crate::ing::{Instruction, Recipe};


pub fn parse_recipe(data: &str) -> Recipe {
    let mut recipe = Recipe::blank();
    recipe.add_instruction(Instruction::with_dependencies("foo", &["bar"], &[]));
    recipe
}


#[cfg(test)]
mod unit_tests {
    use crate::pars::parse_recipe;
    use k9::assert_equal;
    use crate::ing::{Instruction, Recipe};


    #[test]
    fn test_ok() {
        let input = "foo:
    bar
";
        let recipe = parse_recipe(&input);

        assert_equal!(recipe, Recipe::with_instruction(Instruction::with_dependencies("foo", &["bar"], &[])));

    }
}
