use std::sync::Arc;

use anyhow::Context;
use log::{error, info};
use openai_lib::OpenAIClient;
use tokio::{
    join,
    sync::{mpsc::Sender, Mutex},
};

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    file_manager::FileManager,
    game::{
        audio::AudioFactory, character::Character, chat_completion_factory::ChatCompletionFactory,
        image::image_factory::ImageFactory, item::Item, narrative::Narrative, scene::Scene,
        scene_summary::SceneSummary, selection_factory::SelectionFactory, summary::Summary,
        title_music::TitleMusic,
    },
    utils::random::Random,
};

use super::{
    game_generation_update::GameGenerationUpdate, game_metadata::GameMetadata,
    image::image_multiprocessor::ImageMultiprocessor, Game,
};

pub struct GameFactory {
    game_id: String,
    game_metadata: GameMetadata,
    openai_client: OpenAIClient,
    file_manager: FileManager,
    start_time: std::time::Instant,
    updates_tx: Arc<Mutex<Sender<GameGenerationUpdate>>>,
}

impl GameFactory {
    pub fn new(
        request: CreateNewGameRequest,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        updates_tx: &Arc<Mutex<Sender<GameGenerationUpdate>>>,
    ) -> Result<Self, anyhow::Error> {
        let game_id = Random::generate_id();
        info!("Creating new game with id: {}", &game_id);

        let game_metadata = GameMetadata::from_request(&game_id, request);
        info!("Extracted metadata from request: {:?}", &game_metadata);

        file_manager
            .write_json(format!("{}/tmp/metadata.json", &game_id), &game_metadata)
            .context("Error occurred attempting to save new game metadata to file.")?;
        info!("Saved metadata to file.");

        Ok(GameFactory {
            game_id,
            game_metadata,
            openai_client: openai_client.clone(),
            file_manager: file_manager.clone(),
            start_time: std::time::Instant::now(),
            updates_tx: updates_tx.clone(),
        })
    }

    pub fn resume(
        game_id: impl Into<String>,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        updates_tx: &Arc<Mutex<Sender<GameGenerationUpdate>>>,
    ) -> Result<Self, anyhow::Error> {
        let game_id = game_id.into();
        info!("Resuming building game with id: {}", &game_id);

        let game_metadata = file_manager
            .read_json::<GameMetadata>(format!("{}/tmp/metadata.json", &game_id))
            .context("Error occurred attempting to read game metadata json file.")?;
        info!("Loaded existing metadata from file: {:?}", &game_metadata);

        Ok(GameFactory {
            game_id,
            game_metadata,
            openai_client: openai_client.clone(),
            file_manager: file_manager.clone(),
            start_time: std::time::Instant::now(),
            updates_tx: updates_tx.clone(),
        })
    }

