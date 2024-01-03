use anyhow::anyhow;
use log::error;

use serde::{Deserialize, Serialize};


use crate::{
    file_manager::FileManager,
    openai_client::{
        assistant_api::assistant_create_request::AssistantCreateRequest,
        assistant_tool::function::Function,
        chat_completion::chat_completion_model::ChatCompletionModel, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

use self::{character_profile::CharacterProfile, character_save_data::CharacterSaveData};

mod character_context;
mod character_profile;
mod character_save_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSession {
    character_id: String,
    assistant_id: String,
    thread_id: String,
    character_save_data: CharacterSaveData,
}

impl CharacterSession {
    pub async fn new(
        character_id: &str,
        game_id: &str,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        let profile = CharacterProfile::load(character_id, game_id, file_manager)?;
        let profile = serde_json::to_string(&profile)?;

        let instructions = PromptBuilder::new()
            .add_prompt("./prompts/character_actor/main.txt")
            .add_plain_text(&profile)
            .build();

        let functions = vec![
            Function::from_file("./prompts/character_actor/give_function.json")?,
            Function::from_file("./prompts/character_actor/trade_function.json")?,
        ];

        let assistant_response = openai_client
            .create_assistant(AssistantCreateRequest::new(
                instructions,
                game_id.to_string(),
                ChatCompletionModel::Gpt3_5Turbo1106,
                functions,
            ))
            .await
            .expect("Failed to create assistant.");

        let character_assistant_id = assistant_response.id;

        let thread_response = openai_client.create_thread().await.map_err(|e| {
            error!("Failed to create thread for character assistant:\n{:?}", e);
            anyhow!("Failed to start thread.")
        })?;

        let thread_id = thread_response.id;

        let character_save_data = CharacterSaveData::load(character_id, game_id, file_manager)?;

        Ok(CharacterSession {
            character_id: character_id.to_string(),
            assistant_id: character_assistant_id,
            thread_id,
            character_save_data,
        })
    }
}
