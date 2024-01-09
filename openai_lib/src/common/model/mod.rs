use std::fmt::Display;

#[allow(non_camel_case_types)]
pub enum ChatModel {
    Gpt_4_1106_Preview,
    Gpt_4_Vision_Preview,
    Gpt_4,
    Gpt_4_32k,
    Gpt_4_0613,
    Gpt_4_32k_0613,
    Gpt_35_Turbo_1106,
    Gpt_35_Turbo,
    Gpt_35_Turbo_16k,
    Gpt_35_Turbo_Instruct,
}

impl Display for ChatModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatModel::Gpt_4_1106_Preview => write!("{}", "gpt-4-1106-preview"),
            ChatModel::Gpt_4_Vision_Preview => write!("{}", "gpt-4-vision-preview"),
            ChatModel::Gpt_4 => write!("{}", "gpt-4"),
            ChatModel::Gpt_4_32k => write!("{}", "gpt-4-32k"),
            ChatModel::Gpt_4_0613 => write!("{}", "gpt-4-0613"),
            ChatModel::Gpt_4_32k_0613 => write!("{}", "gpt-4-32k-0613"),
            ChatModel::Gpt_35_Turbo_1106 => write!("{}", "gpt-3.5-turbo-1106"),
            ChatModel::Gpt_35_Turbo => write!("{}", "gpt-3.5-turbo"),
            ChatModel::Gpt_35_Turbo_16k => write!("{}", "gpt-3.5-turbo-16k"),
            ChatModel::Gpt_35_Turbo_Instruct => write!("{}", "gpt-3.5-turbo-instruct"),
        }
    }
}
