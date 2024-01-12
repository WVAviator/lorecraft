use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChatModel {
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt_4_1106_Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt_4_Vision_Preview,
    #[serde(rename = "gpt-4")]
    Gpt_4,
    #[serde(rename = "gpt-4-32k")]
    Gpt_4_32k,
    #[serde(rename = "gpt-4-0613")]
    Gpt_4_0613,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt_4_32k_0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt_35_Turbo_1106,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt_35_Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt_35_Turbo_16k,
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    Gpt_35_Turbo_Instruct,
}

impl Display for ChatModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatModel::Gpt_4_1106_Preview => write!(f, "gpt-4-1106-preview"),
            ChatModel::Gpt_4_Vision_Preview => write!(f, "gpt-4-vision-preview"),
            ChatModel::Gpt_4 => write!(f, "gpt-4"),
            ChatModel::Gpt_4_32k => write!(f, "gpt-4-32k"),
            ChatModel::Gpt_4_0613 => write!(f, "gpt-4-0613"),
            ChatModel::Gpt_4_32k_0613 => write!(f, "gpt-4-32k-0613"),
            ChatModel::Gpt_35_Turbo_1106 => write!(f, "gpt-3.5-turbo-1106"),
            ChatModel::Gpt_35_Turbo => write!(f, "gpt-3.5-turbo"),
            ChatModel::Gpt_35_Turbo_16k => write!(f, "gpt-3.5-turbo-16k"),
            ChatModel::Gpt_35_Turbo_Instruct => write!(f, "gpt-3.5-turbo-instruct"),
        }
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Test {
        model: ChatModel,
    }

    #[test]
    fn serializes_correctly() {
        let test = Test {
            model: ChatModel::Gpt_4_1106_Preview,
        };

        let expected = json!({
            "model": "gpt-4-1106-preview"
        });

        let actual = serde_json::to_value(test).unwrap();

        assert_json_include!(actual: actual, expected: expected);
    }

    #[test]
    fn deserializes_correctly() {
        let test = json!({
            "model": "gpt-4-1106-preview"
        });
        let expected = Test {
            model: ChatModel::Gpt_4_1106_Preview,
        };
        let actual: Test = serde_json::from_value(test).unwrap();
        assert_eq!(actual.model, expected.model);
    }

    #[test]
    fn to_string_works() {
        let test = ChatModel::Gpt_4_1106_Preview;
        let expected = "gpt-4-1106-preview";
        assert_eq!(test.to_string(), expected);
    }
}
