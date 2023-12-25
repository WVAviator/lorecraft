use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCharacter {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_significance")]
    pub significance: String,
    #[serde(default = "default_image")]
    pub image: String,
}

fn default_name() -> String {
    warn!("AI-provided key character name is missing.");
    String::new()
}

fn default_description() -> String {
    warn!("AI-provided key character description is missing.");
    String::new()
}

fn default_significance() -> String {
    warn!("AI-provided key character significance is missing.");
    String::new()
}

fn default_image() -> String {
    warn!("AI-provided key character image is missing.");
    String::new()
}

