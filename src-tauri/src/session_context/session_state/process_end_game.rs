use anyhow::{anyhow, bail};
use log::info;
use serde_json::json;

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct ProcessEndGameState {}

impl ProcessEndGameState {
    pub async fn process(
        request: SessionRequest,
        game_state: &mut GameState,
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {

                let reason = arguments["reason"]
                    .as_str()
                    .ok_or(anyhow!(
                        "Unable to interpret arguments for end_game function."
                    ))?
                    .to_string();

                game_state.end_game = Some(reason);
                let output = json!({ "success": "true" }).to_string();

                info!("Processed end_game function with output: {}", &output);

                Ok(SessionState::SubmitToolOutputsState { run_id, tool_call_id, output })
            }
            _ => bail!("Invalid session request for add item processing state: {:?}. Expected ContinueProcessing.", &request),
        }
    }
}
