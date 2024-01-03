use crate::{
    application_state::ApplicationState,
    game_session::{game_session_error::GameSessionError, GameSession},
};

use self::start_game_request::StartGameRequest;

use log::error;
use tauri::State;
use tokio::sync::Mutex;

mod start_game_request;

#[tauri::command]
pub async fn start_game(
    request: StartGameRequest,
    state: State<'_, Mutex<ApplicationState>>,
) -> Result<(), GameSessionError> {
    let mut state = state.lock().await;
    let file_manager = &state.file_manager.as_ref();
    let file_manager = file_manager.ok_or(GameSessionError::ConfigError(String::from(
        "Unable to access file manager.",
    )))?;

    let openai_client = &state.openai_client.as_ref();
    let openai_client = openai_client.ok_or(GameSessionError::ConfigError(String::from(
        "Unable to access OpenAI client.",
    )))?;

    let game_session = GameSession::start_new(request.game_id, openai_client, file_manager)
        .await
        .map_err(|e| {
            error!("Unable to establish game session:\n{:?}", e);
            GameSessionError::SetupFailure(format!(
                "Error occurred while setting up game:\n{:?}",
                e
            ))
        })?;

    state.game_session = Some(game_session);

    Ok(())
}
