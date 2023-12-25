use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttribute {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_advancement")]
    pub advancement: String,
    #[serde(default = "default_significance")]
    pub significance: String,
    #[serde(default = "default_starting_value")]
    pub starting_value: u32,
}

fn default_name() -> String {
    warn!("AI-provided player attribute name is missing.");
    String::new()
}

fn default_description() -> String {
    warn!("AI-provided player attribute description is missing.");
    String::new()
}

fn default_advancement() -> String {
    warn!("AI-provided player attribute advancement is missing.");
    String::new()
}

fn default_significance() -> String {
    warn!("AI-provided player attribute significance is missing.");
    String::new()
}

fn default_starting_value() -> u32 {
    warn!("AI-provided player attribute starting value is missing.");
    1
}