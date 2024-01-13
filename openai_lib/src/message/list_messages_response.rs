use serde::{Deserialize, Serialize};

use super::MessageObject;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListMessagesResponse {
    object: String,
    pub data: Vec<MessageObject>,
}
