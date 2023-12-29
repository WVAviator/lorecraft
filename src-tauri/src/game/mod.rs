use log::{info, trace};
use serde::{Deserialize, Serialize};

use crate::{
    application_state::ApplicationState,
    game::{
        narrative::Narrative,
        scene_summary::{scene_summary_input::SceneSummaryInput, SceneSummary},
        summary::Summary,
    },
    openai_client::{
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_size::ImageGenerationSize,
        },
        OpenAIClient,
    },
    utils::random::Random,
};

use tauri::State;

use self::image::Image;

mod image;
mod narrative;
mod scene_summary;
mod summary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub summary: Summary,
    pub cover_art: Image,
    pub narrative: Narrative,
    pub scene_summary: SceneSummary,
}

impl Game {
    pub async fn create_new(user_prompt: String, state: &State<'_, ApplicationState>) -> Self {
        let id = Random::generate_id();
        info!("Generated new game id: {}.", id);

        let openai = OpenAIClient::new();

        let summary = async {
            info!("Generating game summary information.");
            state
                .send_update(String::from("Generating game summary information."))
                .await;
            Summary::generate(&openai, &user_prompt).await
        };

        let summary = summary.await.expect("Failed to generate summary.");

        let name = summary.name.clone();
        info!("Generated game: {}.", name);
        trace!("Game summary: {:#?}", summary);

        let cover_art_path = format!("{}/cover_art.png", id);
        let cover_art = async {
            info!("Generating game cover art.");
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
            info!("Generating story narrative pages.");
            state
                .send_update(String::from("Generating story narrative pages."))
                .await;
            let narrative = Narrative::generate(&openai, &summary.summary).await;

            trace!("Generated narrative: {:#?}", narrative);
            narrative
        };

        let scene_summary = async {
            info!("Generating scene summary information.");
            state
                .send_update(String::from("Generating scene summary information."))
                .await;
            let scene_summary =
                SceneSummary::generate(&summary.summary, &summary.win_condition, &openai).await;

            trace!("Generated scene summary: {:#?}", scene_summary);
            scene_summary
        };

        let (cover_art, narrative, scene_summary) =
            tokio::join!(cover_art, narrative, scene_summary);
        let cover_art = cover_art.expect("Failed to generate cover art.");
        let narrative = narrative.expect("Failed to generate narrative.");
        let scene_summary = scene_summary.expect("Failed to generate scene summary.");

        info!("Game generation completed for game with id: {}.", id);

        let game = Self {
            id,
            name,
            summary,
            cover_art,
            narrative,
            scene_summary,
        };

        trace!("Generated full game: {:#?}", game);
        game
    }
}
