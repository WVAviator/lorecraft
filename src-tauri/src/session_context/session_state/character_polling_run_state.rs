use anyhow::{bail, anyhow};
use log::{info, error};
use openai_lib::{OpenAIClient, run::{RunClient, RunStatus}};

use crate::{session_context::session_request::SessionRequest, game_state::GameState};

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
                        match retrieve_run_response.status {
                            RunStatus::RequiresAction => {
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
                            RunStatus::Cancelling | RunStatus::Cancelled | RunStatus::Failed | RunStatus::Expired => {
                                error!("The run has expired or has failed.");
                                bail!("Assistant run failed.")
                            }
                            RunStatus::Completed => return Ok(SessionState::CharacterReadMessageState),
                            RunStatus::Queued | RunStatus::InProgress => {},
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
