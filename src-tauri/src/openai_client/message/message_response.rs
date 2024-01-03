use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    id: String,
    object: String,
    created_at: u64,
    thread_id: String,
    role: String,
    content: Vec<Content>,
    file_ids: Vec<String>,
    assistant_id: Option<String>,
    run_id: Option<String>,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    value: String,
    annotations: Vec<String>,
}
