use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTradeItems {
    pub your_item: String,
    pub player_item: String,
}
