use anyhow::{anyhow, Context};
use futures::{StreamExt, TryStreamExt};
use log::{info, trace};
use openai_lib::{
    image::{CreateImageRequest, ImageQuality, ImageSize},
    model::image_model::ImageModel,
};
use serde::{Deserialize, Serialize};
use tokio::{join, sync::MutexGuard};

use crate::{
    application_state::ApplicationState,
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting,
    file_manager::FileManager,
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
    utils::random::Random,
};

use self::{image::Image, item::Item};

pub mod character;
pub mod chat_completion_factory;
pub mod game_factory;
pub mod game_metadata;
pub mod image;
pub mod item;
pub mod json_component;
pub mod narrative;
pub mod scene;
pub mod scene_detail;
pub mod scene_summary;
pub mod summary;

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
        request: CreateNewGameRequest,
        state: &MutexGuard<'_, ApplicationState>,
    ) -> Result<Self, anyhow::Error> {
        Err(anyhow!("Not implemented."))
        // let id = Random::generate_id();
        // info!("Generated new game id: {}.", id);

        // let openai_client = match &state.openai_client {
        //     Some(openai_client) => openai_client,
        //     None => {
        //         return Err(anyhow!(
        //             "Unable to access OpenAI client from application state."
        //         ))
        //     }
        // };

        // let file_manager = match &state.file_manager {
        //     Some(file_manager) => file_manager,
        //     None => {
        //         return Err(anyhow!(
        //             "Unable to access file manager from application state."
        //         ))
        //     }
        // };

        // let summary = async {
        //     info!("Generating game summary information.");
        //     state
        //         .send_update(String::from("Generating game summary information."))
        //         .await;
        //     Summary::generate(&openai_client, &request).await
        // };

        // let summary = summary.await.context("Failed to generate summary.")?;

        // let style_string = format!(
        //     "Use a style of {}. Use themes of {}.",
        //     &summary.art_style, &summary.art_theme
        // );

        // let image_factory = ImageFactory::new(&openai_client, &file_manager, &id, style_string);
        // let narrative_factory =
        //     NarrativeFactory::new(&openai_client, &summary.summary, &image_factory, &request);
        // let character_factory =
        //     CharacterFactory::new(&openai_client, &summary.summary, &image_factory, &request);
        // let item_factory =
        //     ItemFactory::new(&openai_client, &summary.summary, &image_factory, &request);

        // let name = summary.name.clone();
        // info!("Generated game: {}.", name);
        // trace!("Game summary: {:#?}", summary);

        // let cover_art = async {
        //     info!("Generating game cover art.");
        //     state
        //         .send_update(String::from("Generating game cover art."))
        //         .await;

        //     let quality = match &request.image_content_setting {
        //         Some(ContentSetting::Low) => ImageQuality::Standard,
        //         _ => ImageQuality::HD,
        //     };

        //     image_factory
        //         .try_generate_image(
        //             CreateImageRequest::builder()
        //                 .prompt(&summary.cover_art)
        //                 .model(ImageModel::DallE3)
        //                 .size(ImageSize::Size1792x1024)
        //                 .quality(quality)
        //                 .build(),
        //             "cover_art.png",
        //             3,
        //         )
        //         .await
        // };

        // let narrative = async {
        //     info!("Generating story narrative pages.");
        //     state
        //         .send_update(String::from("Generating story narrative pages."))
        //         .await;
        //     let narrative = narrative_factory.create().await;

        //     trace!("Generated narrative: {:#?}", narrative);
        //     narrative
        // };

        // let scene_details = async {
        //     info!("Generating scene summary information.");
        //     state
        //         .send_update(String::from("Generating scene summary information."))
        //         .await;
        //     let scene_summary = SceneSummary::generate(
        //         &summary.summary,
        //         &summary.win_condition,
        //         &openai_client,
        //         &request,
        //     )
        //     .await
        //     .map_err(|e| anyhow!("Failed to generate scene summary: {}", e))?;

        //     trace!("Generated scene summary: {:#?}", scene_summary);

        //     async {
        //         let mut futures = Vec::new();
        //         for summarized_scene in &scene_summary.scenes {
        //             let openai_ref = &openai_client;
        //             let summary_ref = &summary;
        //             let request_ref = &request;

        //             let future = async move {
        //                 SceneDetail::generate(
        //                     &summary_ref.summary,
        //                     &summarized_scene,
        //                     openai_ref,
        //                     request_ref,
        //                 )
        //                 .await
        //             };

        //             futures.push(future);
        //         }

        //         let stream = futures::stream::iter(futures).buffered(5);
        //         stream.try_collect::<Vec<_>>().await
        //     }
        //     .await
        // };

        // let (cover_art, narrative, scene_details) =
        //     tokio::join!(cover_art, narrative, scene_details);

        // let cover_art = cover_art?;
        // let narrative = narrative?;
        // let scene_details = scene_details?;

        // let scenes = async {
        //     let mut futures = Vec::new();
        //     for scene_detail in &scene_details {
        //         let image_factory_ref = &image_factory;
        //         let request_ref = &request;

        //         let future = async move {
        //             Scene::from_scene_detail(scene_detail, image_factory_ref, request_ref)
        //                 .await
        //                 .map_err(|e| anyhow!("Failed to create scene: {}", e))
        //         };

        //         futures.push(future);
        //     }

        //     state.send_update(String::from("Generating scenes.")).await;

        //     let stream = futures::stream::iter(futures).buffered(3);
        //     stream.try_collect::<Vec<_>>().await
        // };

        // let characters = async {
        //     let mut futures = Vec::new();
        //     let character_factory_ref = &character_factory;
        //     for scene_detail in &scene_details {
        //         let future =
        //             async move { character_factory_ref.from_scene_detail(scene_detail).await };

        //         futures.push(future);
        //     }

        //     let stream = futures::stream::iter(futures).buffered(3);
        //     stream.try_collect::<Vec<_>>().await
        // };

        // let scene_items = async {
        //     let item_list: Vec<String> = scene_details
        //         .iter()
        //         .map(|scene_detail| scene_detail.items.clone())
        //         .flatten()
        //         .collect();
        //     item_factory.create_items(item_list).await
        // };

        // let (scenes, characters, scene_items) = join!(scenes, characters, scene_items);

        // let scenes = scenes?;

        // let characters = characters?
        //     .into_iter()
        //     .flatten()
        //     .collect::<Vec<Character>>();

        // let scene_items = scene_items?;

        // let character_items = async {
        //     let item_list: Vec<String> = characters
        //         .iter()
        //         .map(|character| character.inventory.clone())
        //         .flatten()
        //         .collect();
        //     item_factory.create_items(item_list).await
        // };

        // let character_items = character_items.await?;

        // let items = [&scene_items[..], &character_items[..]].concat();

        // info!("Game generation completed for game with id: {}.", id);

        // let game = Self {
        //     id,
        //     name,
        //     summary,
        //     cover_art,
        //     narrative,
        //     scenes,
        //     characters,
        //     items,
        // };

        // trace!("Generated full game: {:#?}", game);
        // Ok(game)
    }

    pub fn load(game_id: &str, file_manager: &FileManager) -> Result<Self, anyhow::Error> {
        let filepath = format!("{}/game.json", game_id);
        let game_json = file_manager
            .read_from_file(&filepath)
            .with_context(|| format!("Unable to read from file at '{}'.", &filepath))?;
        let game = serde_json::from_str::<Game>(&game_json)
            .context("Unable to parse game from json file.")?;

        Ok(game)
    }
}
