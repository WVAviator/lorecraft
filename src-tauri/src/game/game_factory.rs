use anyhow::Context;
use openai_lib::OpenAIClient;

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting, file_manager::FileManager, utils::random::Random,
};

use super::game_metadata::GameMetadata;

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

        // let contents = serde_json::to_string(&game_metadata).unwrap();
        // file_manager
        //     .write_to_file(
        //         format!("{}/tmp/metadata.json", &game_id).as_str(),
        //         &contents,
        //     )
        //     .context("Error occurred attempting to save new game metadata to file.")?;

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
}
