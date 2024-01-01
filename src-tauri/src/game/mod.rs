use anyhow::anyhow;
use futures::StreamExt;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use tokio::{join, sync::MutexGuard};

use crate::{
    application_state::ApplicationState,
    game::{
        character::{character_factory::CharacterFactory, Character},
        image::image_factory::ImageFactory,
        item::item_factory::ItemFactory,
        narrative::{narrative_factory::NarrativeFactory, Narrative},
        scene::Scene,
        scene_detail::SceneDetail,
        scene_summary::SceneSummary,
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

use self::{image::Image, item::Item};

mod character;
mod image;
mod item;
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
    pub scenes: Vec<Scene>,
    pub characters: Vec<Character>,
    pub items: Vec<Item>,
}

impl Game {
    pub async fn create_new(
        user_prompt: String,
        state: &MutexGuard<'_, ApplicationState>,
    ) -> Result<Self, anyhow::Error> {
        let id = Random::generate_id();
        info!("Generated new game id: {}.", id);

        let openai_client = match &state.openai_client {
            Some(openai_client) => openai_client,
            None => {
                return Err(anyhow!(
                    "Unable to access OpenAI client from application state."
                ))
            }
        };

        let file_manager = match &state.file_manager {
            Some(file_manager) => file_manager,
            None => {
                return Err(anyhow!(
                    "Unable to access file manager from application state."
                ))
            }
        };

        let summary = async {
            info!("Generating game summary information.");
            state
                .send_update(String::from("Generating game summary information."))
                .await;
            Summary::generate(&openai_client, &user_prompt).await
        };

        let summary = summary.await.expect("Failed to generate summary.");

        let style_string = format!(
            "Use a style of {}. Use themes of {}.",
            &summary.art_style, &summary.art_theme
        );

        let image_factory = ImageFactory::new(&openai_client, &file_manager, &id, style_string);
        let narrative_factory =
            NarrativeFactory::new(&openai_client, &summary.summary, &image_factory);
        let character_factory =
            CharacterFactory::new(&openai_client, &summary.summary, &image_factory);
        let item_factory = ItemFactory::new(&openai_client, &summary.summary, &image_factory);

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
                .try_generate_image(image_generation_request, "cover_art.png", 3)
                .await
                .expect("Unable to generate cover art.")
        };

        let narrative = async {
            info!("Generating story narrative pages.");
            state
                .send_update(String::from("Generating story narrative pages."))
                .await;
            let narrative = narrative_factory
                .create()
                .await
                .expect("Unable to create narrative.");

            trace!("Generated narrative: {:#?}", narrative);
            narrative
        };

        let scene_details = async {
            info!("Generating scene summary information.");
            state
                .send_update(String::from("Generating scene summary information."))
                .await;
            let scene_summary =
                SceneSummary::generate(&summary.summary, &summary.win_condition, &openai_client)
                    .await
                    .expect("Unable to create scene summary.");

            trace!("Generated scene summary: {:#?}", scene_summary);

            async {
                let mut futures = Vec::new();
                for summarized_scene in &scene_summary.scenes {
                    let openai_ref = &openai_client;
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
            }
            .await
        };

        let (cover_art, narrative, scene_details) =
            tokio::join!(cover_art, narrative, scene_details);

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

        let scene_items = async {
            let item_list: Vec<String> = scene_details
                .iter()
                .map(|scene_detail| scene_detail.items.clone())
                .flatten()
                .collect();
            item_factory
                .create_items(item_list)
                .await
                .expect("Unable to create scene items.")
        };

        let (scenes, characters, scene_items) = join!(scenes, characters, scene_items);

        let character_items = async {
            let item_list: Vec<String> = characters
                .iter()
                .map(|character| character.inventory.clone())
                .flatten()
                .collect();
            item_factory
                .create_items(item_list)
                .await
                .expect("Unable to create character inventory items")
        };

        let character_items = character_items.await;

        let items = [&scene_items[..], &character_items[..]].concat();

        info!("Game generation completed for game with id: {}.", id);

        let game = Self {
            id,
            name,
            summary,
            cover_art,
            narrative,
            scenes,
            characters,
            items,
        };

        trace!("Generated full game: {:#?}", game);
        Ok(game)
    }
}
