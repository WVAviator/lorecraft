// API Docs Reference https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo
pub enum ChatCompletionModel {
    Gpt_4_1106_Preview,
    Gpt_4_Vision_Preview,
    Gpt_4,
    Gpt_4_32k,
    Gpt_35_Turbo_1106,
    Gpt_35_Turbo,
    Gpt_35_Turbo_16k,
}

impl ChatCompletionModel {
    pub fn to_string(&self) -> String {
        match self {
            ChatCompletionModel::Gpt_4_1106_Preview => String::from("gpt-4-1106-preview"),
            ChatCompletionModel::Gpt_4_Vision_Preview => String::from("gpt-4-vision-preview"),
            ChatCompletionModel::Gpt_4 => String::from("gpt-4"),
            ChatCompletionModel::Gpt_4_32k => String::from("gpt-4-32k"),
            ChatCompletionModel::Gpt_35_Turbo_1106 => String::from("gpt-3.5-turbo-1106"),
            ChatCompletionModel::Gpt_35_Turbo => String::from("gpt-3.5-turbo"),
            ChatCompletionModel::Gpt_35_Turbo_16k => String::from("gpt-3.5-turbo-16k"),
        }
    }
}
