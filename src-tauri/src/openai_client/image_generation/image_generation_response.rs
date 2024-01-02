use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageGenerationResponse {
    pub created: i64,
    pub data: Vec<ImageB64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageB64 {
    pub b64_json: String,
    pub revised_prompt: Option<String>,
}
