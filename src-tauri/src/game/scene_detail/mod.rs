use anyhow::anyhow;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};
use serde::{Deserialize, Serialize};

use crate::prompt_builder::PromptBuilder;

use self::scene_detail_input::SceneDetailInput;

use super::scene_summary::summarized_scene::SummarizedScene;

mod scene_detail_input;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneDetail {
    pub name: String,
    pub narrative: String,
    pub metadata: String,
    pub image: String,
    pub characters: Vec<String>,
    pub items: Vec<String>,
}

impl SceneDetail {
    pub async fn generate(
        summary: &str,
        summarized_scene: &SummarizedScene,
        openai_client: &OpenAIClient,
    ) -> Result<Self, anyhow::Error> {
        let scene_detail_input = SceneDetailInput::new(summary, summarized_scene);
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/scene_detail/main.txt")
            .add_example_input("./prompts/scene_detail/example1_input.json")
            .add_example_output("./prompts/scene_detail/example1_output.json")
            .add_example_input("./prompts/scene_detail/example2_input.json")
            .add_example_output("./prompts/scene_detail/example2_output.json")
            .build();
        let user_prompt = serde_json::to_string(&scene_detail_input).unwrap();

        let response_text = openai_client
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(system_prompt)
                    .add_user_message(user_prompt)
                    .model(ChatModel::Gpt_35_Turbo_1106)
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion request: {}", e))?
            .get_content();

        let scene_detail = serde_json::from_str::<SceneDetail>(&response_text)
            .expect("Failed to deserialize scene detail.");

        Ok(scene_detail)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scene_detail_example1_output_matches() {
        let example1 = PromptBuilder::new()
            .add_prompt("./prompts/scene_detail/example1_output.json")
            .build();
        serde_json::from_str::<SceneDetail>(&example1).unwrap();
    }

    #[test]
    fn scene_detail_example2_output_matches() {
        let example2 = PromptBuilder::new()
            .add_prompt("./prompts/scene_detail/example2_output.json")
            .build();
        serde_json::from_str::<SceneDetail>(&example2).unwrap();
    }
}
