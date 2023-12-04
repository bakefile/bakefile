use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use crate::errors::Error;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    label: String,
    actions: Vec<String>,
    deps: Vec<String>,
}

impl Instruction {
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string()
    }
    pub fn name(&self) -> String {
        self.label.clone()
    }
    pub fn set_action(&mut self, actions: &[&str]) {
        self.actions.extend(actions.iter().map(|a| a.to_string() ).collect::<Vec<String>>());
    }
    pub fn add_action(&mut self, action: &str) {
        self.actions.push(action.to_string());
    }
    pub fn command(&self) -> String {
        self.actions.join("\n")
    }
    pub fn steps(&self) -> Vec<String> {
        self.actions.clone()
    }
    pub fn with_dependencies(name: &str, actions: &[&str], dependencies: &[&str]) -> Instruction {
        Instruction {
            label: name.to_string(),
            actions: actions.iter().map(|a| a.to_string() ).collect::<Vec<String>>(),
            deps: dependencies.iter().map(|d| d.to_string() ).collect::<Vec<String>>()
        }
    }
    pub fn of_dependencies(name: &str, dependencies: &[&str]) -> Instruction {
        Instruction {
            label: name.to_string(),
            actions: Vec::new(),
            deps: dependencies.iter().map(|d| d.to_string() ).collect::<Vec<String>>()
        }
    }

    pub fn new(name: &str) -> Instruction {
        Instruction {
            label: name.to_string(),
            actions: Vec::new(),
            deps: Vec::new()
        }
    }
    pub fn with_action(name: &str, action: &str) -> Instruction {
        Instruction {
            label: name.to_string(),
            actions: vec![action.to_string()],
            deps: Vec::new()
        }
    }

    pub fn dependencies(&self) -> Vec<String> {
        self.deps.clone()
    }

    pub fn add_dependency(&mut self, dependency_name: &str) {
        self.deps.push(dependency_name.to_string());
    }
}


#[cfg(test)]
mod instruction_tests {
    use crate::ing::Instruction;

    #[test]
    fn test_attributes() {
        let prepare = Instruction::with_action("nap", "sleep 2");
        assert_eq!(&prepare.name(), "nap");
        assert_eq!(&prepare.command(), "sleep 2");
    }

    #[test]
    fn test_dependencies() {
        let mut bake_with_frosting = Instruction::with_dependencies(
            "produce-cake",
            &["apply-frosting"],
            &vec!["acquire-ingredients", "bake-cake"]
        );
        assert_eq!(&bake_with_frosting.name(), "produce-cake");
        assert_eq!(&bake_with_frosting.command(), "apply-frosting");
        assert_eq!(bake_with_frosting.dependencies(), vec![
            "acquire-ingredients".to_string(),
            "bake-cake".to_string(),
        ]);

        bake_with_frosting.add_dependency("wait-til-cooldown");

        assert_eq!(bake_with_frosting.dependencies(), vec![
            "acquire-ingredients".to_string(),
            "bake-cake".to_string(),
            "wait-til-cooldown".to_string(),
        ]);
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    path: Option<String>,
    inst: BTreeMap<String, Vec<Instruction>>,
    keys: BTreeSet<String>,
    ings: BTreeMap<String, String>,
    requ: Vec<String>,
}
impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.path {
            Some(path) => {
                write!(f, "Recipe({})", path)
            },
            None => {
                write!(f, "Recipe[{}]", self.keys.iter().map(|k|k.to_string()).collect::<Vec<String>>().join(", "))
            }
        }
    }
}
impl Recipe {
    pub fn blank() -> Recipe {
        Recipe {
            path: None,
            inst: BTreeMap::new(),
            keys: BTreeSet::new(),
            ings: BTreeMap::new(),
            requ: Vec::new(),
        }
    }
    pub fn resolve_dependencies(&self, instruction_label: &str) -> Option<Vec<String>> {
        let mut instruction_labels = Vec::<String>::new();
        match self.inst.get(instruction_label) {
            None => return None,
            Some(instructions) => {
                for inst in instructions {
                    let deps = inst.dependencies();
                    if deps.len() > 0 {
                        for dep in deps {
                            match self.resolve_dependencies(&dep) {
                                Some(labels) =>{
                                    instruction_labels.extend(labels);
                                },
                                None => {}
                            }
                        }
                    }
                    instruction_labels.push(inst.name());
                }
            }
        };
        Some(instruction_labels)
    }
    pub fn with_path(&mut self, path: &str) -> Recipe {
        self.path = Some(path.to_string());
        self.clone()
    }
    pub fn with_instruction(instruction: Instruction) -> Recipe {
        let mut recipe = Self::blank();
        recipe.add_instruction(instruction);
        recipe
    }
    pub fn with_instructions(instructions: Vec<Instruction>) -> Recipe {
        let mut recipe = Self::blank();
        for instruction in instructions {
            recipe.add_instruction(instruction);
        }
        recipe
    }
    pub fn instructions(&self) -> BTreeMap<String, Vec<Instruction>> {
        self.inst.clone()
    }
    pub fn main_instruction(&self) -> Result<Instruction, Error> {
        match self.keys.first() {
            None => Err(Error::UnstructedRecipe(format!("{:?} appears to be empty of instructions", self))),
            Some(key) => {
                match self.inst.get(key) {
                    Some(instructions) => if instructions.len() > 0 {
                        Ok(instructions[0].clone())
                    } else {
                        Err(Error::UnstructedRecipe(format!("{:?} appears to be empty of instructions", self)))
                    },
                    None => Err(Error::UnstructedRecipe(format!("{:?} inconsistent state: key {:?} not present in internal table", self, key)))
                }
            }
        }
    }
    pub fn get_instructions(&self, name: &str) -> Vec<Instruction> {
        match self.inst.get(name) {
            Some(instructions) => instructions.clone(),
            None => vec![self.main_instruction().unwrap()],
        }
    }
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.keys.insert(instruction.name());
        match self.inst.get_mut(&instruction.name()) {
            Some(instructions) => {
                instructions.push(instruction);
            }
            None => {
                self.inst
                    .insert(
                        instruction.name(),
                        vec![instruction]
                    );
            }
        }
    }
    pub fn add_ingredient(&mut self, name: &str, substance: &str) {
        match self.ings.get_mut(name) {
            Some(ingredient) => {
                ingredient.push_str(substance)
            }
            None => {
                self.ings
                    .insert(
                        format!("{}", name),
                        format!("{}", substance),
                    );
            }
        }
    }
    pub fn translate_instruction(&mut self, instruction: &Instruction) -> Vec<String> {
        let mut steps = Vec::<String>::new();
        for step in instruction.steps() {
            let mut step = step.clone();
            for (k, v) in self.ings.iter() {
                let pat = format!("%[{}]", k);
                step = step.replace(&pat, v);
            }
            steps.push(step);
        }
        steps
    }
}


