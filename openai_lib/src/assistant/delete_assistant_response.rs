use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAssistantResponse {
    id: String,
    object: String,
    deleted: bool,
}
