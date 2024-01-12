use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ResponseFormat {
    #[serde(rename = "url")]
    URL,
    #[serde(rename = "b64_json")]
    B64Json,
}

impl Display for ResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseFormat::URL => write!(f, "url"),
            ResponseFormat::B64Json => write!(f, "b64_json"),
        }
    }
}
