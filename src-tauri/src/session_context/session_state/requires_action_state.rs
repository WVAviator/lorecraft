use anyhow::{bail, Context};
use log::info;
use serde_json::Value;

use crate::{
    game_state::GameState,
    openai_client::{
        retrieve_run::retrieve_run_response::ToolCall,
        submit_tool_outputs::submit_tool_outputs_request::{SubmitToolOutputsRequest, ToolOutput},
        OpenAIClient,
    },
    session_context::{session_request::SessionRequest, session_state::SessionState},
};

pub struct RequiresActionState {}

impl RequiresActionState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
        run_id: String,
        tool_call: ToolCall,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => {
                let tool_call_id = tool_call.id.clone();
                let function_name = tool_call.function.name.as_str();
                let arguments = serde_json::from_str::<Value>(&tool_call.function.arguments)
                    .context("Unable to parse arguments from function call.")?;

                info!(
                    "Parsed arguments to function {}: {}",
                    &function_name, &tool_call.function.arguments
                );

                match function_name {
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
                        tool_call.function.name
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
