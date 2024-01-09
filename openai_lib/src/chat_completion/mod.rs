use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionObject {
    pub id: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub created: i64,
    pub model: String, //TODO: Map this to a model enum
    pub system_fingerprint: String,
    pub object: String,
    pub usage: UsageStatistics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoice {
    finish_reason: String,
    index: usize,
    message: ChatCompletionMessage,
    logprobs: Option<LogProbabilityInformation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionMessage {
    content: Option<String>,
    role: String, // TODO: Map to enum for role
    tool_calls: Vec<ToolCall>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogProbabilityInformation {
    content: Option<Vec<LogProbabilityContent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogProbabilityContent {
    token: String,
    logprob: i64,
    bytes: Option<Vec<u8>>,
    top_logprobs: Vec<TopLogProbability>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopLogProbability {
    token: String,
    logprob: i64,
    bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageStatistics {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}
