use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyItem {
    pub name: String,
    pub description: String,
    pub location: String,
    pub significance: String,
    pub image: String,
}
