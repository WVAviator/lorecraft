use anyhow::{bail, anyhow};
use log::{info, error};

use crate::{session_context::session_request::SessionRequest, openai_client::OpenAIClient, game_state::GameState};

use super::SessionState;

pub struct CharacterPollingRunState {
}

impl CharacterPollingRunState {
    pub async fn process(request: SessionRequest, openai_client: &OpenAIClient, game_state: &mut GameState, run_id: String) -> Result<SessionState, anyhow::Error>{
        match request {
            SessionRequest::ContinueProcessing => {
                let thread_id = game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction available in game state."))?
                    .thread_id.clone();
                loop {
                    info!("Polling character run status...");
                    if let Ok(retrieve_run_response) = openai_client
                        .retrieve_run(
                            &thread_id,
                            &run_id,
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

                                return Ok(SessionState::CharacterRequiresActionState { run_id, tool_call })
                            }
                            "cancelling" | "cancelled" | "failed" | "expired" => {
                                error!("The run has expired or has failed.");
                                bail!("Assistant run failed.")
                            }
                            "completed" => return Ok(SessionState::CharacterReadMessageState),
                            "queued" | "in_progress" => {}
                            _ => {
                                error!(
                                    "Run returned a status of {} which is not handled.",
                                    &retrieve_run_response.status
                                );
                                bail!("Unknown status received for run retrieval.")
                            }
                        }
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
                }
            }
            _ => bail!(
                "Unexpected request received for CharacterPollingRunState: {:?}. Expected ContinueProcessing.",
                request
            ),
                    
        }
    }
}
