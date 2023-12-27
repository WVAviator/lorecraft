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
            Gpt_4_1106_Preview => String::from("gpt-4-1106-preview"),
            Gpt_4_Vision_Preview => String::from("gpt-4-vision-preview"),
            Gpt_4 => String::from("gpt-4"),
            Gpt_4_32k => String::from("gpt-4-32k"),
            Gpt_35_Turbo_1106 => String::from("gpt-3.5-turbo-1106"),
            Gpt_35_Turbo => String::from("gpt-3.5-turbo"),
            Gpt_35_Turbo_16k => String::from("gpt-3.5-turbo-16k"),
        }
    }

    pub fn request_body(&self) -> String {
        json
    }
}