#[cfg(test)]
mod recipe_tests {
    use std::collections::BTreeMap;
    use crate::ing::{Recipe, Instruction};
    use crate::Error;

    #[test]
    fn test_attributes() {
        let inst1 = Instruction::with_action("fb", ":() { :|: };:");
        let mut inss = BTreeMap::new();
        inss.insert("fb".to_string(), vec![inst1.clone()]);
        let mut recipe = Recipe::blank();
        recipe.add_instruction(inst1);
        assert_eq!(recipe.instructions(), inss);
    }

    #[test]
    fn test_main_instruction() -> Result<(), Error>{
        let inst1 = Instruction::with_action("fb", ":() { :|: };:");
        let mut recipe = Recipe::blank();
        recipe.add_instruction(inst1.clone());
        assert_eq!(recipe.main_instruction()?, inst1);
        Ok(())
    }

    #[test]
    fn test_ingredients() -> Result<(), Error>{
        let mut recipe = Recipe::blank();
        let inst1 = Instruction::with_action("show-ingredient", "echo %[ING1]");
        recipe.add_ingredient("ING1", "sauce");
        assert_eq!(recipe.translate_instruction(&inst1), vec!["echo sauce".to_string()]);
        Ok(())
    }

    #[test]
    fn test_dependency_resolution_two_dimensional() -> Result<(), Error>{
        let mut recipe = Recipe::with_instruction(Instruction::with_action("bar", "echo 'and the bunnymen'"));
        recipe.add_instruction(Instruction::with_dependencies("foo", &[ "echo 'and the bunnymen'"], &["bar"]));
        assert_eq!(recipe.resolve_dependencies("foo"), Some(vec!["bar".to_string(), "foo".to_string()]));
        Ok(())
    }

    #[test]
    fn test_dependency_resolution_three_dimensional() -> Result<(), Error>{
        let mut recipe = Recipe::with_instruction(Instruction::with_action("gamel", "echo 'G'"));
        recipe.add_instruction(Instruction::with_dependencies("aleph", &[ "echo 'A'"], &["bet"]));
        recipe.add_instruction(Instruction::with_dependencies("bet", &[ "echo 'B'"], &["gamel"]));
        assert_eq!(recipe.resolve_dependencies("aleph"), Some(vec!["gamel".to_string(), "bet".to_string(), "aleph".to_string()]));
        Ok(())
    }
}
