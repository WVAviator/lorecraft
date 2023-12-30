use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemOutput {
    pub items: Vec<ItemDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDetail {
    pub name: String,
    pub description: String,
    pub image: String,
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn item_output_example1_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/item_detail/example1_output.json")
            .build();

        serde_json::from_str::<ItemOutput>(&example1).unwrap();
    }

    #[test]
    fn item_output_example2_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/item_detail/example2_output.json")
            .build();

        serde_json::from_str::<ItemOutput>(&example2).unwrap();
    }
}
