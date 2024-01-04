pub struct CharacterPromptError {
    pub message: String,
}

impl CharacterPromptError {
    pub fn new(message: &str) -> Self {
        CharacterPromptError {
            message: String::from(message),
        }
    }
}
