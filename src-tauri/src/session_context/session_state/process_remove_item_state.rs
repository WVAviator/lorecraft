use anyhow::{anyhow, bail};
use serde_json::json;

use crate::{game::Game, game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct ProcessRemoveItemState {}

impl ProcessRemoveItemState {
    pub async fn process(
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

                let output = match game_state.get_player_inventory().iter().any(|i| i.eq(&item)) {
                    true => {
                        game_state.remove_item(&item);
                        let updated_player_inventory = game_state.get_player_inventory();
                        json!({
                            "updated_player_inventory": format!("[{}]", updated_player_inventory.join(", "))
                        }).to_string()
                    }
                    false => {
                        json!({
                            "error": format!("Player does not have any {} in their inventory.", &item)
                        }).to_string()
                    }
                };

                Ok(SessionState::SubmitToolOutputsState { run_id, tool_call_id, output })
            }
            _ => bail!("Invalid session request for add item processing state: {:?}. Expected ContinueProcessing.", &request),
        }
    }
}
