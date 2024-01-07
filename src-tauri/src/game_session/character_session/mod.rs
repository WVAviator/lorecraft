use anyhow::{anyhow, bail, Context};
use log::{debug, error, info, trace};

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
pub mod character_profile;
mod character_save_data;
mod character_trade_items;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct CharacterSession {
//     pub character_id: String,
//     character: Character,
//     assistant_id: String,
//     thread_id: String,
//     character_save_data: CharacterSaveData,
//     last_run_id: Option<String>,
//     last_tool_call: Option<ToolCall>,
// }

// impl CharacterSession {
//     pub async fn new(
//         character_name: &str,
//         game_id: &str,
//         openai_client: &OpenAIClient,
//         file_manager: &FileManager,
//         game_state: &mut GameState,
//     ) -> Result<Self, anyhow::Error> {
//         let game =
//             Game::load(&game_id, file_manager).context("Unable to load game from game ID")?;
//         let character = game
//             .characters
//             .iter()
//             .find(|c| c.name == character_name)
//             .ok_or(anyhow!("Unable to find character with provided name."))?;

//         info!(
//             "Found character information for {}. Generating profile and context for assistant.",
//             &character_name
//         );

//         let profile = CharacterProfile::from_character(&character)?;
//         let profile = serde_json::to_string(&profile)?;

//         let character_save_data = CharacterSaveData::load(&character.id, &game.id, file_manager)
//             .context("Unable to load previous character data.")?;

//         let additional_context = json!({
//             "previous_conversations": &character_save_data.previous_conversations,
//             "character_inventory": &character_save_data.character_inventory,
//         })
//         .to_string();

//         let instructions = PromptBuilder::new()
//             .add_prompt("./prompts/character_actor/main.txt")
//             .add_plain_text(&profile)
//             .add_plain_text(&additional_context)
//             .build();

//         info!("Loaded instructions for character assistant.");
//         trace!("{}", &instructions);

//         let tools = vec![
//             AssistantTool::new_function(Function::from_file(
//                 "./prompts/character_actor/give_function.json",
//             )?),
//             AssistantTool::new_function(Function::from_file(
//                 "./prompts/character_actor/trade_function.json",
//             )?),
//         ];

//         info!("Creating character assistant.");

//         let assistant_response = openai_client
//             .create_assistant(AssistantCreateRequest::new(
//                 instructions,
//                 game_id.to_string(),
//                 ChatCompletionModel::Gpt3_5Turbo1106,
//                 tools,
//             ))
//             .await
//             .expect("Failed to create assistant.");

//         let character_assistant_id = assistant_response.id;

//         info!(
//             "Created assistant with id {} for character.",
//             &character_assistant_id
//         );

//         let thread_response = openai_client.create_thread().await.map_err(|e| {
//             error!("Failed to create thread for character assistant:\n{:?}", e);
//             anyhow!("Failed to start thread.")
//         })?;

//         let thread_id = thread_response.id;

//         info!(
//             "Created thread for character conversation with id {}.",
//             &thread_id
//         );

//         let mut character_session = CharacterSession {
//             character_id: character.id.to_string(),
//             character: character.clone(),
//             assistant_id: character_assistant_id,
//             thread_id,
//             character_save_data,
//             last_run_id: None,
//             last_tool_call: None,
//         };

//         info!("Triggering a run for initial conversation starter.");

//         character_session.trigger_new_run(openai_client).await?;

//         info!("Run created, polling for response.");

//         character_session
//             .poll_last_run(openai_client, game_state)
//             .await?;

//         game_state.character_interact(&character.id);
//         character_session
//             .get_and_apply_response(openai_client, game_state)
//             .await?;
//         info!("Initial response generated and applied to game state.");

//         Ok(character_session)
//     }

//     pub async fn process_prompt(
//         &mut self,
//         request: &CharacterPromptRequest,
//         openai_client: &OpenAIClient,
//         file_manager: &FileManager,
//         game_state: &mut GameState,
//     ) -> Result<(), anyhow::Error> {
//         info!("Processing player interaction with character.");

//         match &self.last_run_id {
//             Some(run_id) => {
//                 info!("Previous run id exists - processing as a trade response.");

//                 let tool_call = self.last_tool_call.as_ref().ok_or(anyhow!(
//                     "Missing last tool call, cannot return run function outputs."
//                 ))?;

//                 let output = match &request.trade_accept {
//                     Some(true) => {
//                         info!("Processing trade acceptance.");

//                         let trade = game_state
//                             .character_interaction
//                             .as_mut()
//                             .ok_or(anyhow!("Missing character interaction."))?
//                             .trade
//                             .take()
//                             .ok_or(anyhow!("No active trade to process in state."))?;
//                         if let Some(item) = trade.from_player {
//                             game_state.inventory.retain(|i| i.ne(&item));
//                         }
//                         // TODO: Also decrement the scene items, which will need to be saved
//                         // somewhere
//                         json!({ "player_response": "accept", "updated_player_inventory": &game_state.inventory }).to_string()
//                     }
//                     Some(false) => {
//                         info!("Processing declined trade request.");

//                         game_state
//                             .character_interaction
//                             .as_mut()
//                             .ok_or(anyhow!("Missing character interaction."))?
//                             .trade = None;

//                         json!({ "player_response": "reject" }).to_string()
//                     }
//                     None => {
//                         error!("A previous run id exists, but the provided response did not include trade accept or decline.");
//                         bail!("Expected trade response.");
//                     }
//                 };

//                 info!("Submitting trade function tool outputs response.");

//                 let mut submit_tool_outputs_request = SubmitToolOutputsRequest::new();
//                 submit_tool_outputs_request.add_output(&tool_call.id, &output);

//                 let submit_tool_outputs_response = openai_client
//                     .submit_tool_outputs(submit_tool_outputs_request, &self.thread_id, &run_id)
//                     .await
//                     .map_err(|e| {
//                         anyhow!(
//                             "Unable to submit tool outputs for character session: {:?}",
//                             e
//                         )
//                     })?;
//                 trace!(
//                     "Received tool outputs response:\n{:?}",
//                     submit_tool_outputs_response
//                 );
//             }
//             None => {
//                 info!("No previous run_id exists, creating new message and appending to thread.");

//                 let message = request
//                     .message
//                     .as_ref()
//                     .ok_or(anyhow!("Expected message from player."))?;
//                 let create_message_response = openai_client
//                     .create_message(CreateMessageRequest::new(&message), &self.thread_id)
//                     .await
//                     .map_err(|e| {
//                         anyhow!("Unable to create message in character session: {:?}", e)
//                     })?;

//                 trace!(
//                     "Received create message response:\n{:?}",
//                     create_message_response
//                 );

//                 info!("Updating game state with player message.");

//                 let message = format!("Player: {}", &create_message_response.content[0].text.value);
//                 game_state
//                     .character_interaction
//                     .as_mut()
//                     .ok_or(anyhow!(
//                         "Could not find character interaction to append message."
//                     ))?
//                     .add_message(&message);

//                 self.trigger_new_run(openai_client).await?;
//             }
//         }

//         if let PollResponse::RequiresAction = self.poll_last_run(openai_client, game_state).await? {
//             return Ok(());
//         }

//         self.last_run_id = None;
//         self.last_tool_call = None;

//         self.get_and_apply_response(openai_client, game_state)
//             .await?;

//         Ok(())
//     }

//     async fn get_and_apply_response(
//         &mut self,
//         openai_client: &OpenAIClient,
//         game_state: &mut GameState,
//     ) -> Result<(), anyhow::Error> {
//         info!("Fetching latest message in thread.");
//         let message_list_response = openai_client
//             .list_messages(
//                 ListMessagesQuery::builder(&self.thread_id)
//                     .limit(1)
//                     .order("desc")
//                     .build(),
//             )
//             .await
//             .map_err(|e| anyhow!("Unable to retrieve latest message from thread:\n{:?}", e))?;
//         trace!(
//             "Received latest message from thread:\n{:?}",
//             &message_list_response
//         );
//         info!("Processing message meta-commands");
//         let character_message = message_list_response.data[0].content[0].text.value.clone();
//         let processed_messages = self.process_meta_commands(character_message);
//         info!("Appending new messages to game state.");
//         Ok(for message in processed_messages {
//             if message.starts_with(format!("{}:", &self.character.name).as_str()) {
//                 game_state
//                     .character_interaction
//                     .as_mut()
//                     .ok_or(anyhow!("No character interaction to append message to."))?
//                     .add_message(&message);
//             } else {
//                 game_state
//                     .character_interaction
//                     .as_mut()
//                     .ok_or(anyhow!(
//                         "No character interaction to append nonverbal action to."
//                     ))?
//                     .add_nonverbal(&message);
//             }
//         })
//     }

//     async fn trigger_new_run(&mut self, openai_client: &OpenAIClient) -> Result<(), anyhow::Error> {
//         info!("Initiating new run on thread {}", &self.thread_id);

//         let run_response = openai_client
//             .create_run(
//                 CreateRunRequest::builder()
//                     .assistant_id(&self.assistant_id)
//                     .build(),
//                 &self.thread_id,
//             )
//             .await
//             .map_err(|e| {
//                 anyhow!(
//                     "Unable to create run in thread of character session: {:?}",
//                     e
//                 )
//             })?;
//         trace!("Received run response:\n{:?}", run_response);
//         self.last_run_id = Some(run_response.id);
//         Ok(())
//     }

//     async fn poll_last_run(
//         &mut self,
//         openai_client: &OpenAIClient,
//         game_state: &mut GameState,
//     ) -> Result<PollResponse, anyhow::Error> {
//         loop {
//             info!("Polling for character run response.");
//             if let Ok(retrieve_run_response) = openai_client
//                 .retrieve_run(
//                     &self.thread_id,
//                     &self
//                         .last_run_id
//                         .as_ref()
//                         .ok_or(anyhow!("Missing run id for run retrieval."))?,
//                 )
//                 .await
//             {
//                 match retrieve_run_response.status.as_str() {
//                     "requires_action" => {
//                         info!("Run requested function response.");

//                         let tool_calls = retrieve_run_response
//                             .required_action
//                             .ok_or(anyhow!(
//                                 "No required actions despite requires_action run status."
//                             ))?
//                             .submit_tool_outputs
//                             .tool_calls;
//                         if tool_calls.len() > 1 {
//                             bail!("Assistant tried to trigger two functions.");
//                         }

//                         let tool_call = tool_calls
//                             .into_iter()
//                             .next()
//                             .ok_or(anyhow!("No tool calls available despite action required."))?;

//                         self.last_tool_call = Some(tool_call.clone());

//                         info!("Assistant triggered function {}", &tool_call.function.name);

//                         match tool_call.function.name.as_str() {
//                             "trade_items" => {
//                                 let character_trade_items = serde_json::from_str::<
//                                     CharacterTradeItems,
//                                 >(
//                                     &tool_call.function.arguments
//                                 )
//                                 .context("Unable to parse arguments from trade function call.")?;

//                                 info!(
//                                     "Character requested to trade {} for the player's {}",
//                                     &character_trade_items.your_item,
//                                     &character_trade_items.player_item
//                                 );

//                                 game_state
//                                     .character_interaction
//                                     .as_mut()
//                                     .ok_or(anyhow!("Character interaction not set on game state."))?
//                                     .propose_trade(
//                                         &character_trade_items.your_item,
//                                         &character_trade_items.player_item,
//                                     );
//                             }
//                             "give_item" => {
//                                 let character_give_item =
//                                     serde_json::from_str::<CharacterGiveItem>(
//                                         &tool_call.function.arguments,
//                                     )
//                                     .context(
//                                         "Unable to parse arguments from gift function call.",
//                                     )?;

//                                 info!(
//                                     "Character requested to give {} to the player.",
//                                     &character_give_item.item
//                                 );

//                                 game_state
//                                     .character_interaction
//                                     .as_mut()
//                                     .ok_or(anyhow!("Character interaction not set on game state."))?
//                                     .propose_gift(&character_give_item.item);
//                             }

//                             _ => bail!(
//                                 "Invalid function call received: {}",
//                                 tool_call.function.name.as_str()
//                             ),
//                         }

//                         info!("Ending interaction early, awaiting response from player to submit function response.");
//                         return Ok(PollResponse::RequiresAction);
//                     }
//                     "cancelling" | "cancelled" | "failed" | "expired" => {
//                         error!("The run has expired or has failed.");
//                         bail!("Assistant run failed.")
//                     }
//                     "completed" => break,
//                     "queued" | "in_progress" => {}
//                     _ => {
//                         error!(
//                             "Run returned a status of {} which is not handled.",
//                             &retrieve_run_response.status
//                         );
//                         bail!("Unknown status received for run retrieval.")
//                     }
//                 }
//             }

//             tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
//         }

//         info!("Completed run polling, state updated.");

//         Ok(PollResponse::Complete)
//     }

//     fn process_meta_commands(&mut self, message: String) -> Vec<String> {
//         let mut messages = Vec::new();

//         debug!("Processing meta-commands for string: {}", &message);

//         let message = {
//             // Sometimes the LLM returns the character name prefix, sometimes it does not
//             if message.starts_with(format!("{}:", &self.character.name).as_str()) {
//                 message.split(":").skip(1).collect::<String>()
//             } else {
//                 message
//             }
//         };

//         debug!("Modified initial string: {}", &message);

//         let mut current_message = Vec::new();
//         for word in message.split(" ") {
//             if word.starts_with("$emotion") {
//                 let new_message =
//                     format!("{}: {}", &self.character.name, current_message.join(" "));

//                 debug!("Extracted message: {}", &new_message);
//                 messages.push(new_message);

//                 current_message.clear();

//                 let emotion: String = word
//                     .chars()
//                     .skip(9)
//                     .take(word.chars().count() - 10)
//                     .collect();
//                 let emotion_message = format!("{} is feeling {}", &self.character.name, emotion);

//                 debug!("Extracted nonverbal: {}", &emotion_message);
//                 messages.push(emotion_message);
//             } else if word.starts_with("$action") {
//                 let new_message =
//                     format!("{}: {}", &self.character.name, current_message.join(" "));

//                 debug!("Extracted message: {}", &new_message);
//                 messages.push(new_message);

//                 current_message.clear();

//                 let action: String = word
//                     .chars()
//                     .skip(8)
//                     .take(word.chars().count() - 9)
//                     .collect();
//                 let action_message = format!("{} {}", &self.character.name, action);

//                 debug!("Extracted nonverbal: {}", &action_message);
//                 messages.push(action_message);
//             } else {
//                 current_message.push(word);
//             }
//         }

//         if !current_message.is_empty() {
//             let new_message = format!("{}: {}", &self.character.name, current_message.join(" "));

//             debug!("Extracted message: {}", &new_message);
//             messages.push(new_message);
//         }

//         debug!("Finished. Returning messages: {:?}", messages);

//         messages
//     }

//     pub async fn end_session(
//         &mut self,
//         openai_client: &OpenAIClient,
//         file_manager: &FileManager,
//         game_state: &mut GameState,
//     ) -> Result<String, anyhow::Error> {
//         info!("Ending character session.");

//         info!("Processing summary request as internal message prompt.");
//         let request = CharacterPromptRequest {
//             message: Some(String::from("$summarize")),
//             trade_accept: None,
//             end_conversation: None,
//         };

//         // TODO: Consider that a conversation may end in the middle of a trade request.

//         self.process_prompt(&request, openai_client, file_manager, game_state)
//             .await?;

//         info!("Extracting summary from newly modified game state.");

//         let summary = game_state
//             .character_interaction
//             .as_ref()
//             .ok_or(anyhow!("Missing charater interaction."))?
//             .messages
//             .last() // This is fine because the state won't be returned back to the UI until after the interaction is deleted anyway. The player will never see the summary.
//             .ok_or(anyhow!("Missing last message, cannot get summary."))?
//             .text
//             .clone();

//         info!("Savings summary to character previous conversations.");
//         self.character_save_data
//             .previous_conversations
//             .push(summary.clone());

//         self.character_save_data
//             .save(file_manager)
//             .context("Unable to save updated character data.")?;

//         game_state.end_character_interaction();

//         info!("Deleting character assistants and threads.");

//         let assistant_delete = openai_client.delete_assistant(&self.assistant_id);
//         let thread_delete = openai_client.delete_thread(&self.thread_id);

//         let (assistant_delete, thread_delete) = join!(assistant_delete, thread_delete);

//         assistant_delete.map_err(|e| anyhow!("Unable to delete character assistant: {:?}", e))?;
//         thread_delete.map_err(|e| anyhow!("Unable to delete character thread: {:?}", e))?;

//         Ok(summary)
//     }
// }

// enum PollResponse {
//     RequiresAction,
//     Complete,
// }
