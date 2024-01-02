pub mod scene_summary_input;
pub mod summarized_scene;

use serde::{Deserialize, Serialize};

use crate::{
    openai_client::{
        chat_completion::chat_completion_model::ChatCompletionModel,
        chat_completion::chat_completion_request::ChatCompletionRequest,
        openai_client_error::OpenAIClientError, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

use self::summarized_scene::SummarizedScene;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneSummary {
    pub scenes: Vec<SummarizedScene>,
}

impl SceneSummary {
    pub async fn generate(
        summary: &str,
        win_condition: &str,
        openai_client: &OpenAIClient,
    ) -> Result<Self, OpenAIClientError> {
        let input = scene_summary_input::SceneSummaryInput::new(
            summary.to_string(),
            win_condition.to_string(),
        );

        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/main.txt")
            .add_example_input("./prompts/scene_summary/example1_input.json")
            .add_example_output("./prompts/scene_summary/example1_output.json")
            .add_example_input("./prompts/scene_summary/example2_input.json")
            .add_example_output("./prompts/scene_summary/example2_output.json")
            .build();

        let user_prompt = input.to_string();

        let response_text = openai_client
            .chat_completion_request(ChatCompletionRequest::new(
                system_prompt,
                user_prompt,
                ChatCompletionModel::Gpt3_5Turbo1106,
            ))
            .await
            .expect("Failed to get response from OpenAI API.")
            .get_content();

        let scene_summary = serde_json::from_str::<SceneSummary>(response_text.as_str())
            .expect("Failed to deserialize scene summary.");

        Ok(scene_summary)
    }
}

#[cfg(test)]
mod test {
    use crate::prompt_builder::PromptBuilder;

    use super::*;

    #[test]
    fn scene_summary_output_matches_example1_output() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/example1_output.json")
            .build();
        serde_json::from_str::<SceneSummary>(&example1).unwrap();
    }

    #[test]
    fn scene_summary_output_matches_example2_output() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/example2_output.json")
            .build();
        serde_json::from_str::<SceneSummary>(&example2).unwrap();
    }
}
