use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    application_state::ApplicationState,
    game::{narrative::Narrative, summary::Summary},
    openai_client::{
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_size::ImageGenerationSize,
        },
        OpenAIClient,
    },
};

use tauri::State;

use self::image::Image;

mod image;
mod narrative;
mod summary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub summary: Summary,
    pub cover_art: Image,
    pub narrative: Narrative,
}

impl Game {
    pub async fn create_new(
        user_prompt: Option<&str>,
        state: &State<'_, ApplicationState>,
    ) -> Self {
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

        let summary = async {
            state
                .send_update(String::from("Generating game summary information."))
                .await;
            Summary::generate(&openai, &user_prompt).await
        };

        let summary = summary.await.expect("Failed to generate summary.");

        let name = summary.name.clone();

        println!("Generated summary:\n{:?}", summary);

        let cover_art_path = format!("{}/cover_art.png", id);
        let cover_art = async {
            state
                .send_update(String::from("Generating game cover art."))
                .await;
            Image::from_image_prompt(
                &summary.cover_art,
                &openai,
                &cover_art_path,
                ImageGenerationModel::Dall_E_3,
                ImageGenerationSize::Size1792x1024,
                &state.file_manager,
            )
            .await
        };

        let narrative = async {
            state
                .send_update(String::from("Generating story narrative pages."))
                .await;
            Narrative::generate(&openai, &summary.summary).await
        };

        let (cover_art, narrative) = tokio::join!(cover_art, narrative);
        let cover_art = cover_art.expect("Failed to generate cover art.");
        let narrative = narrative.expect("Failed to generate narrative.");

        Self {
            id,
            name,
            summary,
            cover_art,
            narrative,
        }
    }
}
