use crate::application_state::ApplicationState;
use crate::commands::create_new_game::create_new_game_error::CreateNewGameError;
use crate::game::Game;
use crate::utils::string_utilities::StringUtilities;
use log::{error, info};

use tauri::State;
use tokio::sync::Mutex;

use self::create_new_game_request::CreateNewGameRequest;
use self::create_new_game_response::{CreateNewGameFailureResponse, CreateNewGameSuccessResponse};

mod create_new_game_error;
pub mod create_new_game_request;
mod create_new_game_response;

#[tauri::command]
pub async fn create_new_game(
    mut request: CreateNewGameRequest,
    state: State<'_, Mutex<ApplicationState>>,
) -> Result<CreateNewGameSuccessResponse, CreateNewGameFailureResponse> {
    request.prompt = {
        match request.prompt.as_str() {
            "" => String::from("choose any random unique game idea you can think of"),
            _ => StringUtilities::truncate(&request.prompt, 497),
        }
    };

    let game = {
        let state = state.lock().await;

        info!("Verifying app setup.");
        if let Err(e) = state.verify_setup() {
            error!("Failed to verify app state setup:\n{:?}", e);
            return Err(CreateNewGameFailureResponse::new(
                CreateNewGameError::SetupError(String::from("Setup not complete.")),
            ));
        }

        info!(
            "Creating new game from user prompt:\n{:?}.",
            &request.prompt
        );
        Game::create_new(request, &state).await.map_err(|e| {
            error!("Failed to generate new game:\n{:?}", e);
            CreateNewGameFailureResponse::new(CreateNewGameError::GameGenerationError(
                String::from("Failed to generate new game."),
            ))
        })?
    };

    info!(
        "Game with id '{}' created. Serializing and saving game to file.",
        game.id
    );
    let game_serialized = serde_json::to_string(&game).map_err(|e| {
        error!("Failed to serialize game:\n{:?}", e);
        CreateNewGameFailureResponse::new(CreateNewGameError::GameGenerationError(String::from(
            "Failed to serialize game.",
        )))
    })?;

    {
        let state = state.lock().await;
        if let Some(file_manager) = &state.file_manager {
            file_manager
                .write_to_file(format!("{}/game.json", game.id).as_str(), &game_serialized)
                .map_err(|e| {
                    error!("Failed to save game to local file system:\n{:?}", e);
                    CreateNewGameFailureResponse::new(CreateNewGameError::FileSystemError(
                        String::from("Failed to save game to local file system."),
                    ))
                })?;
        }
    }

    info!("Game '{}' saved and ready to play.", game.id);
    return Ok(CreateNewGameSuccessResponse::new(game));
}
