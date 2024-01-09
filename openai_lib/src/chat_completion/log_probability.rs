use serde::{Serialize, Deserialize};

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
