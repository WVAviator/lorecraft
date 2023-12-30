use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterInput {
    pub game_summary: String,
    pub scene_description: String,
    pub character_description: String,
}

impl CharacterInput {
    pub fn new(game_summary: &str, scene_description: &str, character_description: &str) -> Self {
        CharacterInput {
            game_summary: game_summary.to_string(),
            scene_description: scene_description.to_string(),
            character_description: character_description.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn character_input_example1_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/character_detail/example1_input.json")
            .build();

        serde_json::from_str::<CharacterInput>(&example1).unwrap();
    }

    #[test]
    fn character_input_example2_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/character_detail/example2_input.json")
            .build();

        serde_json::from_str::<CharacterInput>(&example2).unwrap();
    }
}
