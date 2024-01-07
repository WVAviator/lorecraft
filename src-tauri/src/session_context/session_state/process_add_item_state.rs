use anyhow::{anyhow, bail};
use serde_json::json;

use crate::{game::Game, game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct ProcessAddItemState {}

impl ProcessAddItemState {
    pub fn process(
        request: SessionRequest,
        game_state: &mut GameState,
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
        game: &Game,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {

                let item = arguments["item"]
                    .as_str()
                    .ok_or(anyhow!("Invalid arguments to add_item function."))?
                    .to_string();
                // TODO: Remove the item from the scene
                game_state.add_item(&item);
                let updated_player_inventory = game_state.get_inventory();

                let output = json!({
                    "update_player_inventory": format!("[{}]", updated_player_inventory.join(", "))
                }).to_string();

                Ok(SessionState::SubmitToolOutputsState { run_id, tool_call_id, output })
            }
            _ => bail!("Invalid session request for add item processing state: {:?}. Expected ContinueProcessing.", &request),
        }
    }
}
