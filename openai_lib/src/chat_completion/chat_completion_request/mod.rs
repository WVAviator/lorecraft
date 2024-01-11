pub mod tool_choice;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{error::Error, model::ChatModel, tool::Tool};

use self::tool_choice::ToolChoice;

use super::chat_completion_message::ChatCompletionMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRequest {
    model: ChatModel,
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
    model: Option<ChatModel>,
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

    pub fn model(mut self, model: ChatModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn add_message(mut self, message: ChatCompletionMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<String, f32>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    pub fn top_logprobs(mut self, top_logprobs: u8) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn json_format(mut self) -> Self {
        self.response_format = Some(ResponseFormat {
            type_: "json".to_string(),
        });
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn add_tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn build(self) -> Result<ChatCompletionRequest, Error> {
        let model = self
            .model
            .ok_or(Error::MissingRequiredProperty(String::from("model")))?;

        Ok(ChatCompletionRequest {
            model,
            messages: self.messages,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            stop: self.stop,
            stream: self.stream,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            user: self.user,
        })
    }
}
