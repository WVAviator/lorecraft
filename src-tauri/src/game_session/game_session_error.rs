
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameSessionError {
    SetupFailure(String),
    ConfigError(String),
    APIError(String,)
}
