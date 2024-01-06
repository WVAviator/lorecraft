use crate::{
    application_state::{session_state::SessionState, ApplicationState},
    game_session::{game_session_error::GameSessionError, GameSession},
};

use self::{start_game_request::StartGameRequest, start_game_response::StartGameResponse};

use log::error;
use tauri::State;
use tokio::sync::Mutex;

mod start_game_request;
mod start_game_response;

#[tauri::command]
pub async fn start_game(
    request: StartGameRequest,
    application_state: State<'_, Mutex<ApplicationState>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<StartGameResponse, GameSessionError> {
    let application_state = application_state.lock().await;
    let file_manager = &application_state.file_manager.as_ref();
    let file_manager = file_manager.ok_or(GameSessionError::ConfigError(String::from(
        "Unable to access file manager.",
    )))?;

    let openai_client = &application_state.openai_client.as_ref();
    let openai_client = openai_client.ok_or(GameSessionError::ConfigError(String::from(
        "Unable to access OpenAI client.",
    )))?;

    let mut session_state = session_state.lock().await;

    if let Some(game_session) = session_state.get_game_session() {
        let game_state = game_session.game_state.clone();
        return Ok(StartGameResponse::new(game_state));
    }

    let game_session = GameSession::start_new(request.game_id, openai_client, file_manager)
        .await
        .map_err(|e| {
            error!("Unable to establish game session:\n{:?}", e);
            GameSessionError::SetupFailure(format!(
                "Error occurred while setting up game:\n{:?}",
                e
            ))
        })?;

    let game_state = game_session.game_state.clone();
    session_state.set_game_session(game_session).await;

    Ok(StartGameResponse::new(game_state))
}
