use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    #[serde(default = "default_narrative")]
    pub narrative: String,
    #[serde(default = "default_image")]
    pub image: String,
}

fn default_narrative() -> String {
    warn!("AI-provided narrative description is missing.");
    String::new()
}

fn default_image() -> String {
    warn!("AI-provided narrative image is missing.");
    String::new()
}
