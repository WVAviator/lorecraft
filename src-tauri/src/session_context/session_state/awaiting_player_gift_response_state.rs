use anyhow::{anyhow, bail};
use log::{info, trace};
use openai_lib::{
    run::{RunClient, SubmitToolOutputsRequest},
    OpenAIClient,
};
use serde_json::json;

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct AwaitingPlayerGiftResponseState {}

impl AwaitingPlayerGiftResponseState {
    pub async fn process(
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
        run_id: String,
        tool_call_id: String,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::CharacterTradeResponse(accepted) => {
                let output = match accepted {
                    true => {
                        info!("Processing trade acceptance.");

                        let trade = game_state
                            .character_interaction
                            .as_mut()
                            .ok_or(anyhow!("Missing character interaction."))?
                            .trade
                            .take()
                            .ok_or(anyhow!("No active trade to process in state."))?;

                        let item = trade.to_player.ok_or(anyhow!("Missing to player item."))?;

                        let character_id = game_state
                            .character_interaction
                            .as_ref()
                            .ok_or(anyhow!("Missing character interaction."))?
                            .character_id
                            .clone();

                        game_state.remove_character_item(&character_id, &item);
                        game_state.add_player_item(&item);

                        let updated_character_inventory =
                            game_state.get_character_inventory(&character_id);
                        let updated_player_inventory = game_state.get_player_inventory();

                        json!({ "player_response": "accept", "updated_player_inventory": updated_player_inventory, "updated_character_inventory": updated_character_inventory}).to_string()
                    }
                    false => {
                        info!("Processing declined trade request.");

                        game_state
                            .character_interaction
                            .as_mut()
                            .ok_or(anyhow!("Missing character interaction."))?
                            .trade = None;

                        json!({ "player_response": "decline" }).to_string()
                    }
                };

                info!("Submitting trade function tool outputs response.");

                let submit_tool_outputs_request = SubmitToolOutputsRequest::builder()
                    .add_tool_output(&tool_call_id, &output)
                    .build();

                let thread_id = &game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("Missing character interaction."))?
                    .thread_id;

                let submit_tool_outputs_response = openai_client
                    .submit_tool_outputs(submit_tool_outputs_request, &thread_id, &run_id)
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "Unable to submit tool outputs for character session: {:?}",
                            e
                        )
                    })?;

                trace!(
                    "Received tool outputs response:\n{:?}",
                    submit_tool_outputs_response
                );

                Ok(SessionState::CharacterPollingRunState { run_id })
            }
            _ => bail!(
                "Unexpected request received for AwaitingPlayerGiftResponseState: {:?}.",
                request
            ),
        }
    }
}
