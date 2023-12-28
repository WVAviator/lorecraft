use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    game::{narrative::Narrative, summary::Summary},
    openai_client::OpenAIClient,
};

mod narrative;
mod summary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub summary: Summary,
    pub narrative: Narrative,
}

impl Game {
    pub async fn create_new(user_prompt: Option<&str>) -> Self {
        let id = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();

        let openai = OpenAIClient::new();

        let user_prompt = match user_prompt {
            Some(user_prompt) => user_prompt.to_string(),
            None => String::from("choose any random unique game idea"),
        };

        let summary = Summary::generate(&openai, &user_prompt)
            .await
            .expect("Failed to generate summary.");

        let name = summary.name.clone();

        println!("Generated summary:\n{:?}", summary);

        let narrative = Narrative::generate(&openai, &summary.summary)
            .await
            .expect("Failed to generate narrative.");

        Self {
            id,
            name,
            summary,
            narrative,
        }
    }
}
