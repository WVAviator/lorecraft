pub enum SetupError {
    FileSystemError(String),
    MissingOpenAIKey(String),
    ConnectionFailed(String),
    BadOpenAIKey(String),
}

impl SetupError {
    pub fn get_type(&self) -> String {
        match self {
            SetupError::FileSystemError(_) => String::from("file_system_error"),
            SetupError::MissingOpenAIKey(_) => String::from("missing_openai_key"),
            SetupError::ConnectionFailed(_) => String::from("connection_failed"),
            SetupError::BadOpenAIKey(_) => String::from("bad_openai_key"),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            SetupError::FileSystemError(message) => message.clone(),
            SetupError::MissingOpenAIKey(message) => message.clone(),
            SetupError::ConnectionFailed(message) => message.clone(),
            SetupError::BadOpenAIKey(message) => message.clone(),
        }
    }
}
