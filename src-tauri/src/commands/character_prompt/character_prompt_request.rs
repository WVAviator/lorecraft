use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPromptRequest {
    pub message: Option<String>,
    pub trade_accept: Option<bool>,
    pub end_conversation: Option<bool>,
}
