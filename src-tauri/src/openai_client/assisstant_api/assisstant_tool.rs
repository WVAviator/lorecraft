use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssisstantTool {
    #[serde(rename = "type")]
    pub tool_type: String,
}
