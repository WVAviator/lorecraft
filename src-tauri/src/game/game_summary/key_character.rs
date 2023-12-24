use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCharacter {
    pub name: String,
    pub description: String,
    pub significance: String,
    pub image: String,
}
