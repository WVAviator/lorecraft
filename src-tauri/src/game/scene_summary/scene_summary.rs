use serde::{Deserialize, Serialize};

use crate::{
    game::{
        chat_completion_factory::{ChatCompletionFactory, ChatCompletionFactoryArgs},
        summary::Summary,
    },
    prompt_builder::PromptBuilder,
};

use super::scene_summary_input::SceneSummaryInput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneSummary {
    pub scenes: Vec<SummarizedScene>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummarizedScene {
    name: String,
    description: String,
    actions: String,
}

impl SceneSummary {
    pub async fn create(
        summary: &Summary,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Self, anyhow::Error> {
        let input = SceneSummaryInput::new(
            summary.summary.to_string(),
            summary.win_condition.to_string(),
        );

        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/main.txt")
            .add_example_input("./prompts/scene_summary/example1_input.json")
            .add_example_output("./prompts/scene_summary/example1_output.json")
            .add_example_input("./prompts/scene_summary/example2_input.json")
            .add_example_output("./prompts/scene_summary/example2_output.json")
            .build();

        let user_message = input.to_string();

        factory
            .try_create(
                ChatCompletionFactoryArgs::builder()
                    .name("Scene Summary")
                    .system_message(system_message)
                    .user_message(user_message)
                    .file_name("tmp/scene_summary.json")
                    .build(),
            )
            .await
    }
}
