use serde::{Deserialize, Serialize};

use crate::{
    common::Metadata,
    model::ChatModel,
    tool::{Tool, ToolCall},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunObject {
    pub id: String,
    pub object: String,
    pub created_at: Option<u64>,
    pub thread_id: String,
    pub assistant_id: String,
    pub status: RunStatus,
    pub required_action: Option<RunRequiredAction>,
    pub last_error: Option<RunError>,
    pub expires_at: Option<u64>,
    pub started_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub failed_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub model: ChatModel,
    pub instructions: String,
    pub tools: Vec<Tool>,
    pub file_ids: Vec<String>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RunStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunError {
    pub code: RunErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RunErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunRequiredAction {
    #[serde(rename = "type")]
    pub type_: String,
    pub submit_tool_outputs: SubmitToolOutputs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitToolOutputs {
    pub tool_calls: Vec<ToolCall>,
}
