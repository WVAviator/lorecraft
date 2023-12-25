use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyArea {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_significance")]
    pub significance: String,
}

fn default_name() -> String {
    warn!("AI-provided key area name is missing.");
    String::new()
}

fn default_description() -> String {
    warn!("AI-provided key area description is missing.");
    String::new()
}

fn default_significance() -> String {
    warn!("AI-provided key area significance is missing.");
    String::new()
}
