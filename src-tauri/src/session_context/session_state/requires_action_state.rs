use anyhow::{bail, Context};
use log::info;
use openai_lib::tool::ToolCall;
use serde_json::Value;

use crate::session_context::{session_request::SessionRequest, session_state::SessionState};

pub struct RequiresActionState {}

impl RequiresActionState {
    pub async fn process(
        session_request: SessionRequest,
        run_id: String,
        tool_call: ToolCall,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => {
                let tool_call_id = tool_call.id.clone();
                let function_name = tool_call.get_name();
                let arguments = tool_call
                    .extract_arguments::<Value>()
                    .context("Unable to parse arguments from function call.")?;

                info!(
                    "Parsed arguments to function {}: {}",
                    &function_name, &arguments
                );

                match function_name.as_str() {
                    "new_scene" => {
                        return Ok(SessionState::ProcessNewSceneState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }
                    "add_item" => {
                        return Ok(SessionState::ProcessAddItemState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }
                    "remove_item" => {
                        return Ok(SessionState::ProcessRemoveItemState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }
                    "character_interact" => {
                        return Ok(SessionState::ProcessCharacterInteractState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }
                    "end_game" => {
                        return Ok(SessionState::ProcessEndGameState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }
                    _ => bail!(
                        "Received invalid function call request from narrator: {}.",
                        &function_name
                    ),
                }
            }
            _ => bail!(
                "Received invalid request type {:?}. Expected ContinueProcessing.",
                &session_request
            ),
        }
    }
}
