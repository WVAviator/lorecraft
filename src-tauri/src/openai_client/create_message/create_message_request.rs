use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    role: String,
    content: String,
}

impl CreateMessageRequest {
    pub fn new(content: &str) -> Self {
        CreateMessageRequest {
            role: String::from("user"),
            content: String::from(content),
        }
    }

    pub fn to_request_body(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).context("Unable to serialize message request.")
    }
}
