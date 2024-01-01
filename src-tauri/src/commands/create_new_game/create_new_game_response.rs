use serde::{Deserialize, Serialize};

use crate::game::Game;

use super::create_new_game_error::CreateNewGameError;

#[derive(Serialize, Deserialize)]
pub struct CreateNewGameSuccessResponse {
    success: bool,
    game: Game,
}

impl CreateNewGameSuccessResponse {
    pub fn new(game: Game) -> Self {
        CreateNewGameSuccessResponse {
            success: true,
            game,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateNewGameFailureResponse {
    success: bool,
    error: String,
    message: String,
}

impl CreateNewGameFailureResponse {
    pub fn new(error: CreateNewGameError) -> Self {
        CreateNewGameFailureResponse {
            success: false,
            error: error.get_type(),
            message: error.get_message(),
        }
    }
}
