use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{file_manager::FileManager, game::{Game, character::Character}};

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
    pub fn from_character(
        character: &Character,
    ) -> Result<Self, anyhow::Error> {

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
