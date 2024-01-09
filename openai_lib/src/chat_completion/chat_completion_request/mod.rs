use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::chat_completion_object::ChatCompletionMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionRequest {
    model: String, // TODO: Map to model enum
    messages: ChatCompletionMessage,
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
pub struct Tool {
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    description: Option<String>,
    name: String,
    parameters: Option<FunctionParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    type_: String,
    properties: HashMap<String, FunctionProperty>,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionProperty {
    #[serde(rename = "type")]
    type_: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ToolChoice {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    Function { name: String },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tool_choice_serializes() {
        let tool_choice = ToolChoice::None;
        let tool_choice_json = serde_json::to_string(&tool_choice).unwrap();
        assert_eq!(tool_choice_json, r#""none""#);

        let tool_choice = ToolChoice::Function {
            name: String::from("test"),
        };
        let tool_choice_json = serde_json::to_string(&tool_choice).unwrap();
        assert_eq!(tool_choice_json, r#"{"name":"test"}"#);
    }
}
