use anyhow::{anyhow, bail};
use log::{info, trace};
use openai_lib::{OpenAIClient, thread::ThreadClient, assistant::AssistantClient, message::{MessageClient, CreateMessageRequest}};
use serde_json::json;

use crate::{session_context::{session_request::SessionRequest, session_state::SessionState}, game_state::GameState};

pub struct CharacterEndInteractionState {}

impl CharacterEndInteractionState {
    pub async fn process(request: SessionRequest, openai_client: &OpenAIClient, game_state: &mut GameState, summary: Option<String>) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {

                let closed = &game_state.character_interaction.as_ref().ok_or(anyhow!("Unable to access character interaction."))?.closed;

                match closed {
                    true => {
                        let summary = summary.ok_or(anyhow!("Processing end of character interaction, but no summary provided."))?;
                        let character_id = game_state.character_interaction.as_ref().ok_or(anyhow!("Unable to access character interaction."))?.character_id.clone();

                        game_state.save_previous_conversation(&character_id, &summary);

                        let run_id = game_state.character_interaction.as_ref().ok_or(anyhow!("Unable to access character interaction."))?.initiating_run_id.clone();
                        let tool_call_id = game_state.character_interaction.as_ref().ok_or(anyhow!("Unable to access character interaction."))?.initiating_tool_call_id.clone();

                        let output = json!({
                            "conversation_summary": summary,
                        }).to_string();

                        let thread_id = game_state.character_interaction.as_ref().ok_or(anyhow!("Unable to access character interaction."))?.thread_id.clone();
                        let assistant_id = game_state.character_interaction.as_ref().ok_or(anyhow!("Unable to access character interaction."))?.assistant_id.clone();

                        openai_client.delete_thread(&thread_id).await.map_err(|e| anyhow!("Unable to delete thread: {:?}", e))?;
                        openai_client.delete_assistant(&assistant_id).await.map_err(|e| anyhow!("Unable to delete character assistant: {:?}", e))?;

                        game_state.end_character_interaction();

                        Ok(SessionState::SubmitToolOutputsState { run_id, tool_call_id, output })
                    }
                    false => {
                        game_state.character_interaction.as_mut().ok_or(anyhow!("Unable to access character interaction."))?.closed = true;

                        let prompt = String::from("$summarize");

                        let thread_id = game_state
                            .character_interaction
                            .as_ref()
                            .ok_or(anyhow!("No character interaction to read messages from."))?
                            .thread_id
                            .clone();
                        let create_message_response = openai_client
                            .create_message(CreateMessageRequest::builder().content(&prompt).build(), &thread_id)
                            .await
                            .map_err(|e| {
                                anyhow!(
                                    "Failed to create new character message and add to thread: {:?}",
                                    e
                                )
                            })?;

                        info!("New message appended to character thread. Adding to UI state.");
                        trace!("{:#?}", create_message_response);
                        Ok(SessionState::CharacterRunRequestState)
                    }
                }
            }
            _ => bail!("Invalid session request for character end interaction state: {:?}. Expected ContinueProcessing.", &request),
                    
        }
    }
}
