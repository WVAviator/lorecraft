use anyhow::{bail, anyhow};
use log::info;
use openai_lib::{run::{SubmitToolOutputsRequest, RunClient}, OpenAIClient};

use crate::{session_context::session_request::SessionRequest,  game_state::GameState};

use super::SessionState;

pub struct SubmitToolOutputsState {}

impl SubmitToolOutputsState {
    pub async fn process(request: SessionRequest, openai_client: &OpenAIClient, game_state: &mut GameState, run_id: String, tool_call_id: String, output: String) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {

                info!("Sending tool outputs to run {}", &run_id);

                let thread_id = &game_state.thread_id;

                let submit_tool_outputs_request = SubmitToolOutputsRequest::builder()
                    .add_tool_output(&tool_call_id, &output)
                    .build();
                    
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
