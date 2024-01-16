use anyhow::Context;
use openai_lib::{model::image_model::ImageModel, OpenAIClient};

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    file_manager::FileManager,
    game::{
        chat_completion_factory::ChatCompletionFactory, image::image_factory::ImageFactory,
        narrative::Narrative, summary::Summary,
    },
    utils::random::Random,
};

use super::{game_metadata::GameMetadata, Game};

pub struct GameFactory {
    game_id: String,
    game_metadata: GameMetadata,
    openai_client: OpenAIClient,
    file_manager: FileManager,
}

impl GameFactory {
    pub fn new(
        request: CreateNewGameRequest,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        let game_id = Random::generate_id();

        let game_metadata = GameMetadata::from_request(&game_id, request);

        file_manager
            .write_json(format!("{}/tmp/metadata.json", &game_id), &game_metadata)
            .context("Error occurred attempting to save new game metadata to file.")?;

        Ok(GameFactory {
            game_id,
            game_metadata,
            openai_client: openai_client.clone(),
            file_manager: file_manager.clone(),
        })
    }

    pub fn resume(
        game_id: impl Into<String>,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        let game_id = game_id.into();

        let game_metadata = file_manager
            .read_json::<GameMetadata>(format!("{}/tmp/metadata.json", &game_id))
            .context("Error occurred attempting to read game metadata json file.")?;

        Ok(GameFactory {
            game_id,
            game_metadata,
            openai_client: openai_client.clone(),
            file_manager: file_manager.clone(),
        })
    }

    pub async fn create(&self) -> Result<Game, anyhow::Error> {
        let chat_completion_factory = ChatCompletionFactory::new(
            &self.openai_client,
            &self.file_manager,
            &self.game_metadata,
        );

        let mut summary =
            Summary::create(&chat_completion_factory, &self.game_metadata.prompt).await?;

        let image_style = format!(
            "In the style of {}\nWith themes of {}",
            &summary.art_style, &summary.art_theme
        );

        let image_factory = ImageFactory::new(
            &self.openai_client,
            &self.file_manager,
            &self.game_metadata,
            image_style,
        );

        summary
            .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
            .await?;

        let mut narrative = Narrative::create(&summary, &chat_completion_factory).await?;

        narrative
            .generate_images(&image_factory, &self.game_metadata, &self.file_manager)
            .await?;

        // let narrative_factory = NarrativeFactory::new(
        //     &self.openai_client,
        //     &self.file_manager,
        //     &image_factory,
        //     &self.game_metadata,
        // );
        // let scene_summary_factory = SceneSummaryFactory::new(
        //     &self.openai_client,
        //     &self.file_manager,
        //     &self.game_metadata,
        //     &image_factory,
        // );

        // let summary = summary_factory.try_create(3).await?;

        // image_factory.add_style(format!(
        //     "In the style of {}\nWith themes of {}",
        //     &summary.art_style, &summary.art_theme
        // ));

        // let narrative = narrative_factory.try_create(&summary, 3);
        // let scene_summary = scene_summary_factory.try_create(&summary, 3);

        // let (narrative, scene_summary) = tokio::join!(narrative, scene_summary);

        // let narrative = narrative?;
        // let scene_summary = scene_summary?;

        todo!();
    }
}
