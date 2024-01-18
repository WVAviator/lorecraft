use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{model::ChatModel, tool::Tool};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssistantObject {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub model: ChatModel,
    pub instructions: Option<String>,
    pub tools: Vec<Tool>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}
