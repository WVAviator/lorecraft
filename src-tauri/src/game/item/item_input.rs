use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemInput {
    game_summary: String,
    items: Vec<String>,
}

impl ItemInput {
    pub fn new(game_summary: &str, items: Vec<String>) -> Self {
        ItemInput {
            game_summary: game_summary.to_string(),
            items: items.clone(),
        }
    }

    pub fn to_string(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn item_input_example1_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/item_detail/example1_input.json")
            .build();

        serde_json::from_str::<ItemInput>(&example1).unwrap();
    }

    #[test]
    fn item_input_example2_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/item_detail/example2_input.json")
            .build();

        serde_json::from_str::<ItemInput>(&example2).unwrap();
    }
}
