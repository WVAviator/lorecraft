use serde::{Deserialize, Serialize};

use crate::game_state::GameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPromptResponse {
    game_state: GameState,
}

impl CharacterPromptResponse {
    pub fn new(game_state: GameState) -> Self {
        Self { game_state }
    }
}
