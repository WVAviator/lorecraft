use anyhow::{bail, anyhow};
use log::info;

use crate::{session_context::session_request::SessionRequest, openai_client::{OpenAIClient, submit_tool_outputs::submit_tool_outputs_request::SubmitToolOutputsRequest}, game_state::GameState};

use super::SessionState;

pub struct SubmitToolOutputsState {}

impl SubmitToolOutputsState {
    pub async fn process(request: SessionRequest, openai_client: &OpenAIClient, game_state: &mut GameState, run_id: String, tool_call_id: String, output: String) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {

                info!("Sending tool outputs to run {}", &run_id);

                let thread_id = &game_state.thread_id;

                let mut submit_tool_outputs_request = SubmitToolOutputsRequest::new();
                submit_tool_outputs_request.add_output(&tool_call_id, &output);
                    
                openai_client
                    .submit_tool_outputs(
                        submit_tool_outputs_request,
                        &thread_id,
                        &run_id,
                    )
                    .await
                    .map_err(|e| anyhow!("Unable to submit tool outputs: {:?}", e))?;

                Ok(SessionState::PollingRunState { run_id })
            }
            _ => bail!("Invalid session request for submit tool outputs state: {:?}. Expected ContinueProcessing.", &request),
                    
        }
    }
}
