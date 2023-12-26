use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
    file_manager::FileManager,
    openai::{openai_model::OpenAIModel, OpenAI},
};

use self::game_summary::GameSummary;

pub mod game_summary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    game_summary: GameSummary,
}

impl Game {
    pub fn create_new(user_prompt: Option<&str>) -> Self {
        let openai = OpenAI::new();

        println!("Building system prompt.");

        let main_prompt_path = "./prompts/architect/main.txt";
        let example1_prompt_path = "./prompts/architect/example1.yaml";
        let example2_prompt_path = "./prompts/architect/example2.yaml";

        let mut system_prompt = String::new();
        system_prompt += &fs::read_to_string(main_prompt_path)
            .expect("Something went wrong reading the architect system prompt file.");
        system_prompt += "\n\nExample 1:\n\n";
        system_prompt += &fs::read_to_string(example1_prompt_path)
            .expect("Something went wrong reading the architect system prompt example1 file.");
        system_prompt += "\n\nExample 2:\n\n";
        system_prompt += &fs::read_to_string(example2_prompt_path)
            .expect("Something went wrong reading the architect system prompt example2 file.");

        println!("Building user prompt.");

        let user_prompt = match user_prompt {
            Some(user_prompt) => user_prompt.to_string(),
            None => String::from("choose any random unique game idea"),
        };

        println!("Sending request to OpenAI API.");

        let response_text = openai
            .chat_completion_request(&system_prompt, &user_prompt, OpenAIModel::Gpt3_5)
            .expect("Failed to get response from OpenAI API.");

        println!(
            "Received response from OpenAI API:\n\n{}\n\nAttempting to parse YAML string...",
            response_text
        );

        FileManager::new()
            .write_to_file("game_summary.yaml", &response_text)
            .expect("Failed to write game summary to file.");

        let game_summary =
            GameSummary::from_yaml(&response_text).expect("Failed to parse YAML into GameSummary.");

        println!("Parsed game summary:\n{:?}", game_summary);

        Self {
            name: game_summary.name.clone(),
            game_summary,
        }
    }
}
