use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::openai_client::assistant_tool::function::Function;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrieveRunResponse {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub assistant_id: String,
    pub thread_id: String,
    pub status: String,
    pub started_at: u32,
    pub expires_at: Option<u32>,
    pub cancelled_at: Option<u32>,
    pub failed_at: Option<u32>,
    pub completed_at: Option<u32>,
    pub last_error: Option<String>,
    pub model: String,
    pub instructions: Option<String>,
    pub tools: Vec<Function>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub required_action: Option<RequiredAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredAction {
    #[serde(rename = "type")]
    pub type_: String,
    pub submit_tool_outputs: SubmitToolOutputs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitToolOutputs {
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}
