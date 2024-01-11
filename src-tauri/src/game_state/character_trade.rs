use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTrade {
    pub to_player: Option<String>,
    pub from_player: Option<String>,
}
