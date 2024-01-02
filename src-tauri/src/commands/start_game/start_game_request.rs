use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct StartGameRequest {
  pub game_id: String,
}