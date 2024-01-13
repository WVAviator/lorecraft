use crate::application_state::session_state::SessionState;

use self::{
    game_prompt_error::GamePromptError, game_prompt_request::GamePromptRequest,
    game_prompt_response::GamePromptResponse,
};

use log::{error, info};
use tauri::State;
use tokio::sync::Mutex;

mod game_prompt_error;
mod game_prompt_request;
mod game_prompt_response;

#[tauri::command]
pub async fn game_prompt(
    request: GamePromptRequest,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<GamePromptResponse, GamePromptError> {
    // TODO: Moderation on request prompt

    info!(
        "Received game prompt request from player '{}'",
        request.prompt
    );

    let mut session_state = session_state.lock().await;

    let game_session = session_state.get_game_session();
    let game_session = game_session.ok_or(GamePromptError::new(
        "Unable to submit prompt: No active game session.",
    ))?;

    info!("Loaded game session, processing new prompt.");
    let updated_game_state = game_session
        .receive_player_message(request.prompt)
        .await
        .map_err(|e| {
            error!("Unable to update game state with new prompt:\n{:?}", e);
            GamePromptError::new("An error occurred processing the request.")
        })?;

    info!("Received updated game state, returning to UI for rendering.");

    Ok(GamePromptResponse::new(updated_game_state))
}
