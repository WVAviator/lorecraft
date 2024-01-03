use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePromptError {
    pub message: String,
}

impl GamePromptError {
    pub fn new(message: &str) -> Self {
        GamePromptError {
            message: message.to_string(),
        }
    }
}
