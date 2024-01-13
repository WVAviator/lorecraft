use anyhow::anyhow;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};
use serde::{Deserialize, Serialize};

use crate::prompt_builder::PromptBuilder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Summary {
    pub name: String,
    pub description: String,
    pub art_style: String,
    pub art_theme: String,
    pub cover_art: String,
    pub summary: String,
    pub win_condition: String,
}

impl Summary {
    pub async fn generate(client: &OpenAIClient, user_prompt: &str) -> Result<Self, anyhow::Error> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/summary/main.txt")
            .add_plain_text("Example Input: make a game about mystical forests and ancient ruins")
            .add_example_output("./prompts/summary/example1.json")
            .add_plain_text("Example Input: choose any unique game idea")
            .add_example_output("./prompts/summary/example2.json")
            .build();

        let user_prompt = String::from(user_prompt);

        let response_text = client
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

        let summary = serde_json::from_str::<Summary>(response_text.as_str())
            .map_err(|e| anyhow!("Failed to deserialize summary: {}", e))?;

        Ok(summary)
    }
}
