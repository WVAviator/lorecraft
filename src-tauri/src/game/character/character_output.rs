use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterOutput {
    pub name: String,
    pub physical_description: String,
    pub personality: String,
    pub backstory: String,
    pub thoughts: String,
    pub inventory: Vec<String>,
    pub image: String,
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn character_output_example1_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/character_detail/example1_output.json")
            .build();

        serde_json::from_str::<CharacterOutput>(&example1).unwrap();
    }

    #[test]
    fn character_output_example2_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/character_detail/example2_output.json")
            .build();

        serde_json::from_str::<CharacterOutput>(&example2).unwrap();
    }
}
