use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;


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
    pub fn command(&self) -> String {
        self.actions.join("\n")
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

    pub fn new(name: &str, action: &str) -> Instruction {
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
        let prepare = Instruction::new("nap", "sleep 2");
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
    inst: BTreeMap<String, Vec<Instruction>>,
    requ: Vec<String>,
}
impl Recipe {
    pub fn blank() -> Recipe {
        Recipe {
            inst: BTreeMap::new(),
            requ: Vec::new(),
        }
    }
    pub fn instructions(&self) -> BTreeMap<String, Vec<Instruction>> {
        self.inst.clone()
    }
    pub fn add_instruction(&mut self, instruction: Instruction) {
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
}


#[cfg(test)]
mod recipe_tests {
    use std::collections::BTreeMap;
    use crate::ing::{Recipe, Instruction};

    #[test]
    fn test_attributes() {
        let inst1 = Instruction::new("fb", ":() { :|: };:");
        let mut inss = BTreeMap::new();
        inss.insert("fb".to_string(), vec![inst1.clone()]);
        let mut recipe = Recipe::blank();
        recipe.add_instruction(inst1);
        assert_eq!(recipe.instructions(), inss);
    }
}