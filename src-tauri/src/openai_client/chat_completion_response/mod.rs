use serde::{Serialize, Deserialize};
use self::{choice::Choice, usage::Usage};

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
        match self.choices[0].finish_reason {
            "stop" => true,
            _ => false,
        }
    }

    pub fn get_content(&self) -> String {
        self.choices[0].message.content.clone()
    }
}


