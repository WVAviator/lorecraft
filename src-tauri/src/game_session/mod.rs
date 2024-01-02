pub mod game_session_error;

use anyhow::Context;
use log::info;

use crate::{
    file_manager::FileManager,
    game::Game,
    openai_client::{
        assisstant_api::assisstant_create_request::AssisstantCreateRequest,
        chat_completion::chat_completion_model::ChatCompletionModel, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

pub struct GameSession {
    pub game: Game,
    pub narrator_assisstant_id: String,
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

        let response = openai_client
            .create_assisstant(AssisstantCreateRequest::new(
                instructions,
                game_id.to_string(),
                ChatCompletionModel::Gpt3_5Turbo1106,
            ))
            .await
            .expect("Failed to create assisstant.");

        let narrator_assisstant_id = response.id;

        Ok(GameSession {
            game,
            narrator_assisstant_id,
        })
    }
}
