use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadObject {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}
