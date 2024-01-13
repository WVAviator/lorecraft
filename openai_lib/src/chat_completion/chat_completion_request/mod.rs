pub mod response_format;
pub mod tool_choice;

use std::collections::HashMap;

use log::warn;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    model::ChatModel,
    tool::{Tool, ToolCall},
    Error,
};

use self::{response_format::ResponseFormat, tool_choice::ToolChoice};

use super::chat_completion_message::ChatCompletionMessage;

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder)]
pub struct ChatCompletionRequest {
    model: ChatModel,
    #[builder(default = Vec::new(), via_mutators, mutators(
        pub fn add_system_message(&mut self, content: impl Into<String>) {
            self.messages.push(ChatCompletionMessage::system(content.into(), None));
        }
        pub fn add_user_message(&mut self, content: impl Into<String>) {
            self.messages.push(ChatCompletionMessage::user(content.into(), None));
        }
        pub fn add_assistant_message(&mut self, content: impl Into<String>) {
            self.messages.push(ChatCompletionMessage::assistant(content.into(), None));
        }
        pub fn add_assistant_tool_call(&mut self, tool_calls: Vec<ToolCall>) {
            self.messages
                .push(ChatCompletionMessage::assistant_tool_call(tool_calls, None));
        }
        pub fn add_tool_call_response(&mut self, content: impl Into<String>, tool_call_id: impl Into<String>) {
            self.messages
                .push(ChatCompletionMessage::tool(content.into(), tool_call_id.into()));
        }
    ))]
    messages: Vec<ChatCompletionMessage>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<String, f32>>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<bool>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<u8>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Tool>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl ChatCompletionRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        self.validate()?;
        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }

    fn validate(&self) -> Result<(), Error> {
        if let Some(v) = self.frequency_penalty {
            if v < -2.0 || v > 2.0 {
                return Err(Error::InvalidRequestField(format!("The frequency_penalty value {} does not fall within the required range -2.0, 2.0", v)));
            }
        }
        if let Some(v) = self.top_logprobs {
            if v > 5 {
                return Err(Error::InvalidRequestField(format!(
                    "The top_logprobs value {} does not fall within the required range 0, 5",
                    v
                )));
            }
        }
        if let Some(v) = self.n {
            if v > 1 {
                warn!("Generating more than 1 chat completion choice will incur costs for each choice generated.");
            }
        }
        if let Some(v) = self.presence_penalty {
            if v < -2.0 || v > 2.0 {
                return Err(Error::InvalidRequestField(format!("The presence_penalty value {} does not fall within the required range -2.0, 2.0", v)));
            }
        }
        if let Some(v) = self.temperature {
            if let Some(_) = self.top_p {
                warn!("It is not recommended to specify both top_p and temperature.")
            }
            if v > 2.0 || v < 0.0 {
                return Err(Error::InvalidRequestField(format!(
                    "The temperature value {} does not fall within the required range 0.0, 2.0",
                    v
                )));
            }
        }
        if let Some(v) = self.top_p {
            if v > 2.0 || v < 0.0 {
                return Err(Error::InvalidRequestField(format!(
                    "The top_p value {} does not fall within the required range 0.0, 2.0",
                    v
                )));
            }
        }

        match self {
            ChatCompletionRequest { response_format: Some(_), model, .. } => {
                match model {
                    ChatModel::Gpt_35_Turbo_1106 | ChatModel::Gpt_4_1106_Preview => {},
                    _ => Err(Error::InvalidRequestField(format!("The response_format field is only compatible with models gpt-3.5-turbo-1106 and gpt-4-1106-preview.")))?,
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;

    #[test]
    fn generates_correct_json_format() {
        let request = ChatCompletionRequest::builder()
            .add_system_message("foo")
            .add_user_message("bar")
            .model(ChatModel::Gpt_4)
            .build();

        let actual = serde_json::to_value(&request).unwrap();

        let expected = json!({
            "model": "gpt-4",
            "messages": [
              {
                "role": "system",
                "content": "foo"
              },
              {
                "role": "user",
                "content": "bar"
              }
            ]
        });

        assert_json_include!(actual: actual, expected: expected);
    }
}
