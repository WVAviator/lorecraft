use crate::common::tool_call::ToolCall;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "role")]
pub enum ChatCompletionMessage {
    #[serde(rename = "system")]
    System {
        content: String,
        // role: String,
        name: Option<String>,
    },
    #[serde(rename = "user")]
    User {
        content: String,
        // role: String,
        name: Option<String>,
    },
    #[serde(rename = "assistant")]
    Assistant {
        content: Option<String>,
        // role: String,
        tool_calls: Option<Vec<ToolCall>>,
        name: Option<String>,
    },
    #[serde(rename = "tool")]
    Tool {
        content: String,
        // role: String,
        tool_call_id: String,
    },
}

impl ChatCompletionMessage {
    pub fn system(content: String, name: Option<String>) -> Self {
        ChatCompletionMessage::System {
            content,
            // role: String::from("system"),
            name,
        }
    }

    pub fn user(content: String, name: Option<String>) -> Self {
        ChatCompletionMessage::User {
            content,
            // role: String::from("user"),
            name,
        }
    }

    pub fn assistant(content: Option<String>, name: Option<String>) -> Self {
        ChatCompletionMessage::Assistant {
            content,
            // role: String::from("assistant"),
            tool_calls: None,
            name,
        }
    }

    pub fn assistant_tool_call(tool_calls: Vec<ToolCall>, name: Option<String>) -> Self {
        ChatCompletionMessage::Assistant {
            content: None,
            // role: String::from("assistant"),
            tool_calls: Some(tool_calls),
            name,
        }
    }

    pub fn tool(content: String, tool_call_id: String) -> Self {
        ChatCompletionMessage::Tool {
            content,
            // role: String::from("tool"),
            tool_call_id,
        }
    }
}

// impl<'de> Deserialize<'de> for ChatCompletionMessage {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let v: Value = Deserialize::deserialize(deserializer)?;

//         match v["role"].as_str() {
//             Some("system") => serde_json::from_value(v).map(ChatCompletionMessage::System),
//             Some("user") => serde_json::from_value(v).map(ChatCompletionMessage::User),
//             Some("assistant") => serde_json::from_value(v).map(ChatCompletionMessage::Assistant),
//             Some("tool") => serde_json::from_value(v).map(ChatCompletionMessage::Tool),
//             _ => Err(serde::de::Error::custom("Invalid role")),
//         }
//     }
// }

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
            // role: "system".to_string(),
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
            // role: "user".to_string(),
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
            // role: "assistant".to_string(),
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
            // role: "tool".to_string(),
            tool_call_id: "test".to_string(),
        };
        let actual: ChatCompletionMessage = serde_json::from_str(&tool_json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn properly_serializes_system_variant() {
        let system_message = ChatCompletionMessage::System {
            content: "test".to_string(),
            // role: "system".to_string(),
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
