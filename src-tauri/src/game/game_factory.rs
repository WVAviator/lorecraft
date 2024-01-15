use anyhow::Context;
use openai_lib::OpenAIClient;

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    file_manager::FileManager, game::summary::SummaryFactory, utils::random::Random,
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
        let summary_factory =
            SummaryFactory::new(&self.openai_client, &self.file_manager, &self.game_metadata);

        let summary = summary_factory.try_create(3).await?;

        todo!();
    }
}