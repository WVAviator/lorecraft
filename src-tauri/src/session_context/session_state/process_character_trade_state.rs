use anyhow::{anyhow, bail};
use log::info;

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct ProcessCharacterTradeState {}

impl ProcessCharacterTradeState {
    pub async fn process(
        request: SessionRequest,
        game_state: &mut GameState,
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                let to_player_item = arguments["your_item"]
                    .as_str()
                    .ok_or(anyhow!(
                        "Actor did not provide trade item in trade request."
                    ))?
                    .to_string();
                let from_player_item = arguments["player_item"]
                    .as_str()
                    .ok_or(anyhow!(
                        "Actor did not provide trade item in trade request."
                    ))?
                    .to_string();

                info!(
                    "Character requested to trade {} for the player's {}",
                    to_player_item, from_player_item
                );

                game_state
                    .character_interaction
                    .as_mut()
                    .ok_or(anyhow!("Character interaction not set on game state."))?
                    .propose_trade(&to_player_item, &from_player_item);

                Ok(SessionState::AwaitingPlayerTradeResponseState {
                    run_id,
                    tool_call_id,
                })
            }
            _ => bail!(
                "Unexpected request received for ProcessCharacterTradeState: {:?}.",
                request
            ),
        }
    }
}
