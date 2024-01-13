use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}
