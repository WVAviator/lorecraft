use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSaveData {
    pub previous_conversations: Vec<String>,
    pub character_inventory: Vec<String>,
}

impl CharacterSaveData {
    pub fn new(initial_inventory: Vec<String>) -> Self {
        CharacterSaveData {
            previous_conversations: vec![],
            character_inventory: initial_inventory,
        }
    }
}
