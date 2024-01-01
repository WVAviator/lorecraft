use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupRequest {
    openai_api_key: Option<String>,
}
