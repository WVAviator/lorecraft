use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    role: String,
    content: String,
}

impl MessageRequest {
    pub fn new(content: &str) -> Self {
        MessageRequest {
            role: String::from("user"),
            content: String::from(content),
        }
    }

    pub fn to_request_body(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).context("Unable to serialize message request.")
    }
}
