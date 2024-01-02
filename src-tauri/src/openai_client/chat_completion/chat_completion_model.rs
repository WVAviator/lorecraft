// API Docs Reference https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo
pub enum ChatCompletionModel {
    Gpt4_1106Preview,
    Gpt4VisionPreview,
    Gpt4,
    Gpt4_32k,
    Gpt3_5Turbo1106,
    Gpt3_5Turbo,
    Gpt3_5Turbo16k,
}

impl ChatCompletionModel {
    pub fn to_string(&self) -> String {
        match self {
            ChatCompletionModel::Gpt4_1106Preview => String::from("gpt-4-1106-preview"),
            ChatCompletionModel::Gpt4VisionPreview => String::from("gpt-4-vision-preview"),
            ChatCompletionModel::Gpt4 => String::from("gpt-4"),
            ChatCompletionModel::Gpt4_32k => String::from("gpt-4-32k"),
            ChatCompletionModel::Gpt3_5Turbo1106 => String::from("gpt-3.5-turbo-1106"),
            ChatCompletionModel::Gpt3_5Turbo => String::from("gpt-3.5-turbo"),
            ChatCompletionModel::Gpt3_5Turbo16k => String::from("gpt-3.5-turbo-16k"),
        }
    }
}
