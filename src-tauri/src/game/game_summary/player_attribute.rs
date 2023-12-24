use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttribute {
    pub name: String,
    pub description: String,
    pub advancement: String,
    pub significance: String,
    pub starting_value: u32,
}
