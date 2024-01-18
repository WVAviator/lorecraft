use anyhow::{anyhow, bail};
use log::info;

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct ProcessCharacterGiftState {}

impl ProcessCharacterGiftState {
    pub async fn process(
        request: SessionRequest,
        game_state: &mut GameState,
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                let to_player_item = arguments["item"]
                    .as_str()
                    .ok_or(anyhow!("Actor did not provide item in gift request."))?
                    .to_string();

                info!(
                    "Character requested to gift {} to the player.",
                    &to_player_item
                );

                game_state
                    .character_interaction
                    .as_mut()
                    .ok_or(anyhow!("Character interaction not set on game state."))?
                    .propose_gift(&to_player_item);

                Ok(SessionState::AwaitingPlayerGiftResponseState {
                    run_id,
                    tool_call_id,
                })
            }
            _ => bail!(
                "Unexpected request received for ProcessCharacterGiftState: {:?}.",
                request
            ),
        }
    }
}
