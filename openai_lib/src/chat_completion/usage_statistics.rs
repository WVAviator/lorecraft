use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageStatistics {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}
