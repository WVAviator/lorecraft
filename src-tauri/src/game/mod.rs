use futures::stream::FuturesOrdered;
use futures::StreamExt;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use tokio::stream;

use crate::{
    application_state::ApplicationState,
    game::{
        image::image_factory::ImageFactory,
        narrative::Narrative,
        scene::Scene,
        scene_detail::SceneDetail,
        scene_summary::{scene_summary_input::SceneSummaryInput, SceneSummary},
        summary::Summary,
    },
    openai_client::{
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_request::ImageGenerationRequest,
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
mod scene;
mod scene_detail;
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

        let image_factory = ImageFactory::new(&openai, &state.file_manager, &id);

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

        let cover_art = async {
            info!("Generating game cover art.");
            state
                .send_update(String::from("Generating game cover art."))
                .await;
            let image_generation_request = ImageGenerationRequest::new(
                summary.cover_art.clone(),
                ImageGenerationModel::Dall_E_3,
                ImageGenerationSize::Size1792x1024,
            );
            image_factory
                .generate_image(image_generation_request, "cover_art.png")
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

        // let scenes = {
        //     let futures_ordered: FuturesOrdered<_> = scene_summary
        //         .scenes
        //         .iter()
        //         .map(|summarized_scene| async {
        //             let scene_detail =
        //                 SceneDetail::generate(&summary.summary, summarized_scene, &openai)
        //                     .await
        //                     .expect("Failed to generate scene detail.");
        //             Scene::from_scene_detail(scene_detail, &image_factory)
        //                 .await
        //                 .expect("Failed to generate scene.")
        //         })
        //         .collect();

        //     futures_ordered.collect::<Vec<_>>().await
        // };

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