    pub async fn create(&self) -> Result<Game, anyhow::Error> {
        info!("Starting game creation process for game {}.", &self.game_id);
        self.send_update("Starting game creation process").await;

        let chat_completion_factory = ChatCompletionFactory::new(
            &self.openai_client,
            &self.file_manager,
            &self.game_metadata,
        );
        info!("Initialized chat completion factory.");

        let selection_factory =
            SelectionFactory::new(&self.openai_client, &self.file_manager, &self.game_metadata);
        info!("Initialized selection factory.");

        let audio_factory =
            AudioFactory::new(&self.openai_client, &self.file_manager, &self.game_metadata);
        info!("Initialized audio factory.");

        let mut summary =
            Summary::create(&chat_completion_factory, &self.game_metadata.prompt).await?;
        self.send_update("Generated game name, style, and summary.")
            .await;

        let image_style = format!(
            "In the style of {}\nWith themes of {}",
            &summary.art_style, &summary.art_theme
        );
        info!(
            "Extracted image style phrasing from summary: '{}'",
            &image_style
        );

        let image_factory = ImageFactory::new(
            &self.openai_client,
            &self.file_manager,
            &self.game_metadata,
            image_style,
        );
        info!("Initialized image factory.");

        summary
            .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
            .await?;
        self.send_update("Generated cover art for game.").await;

        let title_music = TitleMusic::create(&summary, &selection_factory).await?;
        self.send_update("Selected main menu music.").await;

        let narrative = async {
            let mut narrative = Narrative::create(&summary, &chat_completion_factory).await?;
            self.send_update("Generated opening cutscene text.").await;

            narrative
                .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
                .await?;
            self.send_update("Generated opening cutscene images.").await;

            narrative.generate_audio(&audio_factory).await?;
            self.send_update("Generated speech audio for opening cutscene.")
                .await;

            Ok(narrative) as Result<Narrative, anyhow::Error>
        };

        let scenes = async {
            let scene_summary = SceneSummary::create(&summary, &chat_completion_factory).await?;
            self.send_update("Generated summaries for each scene.")
                .await;

            let mut scenes: Vec<Scene> =
                Scene::create_all(&summary, &scene_summary, &chat_completion_factory).await?;
            self.send_update("Generated detailed information for each scene.")
                .await;

            scenes
                .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
                .await?;
            self.send_update("Generated all scene images.").await;

            Ok(scenes) as Result<Vec<Scene>, anyhow::Error>
        };

        let (narrative, scenes) = futures::join!(narrative, scenes);
        let (narrative, scenes) = (narrative?, scenes?);

        let scene_items = async {
            let mut scene_items =
                Item::create_from_scenes(&summary, &scenes, &chat_completion_factory).await?;
            self.send_update("Generated items found in scenes.").await;

            scene_items
                .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
                .await?;
            self.send_update("Finished generating scene item images.")
                .await;

            Ok(scene_items) as Result<Vec<Item>, anyhow::Error>
        };

        let character_future = async {
            let characters = self
                .generate_character_detail(&summary, &scenes, &chat_completion_factory)
                .await?;
            let characters = self
                .generate_character_images(characters, &image_factory)
                .await?;
            let character_items = self
                .generate_character_items(
                    &summary,
                    &characters,
                    &chat_completion_factory,
                    &image_factory,
                )
                .await?;

            Ok((characters, character_items)) as Result<(Vec<Character>, Vec<Item>), anyhow::Error>
        };

        let (scene_items, character_future) = join!(scene_items, character_future);
        let (scene_items, (characters, character_items)) = (scene_items?, character_future?);

        let items = [&scene_items[..], &character_items[..]].concat();

        let id = self.game_metadata.game_id.clone();
        let name = summary.name.clone();
        let cover_art = summary.cover_art.clone();

        let game = Game {
            id,
            name,
            cover_art,
            summary,
            narrative,
            scenes,
            characters,
            items,
            title_music,
        };

        info!("Game creation process complete.");

        let elapsed_time = self.start_time.elapsed().as_secs();
        let elapsed_time = format!("{:02}:{:02}", elapsed_time / 60, elapsed_time % 60);

        self.send_update(format!("Finished generating game in {}.", elapsed_time))
            .await;

        let file_path = format!("{}/game.json", &self.game_metadata.game_id);

        self.file_manager
            .write_json(&file_path, &game)
            .context("Error occurred attempting to write game json file.")?;

        info!("Game saved to file: {}", &file_path);

        Ok(game)
    }

    async fn generate_character_detail(
        &self,
        summary: &Summary,
        scenes: &Vec<Scene>,
        chat_completion_factory: &ChatCompletionFactory<'_>,
    ) -> Result<Vec<Character>, anyhow::Error> {
        let characters =
            Character::create_from_scenes(summary, scenes, &chat_completion_factory).await?;
        self.send_update("Generated detailed profiles for each character.")
            .await;
        Ok(characters)
    }

    async fn generate_character_images(
        &self,
        mut characters: Vec<Character>,
        image_factory: &ImageFactory<'_>,
    ) -> Result<Vec<Character>, anyhow::Error> {
        characters
            .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
            .await?;
        self.send_update("Generated images for characters.").await;

        Ok(characters)
    }

    async fn generate_character_items(
        &self,
        summary: &Summary,
        characters: &Vec<Character>,
        chat_completion_factory: &ChatCompletionFactory<'_>,
        image_factory: &ImageFactory<'_>,
    ) -> Result<Vec<Item>, anyhow::Error> {
        let mut items =
            Item::create_from_characters(summary, characters, &chat_completion_factory).await?;
        self.send_update("Generated items found in character inventories.")
            .await;

        items
            .generate_images(image_factory, &self.game_metadata, &self.file_manager)
            .await?;
        self.send_update("Generated images for character items.")
            .await;

        Ok(items)
    }

    pub async fn send_update(&self, update: impl Into<String>) {
        let update = update.into();
        info!("{}", &update);
        let updates_tx = self.updates_tx.lock().await;
        if let Err(_) = updates_tx
            .send(GameGenerationUpdate::new(&self.game_id, update))
            .await
        {
            error!("Failed to send update to UI.");
        }
    }
}
