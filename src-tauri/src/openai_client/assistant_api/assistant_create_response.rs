use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::openai_client::assistant_tool::function::Function;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantCreateResponse {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub name: String,
    pub description: Option<String>,
    pub model: String,
    pub instructions: String,
    pub tools: Vec<Function>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}
