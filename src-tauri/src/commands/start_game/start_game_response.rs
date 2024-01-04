use serde::{Deserialize, Serialize};

use crate::game_state::GameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartGameResponse {
    pub game_state: GameState,
}

impl StartGameResponse {
    pub fn new(game_state: GameState) -> Self {
        StartGameResponse { game_state }
    }
}
