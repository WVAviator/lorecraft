use self::{choice::Choice, usage::Usage};
use serde::{Deserialize, Serialize};

mod choice;
mod message;
mod usage;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl ChatCompletionResponse {
    pub fn valid(&self) -> bool {
        match self.choices[0].finish_reason.as_str() {
            "stop" => true,
            _ => false,
        }
    }

    pub fn get_content(&self) -> String {
        self.choices[0].message.content.clone()
    }
}
