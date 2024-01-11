use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::openai_client::assistant_tool::{function::Function, AssistantTool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantCreateResponse {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub model: String,
    pub instructions: Option<String>,
    pub tools: Vec<AssistantTool>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}
