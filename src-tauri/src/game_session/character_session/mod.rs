use anyhow::{anyhow, bail, Context};
use log::error;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    commands::character_prompt::character_prompt_request::CharacterPromptRequest,
    file_manager::FileManager,
    game_state::GameState,
    openai_client::{
        assistant::assistant_create_request::AssistantCreateRequest,
        assistant_tool::function::Function,
        chat_completion::chat_completion_model::ChatCompletionModel,
        create_message::create_message_request::CreateMessageRequest,
        create_run::create_run_request::CreateRunRequest,
        retrieve_run::retrieve_run_response::ToolCall,
        submit_tool_outputs::submit_tool_outputs_request::SubmitToolOutputsRequest, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

use self::{
    character_give_item::CharacterGiveItem, character_profile::CharacterProfile,
    character_save_data::CharacterSaveData, character_trade_items::CharacterTradeItems,
};

mod character_context;
mod character_give_item;
mod character_profile;
mod character_save_data;
mod character_trade_items;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterSession {
    character_id: String,
    assistant_id: String,
    thread_id: String,
    character_save_data: CharacterSaveData,
    last_run_id: Option<String>,
    last_tool_call: Option<ToolCall>,
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

        // TODO: Run should be triggered immediately

        Ok(CharacterSession {
            character_id: character_id.to_string(),
            assistant_id: character_assistant_id,
            thread_id,
            character_save_data,
            last_run_id: None,
            last_tool_call: None,
        })
    }

    pub async fn process_prompt(
        &mut self,
        request: &CharacterPromptRequest,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        game_state: &mut GameState,
    ) -> Result<(), anyhow::Error> {
        // Add message to thread
        match &self.last_run_id {
            Some(run_id) => {
                let tool_call = self.last_tool_call.as_ref().ok_or(anyhow!(
                    "Missing last tool call, cannot return run function outputs."
                ))?;

                let output = match &request.trade_accept {
                    Some(true) => {
                        // TODO: Set trade response accepted in game state
                        json!({ "response": "accept" }).to_string()
                    }
                    Some(false) => {
                        // TODO: Set trade response declined in game state
                        json!({ "response": "reject" }).to_string()
                    }
                    None => bail!("Expected trade response."),
                };

                let mut submit_tool_outputs_request = SubmitToolOutputsRequest::new();
                submit_tool_outputs_request.add_output(&tool_call.id, &output);

                let submit_tool_outputs_response = openai_client
                    .submit_tool_outputs(submit_tool_outputs_request, &self.thread_id, &run_id)
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "Unable to submit tool outputs for character session: {:?}",
                            e
                        )
                    })?;
            }
            None => {
                let message = request
                    .message
                    .as_ref()
                    .ok_or(anyhow!("Expected message from player."))?;
                let create_message_response = openai_client
                    .create_message(CreateMessageRequest::new(&message), &self.thread_id)
                    .await
                    .map_err(|e| {
                        anyhow!("Unable to create message in character session: {:?}", e)
                    })?;

                // TODO: Add message in game state

                let run_response = openai_client
                    .create_run(
                        CreateRunRequest::builder()
                            .assistant_id(&self.assistant_id)
                            .build(),
                        &self.thread_id,
                    )
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "Unable to create run in thread of character session: {:?}",
                            e
                        )
                    })?;

                self.last_run_id = Some(run_response.id);
            }
        }

        loop {
            if let Ok(retrieve_run_response) = openai_client
                .retrieve_run(
                    &self.thread_id,
                    &self
                        .last_run_id
                        .as_ref()
                        .ok_or(anyhow!("Missing run id for run retrieval."))?,
                )
                .await
            {
                match retrieve_run_response.status.as_str() {
                    "requires_action" => {
                        let tool_calls = retrieve_run_response
                            .required_action
                            .ok_or(anyhow!(
                                "No required actions despite requires_action run status."
                            ))?
                            .submit_tool_outputs
                            .tool_calls;
                        if tool_calls.len() > 1 {
                            bail!("Assistant tried to trigger two functions.");
                        }

                        let tool_call = tool_calls
                            .into_iter()
                            .next()
                            .ok_or(anyhow!("No tool calls available despite action required."))?;

                        self.last_tool_call = Some(tool_call.clone());

                        match tool_call.function.name.as_str() {
                            "trade_items" => {
                                let character_trade_items = serde_json::from_str::<
                                    CharacterTradeItems,
                                >(
                                    &tool_call.function.arguments
                                )
                                .context("Unable to parse arguments from trade function call.")?;
                                game_state
                                    .character_interaction
                                    .as_mut()
                                    .ok_or(anyhow!("Character interaction not set on game state."))?
                                    .propose_trade(
                                        &character_trade_items.your_item,
                                        &character_trade_items.player_item,
                                    );
                            }
                            "give_item" => {
                                let character_give_item =
                                    serde_json::from_str::<CharacterGiveItem>(
                                        &tool_call.function.arguments,
                                    )
                                    .context(
                                        "Unable to parse arguments from gift function call.",
                                    )?;
                                game_state
                                    .character_interaction
                                    .as_mut()
                                    .ok_or(anyhow!("Character interaction not set on game state."))?
                                    .propose_gift(&character_give_item.item);
                            }

                            _ => bail!(
                                "Invalid function call received: {}",
                                tool_call.function.name.as_str()
                            ),
                        }

                        // Exit early, anticipating a later return response with trade request reply
                        return Ok(());
                    }
                    "cancelling" | "cancelled" | "failed" | "expired" => {
                        bail!("Assistant run failed.")
                    }
                    "completed" => break,
                    _ => bail!("Unknown status received for run retrieval."),
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        }

        self.last_run_id = None;
        self.last_tool_call = None;

        // TODO: Read latest message, apply to game state

        Ok(())
    }

    fn process_meta_commands(message: &str) -> Vec<String> {
        vec![]
    }
}
