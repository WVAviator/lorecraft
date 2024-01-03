use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMessage {
    pub text: String,
    pub is_dialog: bool,
}
