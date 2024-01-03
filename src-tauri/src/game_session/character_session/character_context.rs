use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterContext {
    character_inventory: Vec<String>,
    player_inventory: Vec<String>,
    previous_conversations: Vec<String>,
}
