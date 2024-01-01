pub enum CreateNewGameError {
    FileSystemError(String),
    SetupError(String),
    GameGenerationError(String),
}

impl CreateNewGameError {
    pub fn get_type(&self) -> String {
        match self {
            CreateNewGameError::FileSystemError(_) => String::from("file_system_error"),
            CreateNewGameError::SetupError(_) => String::from("setup_error"),
            CreateNewGameError::GameGenerationError(_) => String::from("game_generation_error"),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            CreateNewGameError::FileSystemError(message) => message.clone(),
            CreateNewGameError::SetupError(message) => message.clone(),
            CreateNewGameError::GameGenerationError(message) => message.clone(),
        }
    }
}
