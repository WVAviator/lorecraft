use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{model::ChatModel, tool::Tool};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssistantObject {
    id: String,
    object: String,
    created_at: u64,
    name: Option<String>,
    description: Option<String>,
    model: ChatModel,
    instructions: Option<String>,
    tools: Vec<Tool>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>,
}
