use serde::{Deserialize, Serialize};

use crate::{
    openai_client::{
        chat_completion_model::ChatCompletionModel, chat_completion_request::ChatCompletionRequest,
        openai_client_error::OpenAIClientError, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

use self::page::Page;

mod page;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Narrative {
    pages: Vec<Page>,
}

impl Narrative {
    pub async fn generate(client: &OpenAIClient, summary: &str) -> Result<Self, OpenAIClientError> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/narrative/main.txt")
            .add_example_input("./prompts/narrative/example1_input.json")
            .add_example_output("./prompts/narrative/example1_output.json")
            .add_example_input("./prompts/narrative/example2_input.json")
            .add_example_output("./prompts/narrative/example2_output.json")
            .build();

        let user_prompt = summary.to_string();

        let response_text = client
            .chat_completion_request(ChatCompletionRequest::new(
                system_prompt,
                user_prompt,
                ChatCompletionModel::Gpt_35_Turbo_1106,
            ))
            .await
            .expect("Failed to get response from OpenAI API.")
            .get_content();

        let narrative = serde_json::from_str::<Narrative>(response_text.as_str())
            .expect("Failed to deserialize narrative.");

        Ok(narrative)
    }
}
