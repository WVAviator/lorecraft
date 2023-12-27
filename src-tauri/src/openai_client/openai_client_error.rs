pub enum OpenAIClientError {
    RequestFailed(String),
    ResponseBadStatus(String),
    IncompleteResponse(String),
    InvalidResponse(String),
}