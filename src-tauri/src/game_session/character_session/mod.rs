use anyhow::{anyhow, bail, Context};
use log::{error, info, trace};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::join;

use crate::{
    commands::character_prompt::character_prompt_request::CharacterPromptRequest,
    file_manager::FileManager,
    game::{character::Character, Game},
    game_state::GameState,
    openai_client::{
        assistant_create::assistant_create_request::AssistantCreateRequest,
        assistant_tool::{function::Function, AssistantTool},
        chat_completion::chat_completion_model::ChatCompletionModel,
        create_message::create_message_request::CreateMessageRequest,
        create_run::create_run_request::CreateRunRequest,
        list_messages::list_messages_query::ListMessagesQuery,
        retrieve_run::retrieve_run_response::ToolCall,
        submit_tool_outputs::submit_tool_outputs_request::SubmitToolOutputsRequest,
        OpenAIClient,
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
    character: Character,
    assistant_id: String,
    thread_id: String,
    character_save_data: CharacterSaveData,
    last_run_id: Option<String>,
    last_tool_call: Option<ToolCall>,
}

impl CharacterSession {
    pub async fn new(
        character_name: &str,
        game_id: &str,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        game_state: &mut GameState,
    ) -> Result<Self, anyhow::Error> {
        let game =
            Game::load(&game_id, file_manager).context("Unable to load game from game ID")?;
        let character = game
            .characters
            .iter()
            .find(|c| c.name == character_name)
            .ok_or(anyhow!("Unable to find character with provided name."))?;

        let profile = CharacterProfile::from_character(&character)?;
        let profile = serde_json::to_string(&profile)?;

        let character_save_data = CharacterSaveData::load(&character.id, &game.id, file_manager)
            .context("Unable to load previous character data.")?;

        let additional_context = json!({
            "previous_conversations": &character_save_data.previous_conversations,
            "character_inventory": &character_save_data.character_inventory,
        })
        .to_string();

        let instructions = PromptBuilder::new()
            .add_prompt("./prompts/character_actor/main.txt")
            .add_plain_text(&profile)
            .add_plain_text(&additional_context)
            .build();

        let tools = vec![
            AssistantTool::new_function(Function::from_file(
                "./prompts/character_actor/give_function.json",
            )?),
            AssistantTool::new_function(Function::from_file(
                "./prompts/character_actor/trade_function.json",
            )?),
        ];

        let assistant_response = openai_client
            .create_assistant(AssistantCreateRequest::new(
                instructions,
                game_id.to_string(),
                ChatCompletionModel::Gpt3_5Turbo1106,
                tools,
            ))
            .await
            .expect("Failed to create assistant.");

        let character_assistant_id = assistant_response.id;

        let thread_response = openai_client.create_thread().await.map_err(|e| {
            error!("Failed to create thread for character assistant:\n{:?}", e);
            anyhow!("Failed to start thread.")
        })?;

        let thread_id = thread_response.id;

        // TODO: Run should be triggered immediately

        let mut character_session = CharacterSession {
            character_id: character.id.to_string(),
            character: character.clone(),
            assistant_id: character_assistant_id,
            thread_id,
            character_save_data,
            last_run_id: None,
            last_tool_call: None,
        };

        character_session.trigger_new_run(openai_client).await?;
        character_session
            .poll_last_run(openai_client, game_state)
            .await?;

        Ok(character_session)
    }

    pub async fn process_prompt(
        &mut self,
        request: &CharacterPromptRequest,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        game_state: &mut GameState,
    ) -> Result<(), anyhow::Error> {
        match &self.last_run_id {
            Some(run_id) => {
                let tool_call = self.last_tool_call.as_ref().ok_or(anyhow!(
                    "Missing last tool call, cannot return run function outputs."
                ))?;

                let output = match &request.trade_accept {
                    Some(true) => {
                        let trade = game_state
                            .character_interaction
                            .as_mut()
                            .ok_or(anyhow!("Missing character interaction."))?
                            .trade
                            .take()
                            .ok_or(anyhow!("No active trade to process in state."))?;
                        if let Some(item) = trade.from_player {
                            game_state.inventory.retain(|i| i.ne(&item));
                        }
                        // TODO: Also decrement the scene items, which will need to be saved
                        // somewhere
                        json!({ "player_response": "accept", "updated_player_inventory": &game_state.inventory }).to_string()
                    }
                    Some(false) => {
                        game_state
                            .character_interaction
                            .as_mut()
                            .ok_or(anyhow!("Missing character interaction."))?
                            .trade = None;
                        json!({ "player_response": "reject" }).to_string()
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
                trace!(
                    "Received tool outputs response:\n{:?}",
                    submit_tool_outputs_response
                );
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

                trace!(
                    "Received create message response:\n{:?}",
                    create_message_response
                );

                let message = format!("Player: {}", &create_message_response.content[0].text.value);
                game_state
                    .character_interaction
                    .as_mut()
                    .ok_or(anyhow!(
                        "Could not find character interaction to append message."
                    ))?
                    .add_message(&message);

                self.trigger_new_run(openai_client).await?;
            }
        }

        if let PollResponse::RequiresAction = self.poll_last_run(openai_client, game_state).await? {
            return Ok(());
        }

        self.last_run_id = None;
        self.last_tool_call = None;

        let message_list_response = openai_client
            .list_messages(
                ListMessagesQuery::builder(&self.thread_id)
                    .limit(1)
                    .order("desc")
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Unable to retrieve latest message from thread:\n{:?}", e))?;

        trace!(
            "Received latest message from thread:\n{:?}",
            &message_list_response
        );

        let character_message = message_list_response.data[0].content[0].text.value.clone();
        let processed_messages = self.process_meta_commands(character_message);

        info!("Received and processed character messages. Appending new messages to game state.");
        for message in processed_messages {
            if message.starts_with(format!("{}:", &self.character.name).as_str()) {
                game_state
                    .character_interaction
                    .as_mut()
                    .ok_or(anyhow!("No character interaction to append message to."))?
                    .add_message(&message);
            } else {
                game_state
                    .character_interaction
                    .as_mut()
                    .ok_or(anyhow!(
                        "No character interaction to append nonverbal action to."
                    ))?
                    .add_nonverbal(&message);
            }
        }

        Ok(())
    }

    async fn trigger_new_run(&mut self, openai_client: &OpenAIClient) -> Result<(), anyhow::Error> {
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
        trace!("Received run response:\n{:?}", run_response);
        self.last_run_id = Some(run_response.id);
        Ok(())
    }

    async fn poll_last_run(
        &mut self,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<PollResponse, anyhow::Error> {
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
                        info!("Run requested function response.");

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

                                info!(
                                    "Character requested to trade {} for the player's {}",
                                    &character_trade_items.your_item,
                                    &character_trade_items.player_item
                                );

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

                                info!(
                                    "Character requested to give {} to the player.",
                                    &character_give_item.item
                                );

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

                        info!("Ending interaction early, awaiting response from player to submit function response.");
                        return Ok(PollResponse::RequiresAction);
                    }
                    "cancelling" | "cancelled" | "failed" | "expired" => {
                        error!("The run has expired or has failed.");
                        bail!("Assistant run failed.")
                    }
                    "completed" => break,
                    _ => bail!("Unknown status received for run retrieval."),
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
            trace!("Polling for run response.");
        }

        Ok(PollResponse::Complete)
    }

    fn process_meta_commands(&mut self, message: String) -> Vec<String> {
        let mut messages = Vec::new();

        let message = {
            // Sometimes the LLM returns the character name prefix, sometimes it does not
            if message.starts_with(format!("{}:", &self.character.name).as_str()) {
                message.split(":").skip(1).collect::<String>()
            } else {
                message
            }
        };

        let mut current_message = Vec::new();
        for word in message.split(" ") {
            if word.starts_with("$emotion") {
                messages.push(format!(
                    "{}: {}",
                    &self.character.name,
                    current_message.join(" ")
                ));
                current_message.clear();
                let emotion: String = word
                    .chars()
                    .skip(9)
                    .take(word.chars().count() - 10)
                    .collect();
                messages.push(format!("{} is feeling {}", &self.character.name, emotion));
            } else if word.starts_with("$action") {
                messages.push(format!(
                    "{}: {}",
                    &self.character.name,
                    current_message.join(" ")
                ));
                current_message.clear();
                let action: String = word
                    .chars()
                    .skip(8)
                    .take(word.chars().count() - 9)
                    .collect();
                messages.push(format!("{} {}", &self.character.name, action));
            } else {
                current_message.push(word);
            }
        }

        if !current_message.is_empty() {
            messages.push(format!(
                "{}: {}",
                &self.character.name,
                current_message.join(" ")
            ));
        }

        messages
    }

    pub async fn end_session(
        &mut self,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        game_state: &mut GameState,
    ) -> Result<String, anyhow::Error> {
        let request = CharacterPromptRequest {
            message: Some(String::from("$summarize")),
            trade_accept: None,
            end_conversation: None,
        };
        self.process_prompt(&request, openai_client, file_manager, game_state)
            .await?;

        let summary = game_state
            .character_interaction
            .as_ref()
            .ok_or(anyhow!("Missing charater interaction."))?
            .messages
            .last()
            .ok_or(anyhow!("Missing last message, cannot get summary."))?
            .text
            .clone();

        self.character_save_data
            .previous_conversations
            .push(summary.clone());

        self.character_save_data
            .save(file_manager)
            .context("Unable to save updated character data.")?;

        game_state.end_character_interaction();

        let assistant_delete = openai_client.delete_assistant(&self.assistant_id);
        let thread_delete = openai_client.delete_thread(&self.thread_id);

        let (assistant_delete, thread_delete) = join!(assistant_delete, thread_delete);

        assistant_delete.map_err(|e| anyhow!("Unable to delete character assistant: {:?}", e))?;
        thread_delete.map_err(|e| anyhow!("Unable to delete character thread: {:?}", e))?;

        Ok(summary)
    }
}

enum PollResponse {
    RequiresAction,
    Complete,
}
