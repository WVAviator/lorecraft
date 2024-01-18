use crate::application_state::ApplicationState;
use crate::commands::create_new_game::create_new_game_error::CreateNewGameError;
use crate::game::GameFactory;
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
    application_state: State<'_, Mutex<ApplicationState>>,
) -> Result<CreateNewGameSuccessResponse, CreateNewGameFailureResponse> {
    request.prompt = {
        match request.prompt.as_str() {
            "" => String::from("choose any random unique game idea you can think of"),
            _ => StringUtilities::truncate(&request.prompt, 497),
        }
    };

    let game_factory = {
        let application_state = application_state.lock().await;

        let file_manager = &application_state.file_manager.as_ref();
        let file_manager = file_manager.ok_or(CreateNewGameFailureResponse::new(
            CreateNewGameError::SetupError(String::from("Unable to access file manager.")),
        ))?;

        let openai_client = &application_state.openai_client.as_ref();
        let openai_client = openai_client.ok_or(CreateNewGameFailureResponse::new(
            CreateNewGameError::SetupError(String::from("Unable to access OpenAI client.")),
        ))?;

        let updates_tx = &application_state.updates_tx;

        let game_factory = match request.resume_previous {
            Some(game_id) => {
                GameFactory::resume(game_id, &openai_client, &file_manager, &updates_tx)
            }
            None => GameFactory::new(request, &openai_client, &file_manager, &updates_tx),
        };

        let game_factory = game_factory.map_err(|e| {
            error!("Unable to establish game factory:\n{:?}", e);
            CreateNewGameFailureResponse::new(CreateNewGameError::SetupError(String::from(
                "Unable to create factory for game construction.",
            )))
        })?;

        game_factory
    };

    let game = game_factory.create().await.map_err(|e| {
        error!("Unable to create game:\n{:?}", e);
        CreateNewGameFailureResponse::new(CreateNewGameError::SetupError(String::from(
            "Unable to create game.",
        )))
    })?;

    info!("Game '{}' saved and ready to play.", game.id);
    return Ok(CreateNewGameSuccessResponse::new(game));
}
