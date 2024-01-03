use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::openai_client::assistant_tool::function::Function;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRunResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub assistant_id: String,
    pub thread_id: String,
    pub status: String,
    pub started_at: u64,
    pub expires_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub failed_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub last_error: Option<String>,
    pub model: String,
    pub instructions: Option<String>,
    pub tools: Vec<Function>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}
