use serde::{Deserialize, Serialize};

use crate::openai_client::assistant_tool::function::Function;
use crate::openai_client::assistant_tool::AssistantTool;
use crate::openai_client::chat_completion::chat_completion_model::ChatCompletionModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantCreateRequest {
    pub instructions: String,
    pub name: String,
    pub tools: Vec<AssistantTool>,
    pub model: String,
}

impl AssistantCreateRequest {
    pub fn new(
        instructions: String,
        name: String,
        model: ChatCompletionModel,
        tools: Vec<AssistantTool>,
    ) -> Self {
        Self {
            instructions,
            name,
            tools,
            model: model.to_string(),
        }
    }

    pub fn to_request_body(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
