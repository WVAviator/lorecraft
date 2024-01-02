use serde::{Deserialize, Serialize};

use crate::{
    openai_client::{
        chat_completion::chat_completion_model::ChatCompletionModel,
        chat_completion::chat_completion_request::ChatCompletionRequest,
        openai_client_error::OpenAIClientError, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

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
    pub async fn generate(
        client: &OpenAIClient,
        user_prompt: &str,
    ) -> Result<Self, OpenAIClientError> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/summary/main.txt")
            .add_plain_text("Example Input: make a game about mystical forests and ancient ruins")
            .add_example_output("./prompts/summary/example1.json")
            .add_plain_text("Example Input: choose any unique game idea")
            .add_example_output("./prompts/summary/example2.json")
            .build();

        let user_prompt = String::from(user_prompt);

        let response_text = client
            .chat_completion_request(ChatCompletionRequest::new(
                system_prompt,
                user_prompt,
                ChatCompletionModel::Gpt3_5Turbo1106,
            ))
            .await
            .expect("Failed to get response from OpenAI API.")
            .get_content();

        let summary = serde_json::from_str::<Summary>(response_text.as_str())
            .expect("Failed to deserialize summary.");

        Ok(summary)
    }
}
