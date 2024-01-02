#[derive(Debug)]
pub enum OpenAIClientError {
    RequestFailed(String),
    ResponseBadStatus(String),
    InvalidResponse(String),
    MaxAttemptsExceeded(String),
    NotAuthorized,
}
