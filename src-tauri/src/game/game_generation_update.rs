use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameGenerationUpdate {
    game_id: String,
    message: String,
}

impl GameGenerationUpdate {
    pub fn new(game_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            game_id: game_id.into(),
            message: message.into(),
        }
    }
}
