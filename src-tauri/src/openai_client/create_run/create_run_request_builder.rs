#![allow(dead_code)]

use std::collections::HashMap;

use crate::openai_client::{
    assistant_tool::function::Function, chat_completion::chat_completion_model::ChatCompletionModel,
};

use super::create_run_request::CreateRunRequest;

#[derive(Debug)]
pub struct CreateRunRequestBuilder {
    pub assistant_id: String,
    pub model: Option<String>,
    pub instructions: Option<String>,
    pub additional_instructions: Option<String>,
    pub tools: Option<Vec<Function>>,
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateRunRequestBuilder {
    pub fn model(mut self, model: ChatCompletionModel) -> Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn instructions(mut self, instructions: String) -> Self {
        self.instructions = Some(instructions);
        self
    }

    pub fn additional_instructions(mut self, additional_instructions: String) -> Self {
        self.additional_instructions = Some(additional_instructions);
        self
    }

    pub fn tools(mut self, tools: Vec<Function>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> CreateRunRequest {
        CreateRunRequest {
            assistant_id: self.assistant_id,
            model: self.model,
            instructions: self.instructions,
            additional_instructions: self.additional_instructions,
            tools: self.tools,
            metadata: self.metadata,
        }
    }
}

pub struct CreateRunRequestBuilderRequired {}

impl CreateRunRequestBuilderRequired {
    pub fn new() -> Self {
        CreateRunRequestBuilderRequired {}
    }

    pub fn assistant_id(self, assistant_id: &str) -> CreateRunRequestBuilder {
        CreateRunRequestBuilder {
            assistant_id: assistant_id.to_string(),
            model: None,
            instructions: None,
            additional_instructions: None,
            tools: None,
            metadata: None,
        }
    }
}
