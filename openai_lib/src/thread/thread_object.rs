use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadObject {
    id: String,
    object: String,
    created_at: u64,
    metadata: HashMap<String, String>,
}
