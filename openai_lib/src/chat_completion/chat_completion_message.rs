use crate::tool::ToolCall;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "role")]
pub enum ChatCompletionMessage {
    #[serde(rename = "system")]
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "user")]
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "assistant")]
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "tool")]
    Tool {
        content: String,
        tool_call_id: String,
    },
}

impl ChatCompletionMessage {
    pub fn system(content: String, name: Option<String>) -> Self {
        ChatCompletionMessage::System { content, name }
    }

    pub fn user(content: String, name: Option<String>) -> Self {
        ChatCompletionMessage::User { content, name }
    }

    pub fn assistant(content: String, name: Option<String>) -> Self {
        ChatCompletionMessage::Assistant {
            content: Some(content),
            tool_calls: None,
            name,
        }
    }

    pub fn assistant_tool_call(tool_calls: Vec<ToolCall>, name: Option<String>) -> Self {
        ChatCompletionMessage::Assistant {
            content: None,
            tool_calls: Some(tool_calls),
            name,
        }
    }

    pub fn tool(content: String, tool_call_id: String) -> Self {
        ChatCompletionMessage::Tool {
            content,
            tool_call_id,
        }
    }

    pub fn text(&self) -> String {
        match self {
            ChatCompletionMessage::System { content, .. } => content.clone(),
            ChatCompletionMessage::User { content, .. } => content.clone(),
            ChatCompletionMessage::Assistant { content, .. } => {
                content.clone().unwrap_or(String::new())
            }
            ChatCompletionMessage::Tool { content, .. } => content.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;

    #[test]
    fn properly_deserializes_system_variant() {
        let system_json = json!({
            "content": "test",
            "role": "system",
        })
        .to_string();
        let expected = ChatCompletionMessage::System {
            content: "test".to_string(),
            name: None,
        };
        let actual: ChatCompletionMessage = serde_json::from_str(&system_json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn properly_deserializes_user_variant() {
        let user_json = json!({
            "content": "test",
            "role": "user",
        })
        .to_string();
        let expected = ChatCompletionMessage::User {
            content: "test".to_string(),
            name: None,
        };
        let actual: ChatCompletionMessage = serde_json::from_str(&user_json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn properly_deserializes_assistant_variant() {
        let assistant_json = json!({
            "content": "test",
            "role": "assistant",
        })
        .to_string();
        let expected = ChatCompletionMessage::Assistant {
            content: Some("test".to_string()),
            tool_calls: None,
            name: None,
        };
        let actual: ChatCompletionMessage = serde_json::from_str(&assistant_json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn properly_deserializes_tool_variant() {
        let tool_json = json!({
            "content": "test",
            "role": "tool",
            "tool_call_id": "test",
        })
        .to_string();
        let expected = ChatCompletionMessage::Tool {
            content: "test".to_string(),
            tool_call_id: "test".to_string(),
        };
        let actual: ChatCompletionMessage = serde_json::from_str(&tool_json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn properly_serializes_system_variant() {
        let system_message = ChatCompletionMessage::System {
            content: "test".to_string(),
            name: None,
        };
        let expected = json!({
            "content": "test",
            "role": "system",
        });
        let actual = serde_json::to_value(&system_message).unwrap();
        assert_json_include!(actual: actual, expected: expected);
    }
}
