use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ThreadCreateResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}
