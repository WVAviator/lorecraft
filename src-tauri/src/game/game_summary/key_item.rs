use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyItem {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_location")]
    pub location: String,
    #[serde(default = "default_significance")]
    pub significance: String,
    #[serde(default = "default_image")]
    pub image: String,
}

fn default_name() -> String {
    warn!("AI-provided key item name is missing.");
    String::new()
}

fn default_description() -> String {
    warn!("AI-provided key item description is missing.");
    String::new()
}

fn default_location() -> String {
    warn!("AI-provided key item location is missing.");
    String::new()
}

fn default_significance() -> String {
    warn!("AI-provided key item significance is missing.");
    String::new()
}

fn default_image() -> String {
    warn!("AI-provided key item image is missing.");
    String::new()
}

