use serde::{Deserialize, Serialize};

use crate::game::scene_summary::SummarizedScene;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneInput {
    pub game_summary: String,
    pub scene_summary: SummarizedScene,
}

impl SceneInput {
    pub fn new(game_summary: &str, scene_summary: &SummarizedScene) -> Self {
        Self {
            game_summary: game_summary.to_string(),
            scene_summary: scene_summary.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn scene_detail_input_example1_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/scene_detail/example1_input.json")
            .build();

        serde_json::from_str::<SceneInput>(&example1).unwrap();
    }

    #[test]
    fn scene_detail_input_example2_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/scene_detail/example2_input.json")
            .build();

        serde_json::from_str::<SceneInput>(&example2).unwrap();
    }
}
