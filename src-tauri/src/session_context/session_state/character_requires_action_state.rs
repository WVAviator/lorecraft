use anyhow::{bail, Context};
use openai_lib::tool::ToolCall;

use crate::session_context::session_request::SessionRequest;

use super::SessionState;

pub struct CharacterRequiresActionState {}

impl CharacterRequiresActionState {
    pub async fn process(
        request: SessionRequest,
        run_id: String,
        tool_call: ToolCall,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                let tool_call_id = tool_call.id.clone();
                // let arguments =
                //     serde_json::from_str::<serde_json::Value>(&tool_call.function.arguments)
                let arguments = tool_call
                    .extract_arguments()
                    .context("Unable to parse arguments from tool call.")?;

                match tool_call.get_name().as_str() {
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
                        tool_call.get_name().as_str()
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
