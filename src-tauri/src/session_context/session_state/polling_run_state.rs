use anyhow::{anyhow, bail};
use log::{error, info};

use crate::{
    game_state::GameState,
    openai_client::{retrieve_run::retrieve_run_response::ToolCall, OpenAIClient},
    session_context::{session_request::SessionRequest, session_state::SessionState},
};

pub struct PollingRunState {}

impl PollingRunState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
        run_id: String,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => loop {
                info!("Polling run status...");
                if let Ok(retrieve_run_response) = openai_client
                    .retrieve_run(&game_state.thread_id, &run_id)
                    .await
                {
                    match retrieve_run_response.status.as_str() {
                        "requires_action" => {
                            info!("Assistant requested function invocation.");
                            let tool_calls: Vec<ToolCall> = retrieve_run_response.required_action.ok_or(anyhow!("Received requires action status without required_action on response object."))?.submit_tool_outputs.tool_calls;
                            if tool_calls.len() > 1 {
                                bail!("Multiple function calls not supported.");
                            }
                            let tool_call = tool_calls
                                .into_iter()
                                .next()
                                .ok_or(anyhow!("No tool calls in response array."))?;
                            return Ok(SessionState::RequiresActionState { run_id, tool_call });
                        }
                        "cancelling" | "cancelled" | "failed" | "expired" => {
                            return Err(anyhow!("Run {} was cancelled or expired.", &run_id));
                        }
                        "completed" => {
                            info!("Completed run response received.");
                            return Ok(SessionState::ReadMessageState);
                        }
                        "queued" | "in_progress" => {}
                        _ => {
                            return Err(anyhow!(
                                "Received unknown status from run response: {}",
                                &retrieve_run_response.status
                            ));
                        }
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
            },
            _ => bail!(
                "Invalid session request for polling run state {:?}. Expected ContinueProcessing.",
                &session_request
            ),
        }
    }
}
