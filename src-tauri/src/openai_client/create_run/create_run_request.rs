use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::openai_client::assistant_tool::function::Function;

use super::create_run_request_builder::CreateRunRequestBuilderRequired;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRunRequest {
    pub assistant_id: String,
    pub model: Option<String>,
    pub instructions: Option<String>,
    pub additional_instructions: Option<String>,
    pub tools: Option<Vec<Function>>,
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateRunRequest {
    pub fn builder() -> CreateRunRequestBuilderRequired {
        CreateRunRequestBuilderRequired::new()
    }

    pub fn to_request_body(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).context("Unable to serialize run request.")
    }
}
