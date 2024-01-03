use serde::{Deserialize, Serialize};

use crate::game_state::GameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePromptResponse {
    pub game_state: GameState,
}

impl GamePromptResponse {
    pub fn new(game_state: &GameState) -> Self {
        GamePromptResponse {
            game_state: game_state.clone(),
        }
    }
}
