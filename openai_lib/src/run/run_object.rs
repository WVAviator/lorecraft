use serde::{Deserialize, Serialize};

use crate::{
    common::Metadata,
    model::ChatModel,
    tool::{Tool, ToolCall},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunObject {
    id: String,
    object: String,
    created_at: u64,
    thread_id: String,
    assistant_id: String,
    status: RunStatus,
    required_action: Option<RunRequiredAction>,
    last_error: Option<RunError>,
    expires_at: u64,
    started_at: Option<u64>,
    cancelled_at: Option<u64>,
    failed_at: Option<u64>,
    completed_at: Option<u64>,
    model: ChatModel,
    instructions: String,
    tools: Vec<Tool>,
    file_ids: Vec<String>,
    metadata: Metadata,
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
    code: RunErrorCode,
    message: String,
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
    type_: String,
    submit_tool_outputs: SubmitToolOutputs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitToolOutputs {
    tool_calls: Vec<ToolCall>,
}
