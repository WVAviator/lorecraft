pub mod tool_choice;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use self::tool_choice::ToolChoice;

use super::chat_completion_object::ChatCompletionMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRequest {
    model: String, // TODO: Map to model enum
    messages: Vec<ChatCompletionMessage>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<String, f32>>,
    logprobs: Option<bool>,
    top_logprobs: Option<u8>,
    max_tokens: Option<u32>,
    n: Option<u32>,
    presence_penalty: Option<f32>,
    response_format: Option<ResponseFormat>,
    seed: Option<u32>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Vec<Tool>,
    tool_choice: Option<ToolChoice>,
    user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

pub struct ChatCompletionRequestBuilder {
    model: Option<String>,
    messages: Vec<ChatCompletionMessage>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<String, f32>>,
    logprobs: Option<bool>,
    top_logprobs: Option<u8>,
    max_tokens: Option<u32>,
    n: Option<u32>,
    presence_penalty: Option<f32>,
    response_format: Option<ResponseFormat>,
    seed: Option<u32>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Vec<Tool>,
    tool_choice: Option<ToolChoice>,
    user: Option<String>,
}

impl ChatCompletionRequestBuilder {
    fn new() -> Self {
        ChatCompletionRequestBuilder {
            model: None,
            messages: Vec::new(),
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
            tools: Vec::new(),
            tool_choice: None,
            user: None,
        }
    }

    pub fn model(mut self, model: ChatModel) -> self {
        self.model = Some(model);
        self
    }
}
