use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneSummaryInput {
    summary: String,
    win_condition: String,
}

impl SceneSummaryInput {
    pub fn new(summary: String, win_condition: String) -> Self {
        Self {
            summary,
            win_condition,
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
    fn matches_example1_input() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/example1_input.json")
            .build();
        serde_json::from_str::<SceneSummaryInput>(&example1).unwrap();
    }

    #[test]
    fn matches_example2_input() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/example2_input.json")
            .build();
        serde_json::from_str::<SceneSummaryInput>(&example2).unwrap();
    }
}
