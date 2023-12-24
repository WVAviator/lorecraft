pub enum OpenAIModel {
    Gpt4,
    Gpt3_5,
    DallE3,
    DallE2,
}

impl OpenAIModel {
    pub fn to_string(&self) -> String {
        match self {
            OpenAIModel::Gpt4 => "gpt-4-1106-preview".to_string(),
            OpenAIModel::Gpt3_5 => "gpt-3.5-turbo-1106".to_string(),
            OpenAIModel::DallE3 => "dall-e-3".to_string(),
            OpenAIModel::DallE2 => "dall-e-2".to_string(),
        }
    }
}
