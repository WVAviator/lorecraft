use std::collections::HashMap;

use super::assisstant_tool::AssisstantTool;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssisstantCreateResponse {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub name: String,
    pub description: Option<String>,
    pub model: String,
    pub instructions: String,
    pub tools: Vec<AssisstantTool>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}
