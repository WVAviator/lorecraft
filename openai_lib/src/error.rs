#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Missing required property: {0}.")]
    MissingRequiredProperty(String),
    #[error("Unable to serialize JSON. Error: {0:?}")]
    SerializationFailure(#[source] anyhow::Error),
    #[error("Unable to deserialize JSON: {0:?}")]
    DeserializationFailure(#[source] anyhow::Error),
    #[error("Failed to initialize OpenAIClient with the provided configuration: {0:?}")]
    ConfigurationFailure(#[source] anyhow::Error),
    #[error("Failed to complete request to OpenAI API: {0:?}")]
    RequestFailure(#[source] anyhow::Error),
    #[error("Received bad status from OpenAI API: {0:?}")]
    ResponseFailure(reqwest::StatusCode),
    #[error("Request violates API restrictions: {0:?}")]
    InvalidRequestField(String),
    #[error("An error occurred attempting to rate limit requests: {0:?}")]
    RateLimitFailure(#[source] anyhow::Error),
    #[error("An error occurred attempting to read a file: {0:?}")]
    FileReadFailure(#[source] anyhow::Error),
    #[error(
        "The provided input violates OpenAI's content policy for the following reasons: {0:?}"
    )]
    ContentPolicyViolation(Vec<String>),
    #[error("An error occurred attempting to extract bytes from response: {0:?}")]
    ByteResponseFailure(#[source] anyhow::Error),
}
