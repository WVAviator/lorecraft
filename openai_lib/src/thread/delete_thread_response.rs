use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteThreadResponse {
    id: String,
    object: String,
    deleted: bool,
}
