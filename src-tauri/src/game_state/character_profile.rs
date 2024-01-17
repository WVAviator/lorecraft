use serde::{Deserialize, Serialize};

use crate::game::Character;

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
    pub fn from_character(character: &Character) -> Result<Self, anyhow::Error> {
        Ok(CharacterProfile {
            name: character.name.clone(),
            physical_description: character.physical_description.clone(),
            speech: character.speech.clone(),
            personality: character.personality.clone(),
            backstory: character.backstory.clone(),
            thoughts: character.thoughts.clone(),
        })
    }
}
