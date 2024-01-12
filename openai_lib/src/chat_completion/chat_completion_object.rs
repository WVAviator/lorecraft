use serde::{Deserialize, Serialize};

use crate::model::ChatModel;

use super::{
    chat_completion_message::ChatCompletionMessage, log_probability::LogProbabilityInformation,
    usage_statistics::UsageStatistics,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionObject {
    pub id: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub created: i64,
    pub model: ChatModel,
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
