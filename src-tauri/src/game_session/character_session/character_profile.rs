use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{file_manager::FileManager, game::Game};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub name: String,
    pub physical_description: String,
    pub speech: String,
    pub personality: String,
    pub backstory: String,
    pub thoughts: String,
}

impl CharacterProfile {
    pub fn load(
        character_id: &str,
        game_id: &str,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        let game_path = format!("{}/game.json", game_id);
        let game_json = file_manager.read_from_file(&game_path)?;
        let game = serde_json::from_str::<Game>(&game_json)?;

        let character = game
            .characters
            .iter()
            .find(|c| c.id == character_id)
            .context("Character not found in game.")?;
        let character = character.clone();

        Ok(CharacterProfile {
            name: character.name,
            physical_description: character.physical_description,
            speech: character.speech,
            personality: character.personality,
            backstory: character.backstory,
            thoughts: character.thoughts,
        })
    }
}