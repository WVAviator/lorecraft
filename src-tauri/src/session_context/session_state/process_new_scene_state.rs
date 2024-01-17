use anyhow::{anyhow, bail};
use log::info;
use serde_json::json;

use crate::{game::Game, game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct ProcessNewSceneState {}

impl ProcessNewSceneState {
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

                info!("Processing scene change.");

                let new_scene: String = arguments["scene"]
                    .as_str()
                    .ok_or(anyhow!("Arguments provided in invalid format."))?
                    .to_string();
                let new_scene = game
                    .scenes
                    .iter()
                    .find(|s| s.name == new_scene)
                    .ok_or(anyhow!("Invalid scene name provided: {}.", new_scene))?;

                info!("Updating scene to {} in game state.", &new_scene.name);
                game_state.new_scene(&new_scene.name);

                let characters = new_scene
                    .characters
                    .iter()
                    .map(
                        |ch1| match game.characters.iter().find(|ch2| ch1.eq(&ch2.name)) {
                            Some(c) => {
                                format!("{}: {}", c.name, c.short_description)
                            }
                            None => format!("{}: a stranger", ch1),
                        },
                    )
                    .collect::<Vec<String>>();

                let output = json!({
                    "name": new_scene.name.clone(),
                    "narrative": new_scene.narrative.clone(),
                    "metadata": new_scene.metadata.clone(),
                    "characters": characters,
                    "items": new_scene.items.clone() // TODO: This should be the saved scene items
                    // from game_state
                }).to_string();

                Ok(SessionState::SubmitToolOutputsState { run_id, tool_call_id, output })
            }
            _ => bail!("Invalid request type received for new scene processing state: {:?}. Expected ContinueProcessing.", &request),
        }
    }
}
