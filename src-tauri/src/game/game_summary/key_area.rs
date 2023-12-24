use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyArea {
    pub name: String,
    pub description: String,
    pub significance: String,
}
