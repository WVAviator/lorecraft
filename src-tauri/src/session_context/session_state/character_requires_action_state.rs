use anyhow::{bail, Context};

use crate::{
    game_state::GameState, openai_client::retrieve_run::retrieve_run_response::ToolCall,
    session_context::session_request::SessionRequest,
};

use super::SessionState;

pub struct CharacterRequiresActionState {}

impl CharacterRequiresActionState {
    pub async fn process(
        request: SessionRequest,
        game_state: &mut GameState,
        run_id: String,
        tool_call: ToolCall,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                let tool_call_id = tool_call.id.clone();
                let arguments =
                    serde_json::from_str::<serde_json::Value>(&tool_call.function.arguments)
                        .context("Unable to parse arguments from tool call.")?;

                match tool_call.function.name.as_str() {
                    "trade_items" => {
                        return Ok(SessionState::ProcessCharacterTradeState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }
                    "give_item" => {
                        return Ok(SessionState::ProcessCharacterGiftState {
                            run_id,
                            tool_call_id,
                            arguments,
                        })
                    }

                    _ => bail!(
                        "Invalid function call received: {}",
                        tool_call.function.name.as_str()
                    ),
                }
            }
            _ => bail!(
                "Unexpected request received for ProcessCharacterGiftState: {:?}.",
                request
            ),
        }
    }
}
