use futures::stream::FuturesOrdered;
use futures::StreamExt;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use tokio::{join, stream};

use crate::{
    application_state::ApplicationState,
    game::{
        character::{character_factory::CharacterFactory, Character},
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

mod character;
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
    pub scenes: Vec<Scene>,
    pub characters: Vec<Character>,
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

        let style_string = format!(
            "Use a style of {}. Use themes of {}.",
            &summary.art_style, &summary.art_theme
        );
        let image_factory = ImageFactory::new(&openai, &state.file_manager, &id, style_string);
        let character_factory = CharacterFactory::new(&summary.summary, &openai, &image_factory);

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

        let scene_details = async {
            let mut futures = Vec::new();
            for summarized_scene in &scene_summary.scenes {
                let openai_ref = &openai;
                let summary_ref = &summary;

                let future = async move {
                    SceneDetail::generate(&summary_ref.summary, &summarized_scene, openai_ref)
                        .await
                        .expect("Failed to generate scene detail.")
                };

                futures.push(future);
            }

            state
                .send_update(String::from("Generating scene details."))
                .await;

            let stream = futures::stream::iter(futures).buffered(5);
            stream.collect::<Vec<_>>().await
        };

        let scene_details = scene_details.await;

        let scenes = async {
            let mut futures = Vec::new();
            for scene_detail in &scene_details {
                let image_factory_ref = &image_factory;

                let future = async move {
                    let scene = Scene::from_scene_detail(scene_detail, image_factory_ref)
                        .await
                        .expect("Failed to generate scene.");

                    trace!("Generated scene: {:#?}", &scene);
                    scene
                };

                futures.push(future);
            }

            state.send_update(String::from("Generating scenes.")).await;

            let stream = futures::stream::iter(futures).buffered(3);
            stream.collect::<Vec<_>>().await
        };

        let characters = async {
            let mut futures = Vec::new();
            let character_factory_ref = &character_factory;
            for scene_detail in &scene_details {
                let future = async move {
                    let characters = character_factory_ref
                        .from_scene_detail(scene_detail)
                        .await
                        .expect("Failed to create characters.");

                    characters
                };

                futures.push(future);
            }

            let stream = futures::stream::iter(futures).buffered(3);
            stream
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<Character>>()
        };

        let (scenes, characters) = join!(scenes, characters);

        info!("Game generation completed for game with id: {}.", id);

        let game = Self {
            id,
            name,
            summary,
            cover_art,
            narrative,
            scene_summary,
            scenes,
            characters,
        };

        trace!("Generated full game: {:#?}", game);
        game
    }
}
