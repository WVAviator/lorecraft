pub mod game_session_error;

use anyhow::{anyhow, Context};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    file_manager::{self, FileManager},
    game::Game,
    game_session::game_session_error::GameSessionError,
    openai_client::{
        assisstant_api::assisstant_create_request::AssisstantCreateRequest,
        chat_completion::chat_completion_model::ChatCompletionModel, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
    utils::random::Random,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSession {
    pub id: String,
    pub game: Game,
    pub narrator_assisstant_id: String,
    pub thread_id: String,
}

impl GameSession {
    pub async fn start_new(
        game_id: String,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        info!("Starting new game session for game id {}.", &game_id);
        let filepath = format!("{}/game.json", game_id);
        let game_json = file_manager
            .read_from_file(&filepath)
            .with_context(|| format!("Unable to read from file at '{}'.", &filepath))?;
        let game = serde_json::from_str::<Game>(&game_json)
            .context("Unable to parse game from json file.")?;

        let summary_text = format!("Game Summary:\n{}", &game.summary.summary);
        let scene_list_text = format!(
            "Scene List:\n[{}]",
            game.scenes
                .iter()
                .map(|scene| format!("{},", scene.name))
                .collect::<String>()
        );

        let instructions = PromptBuilder::new()
            .add_prompt("./prompts/narrator/main.txt")
            .add_plain_text(&summary_text)
            .add_plain_text(&scene_list_text)
            .build();

        let assisstant_response = openai_client
            .create_assisstant(AssisstantCreateRequest::new(
                instructions,
                game_id.to_string(),
                ChatCompletionModel::Gpt3_5Turbo1106,
            ))
            .await
            .expect("Failed to create assisstant.");

        let narrator_assisstant_id = assisstant_response.id;

        let thread_response = openai_client.create_thread().await.map_err(|e| {
            error!("Failed to create thread for Assisstant API:\n{:?}", e);
            anyhow!("Failed to start thread.")
        })?;

        let thread_id = thread_response.id;

        let id = Random::generate_id();

        let game_session = GameSession {
            id,
            game,
            narrator_assisstant_id,
            thread_id,
        };

        game_session.save(file_manager)?;

        Ok(game_session)
    }

    pub fn save(&self, file_manager: &FileManager) -> Result<(), anyhow::Error> {
        let filepath = format!("save_data/{}/{}.json", self.game.id, self.id);
        let json =
            serde_json::to_string(&self).context("Error serializing game session to json.")?;
        file_manager
            .write_to_file(&filepath, &json)
            .context("Error writing game session to file.")?;

        Ok(())
    }
}
