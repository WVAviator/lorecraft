pub enum SetupError {
    FileSystemError(String),
    MissingOpenAIKey(String),
}

impl SetupError {
    pub fn get_type(&self) -> String {
        match self {
            SetupError::FileSystemError(_) => String::from("file_system_error"),
            SetupError::MissingOpenAIKey(_) => String::from("missing_openai_key"),
            _ => String::new(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            SetupError::FileSystemError(message) => message.clone(),
            SetupError::MissingOpenAIKey(message) => message.clone(),
            _ => String::new(),
        }
    }
}
