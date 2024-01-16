use serde::{Deserialize, Serialize};

use crate::game::image::Image;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NarrativeOutput {
    pub pages: Vec<OutputPage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputPage {
    pub narrative: String,
    pub image: String,
    pub image_object: Option<Image>,
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn narrative_output_example1_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/narrative/example1_output.json")
            .build();
        serde_json::from_str::<NarrativeOutput>(&example1).unwrap();
    }

    #[test]
    fn narrative_output_example2_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/narrative/example2_output.json")
            .build();
        serde_json::from_str::<NarrativeOutput>(&example2).unwrap();
    }
}
