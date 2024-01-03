use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{game::Game, game_state::GameState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneUpdate {
    pub name: String,
    pub narrative: String,
    pub metadata: String,
    pub characters: Vec<String>,
    pub items: Vec<String>,
}
impl SceneUpdate {
    pub fn new_scene(
        arguments: serde_json::Value,
        game: &Game,
        game_state: &mut GameState,
    ) -> Result<Self, anyhow::Error> {
        let new_scene: String = arguments["scene"]
            .as_str()
            .ok_or(anyhow!("Arguments provided in invalid format."))?
            .to_string();
        let new_scene = game
            .scenes
            .iter()
            .find(|s| s.name == new_scene)
            .ok_or(anyhow!("Invalid scene name provided: {}.", new_scene))?;

        game_state.current_scene_id = Some(new_scene.id.clone());

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
            .collect();

        Ok(SceneUpdate {
            name: new_scene.name.clone(),
            narrative: new_scene.narrative.clone(),
            metadata: new_scene.metadata.clone(),
            characters,
            items: new_scene.items.clone(),
        })
    }
}
