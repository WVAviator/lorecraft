#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing required property {0}.")]
    MissingRequiredProperty(&str),
    #[error("Unable to deserialize JSON. Error: {0:?}")]
    DeserializationFailure(serde_json::Error),
}
