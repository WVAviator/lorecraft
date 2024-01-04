use crate::{
    application_state::{session_state::SessionState, ApplicationState},
    openai_client,
};

use self::{
    game_prompt_error::GamePromptError, game_prompt_request::GamePromptRequest,
    game_prompt_response::GamePromptResponse,
};

use log::error;
use tauri::State;
use tokio::sync::Mutex;

mod game_prompt_error;
mod game_prompt_request;
mod game_prompt_response;

#[tauri::command]
pub async fn game_prompt(
    request: GamePromptRequest,
    application_state: State<'_, Mutex<ApplicationState>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<GamePromptResponse, GamePromptError> {
    // TODO: Moderation on request prompt

    let application_state = application_state.lock().await;
    let openai_client = &application_state.openai_client.as_ref();
    let openai_client = openai_client.ok_or(GamePromptError::new(
        "Unable to submit prompt: No OpenAI client.",
    ))?;

    let file_manager = &application_state.file_manager.as_ref();
    let file_manager = file_manager.ok_or(GamePromptError::new(
        "Unable to submit prompt: No file manager.",
    ))?;

    let mut session_state = session_state.lock().await;

    let game_session = session_state.get_game_session();
    let game_session = game_session.ok_or(GamePromptError::new(
        "Unable to submit prompt: No active game session.",
    ))?;

    let updated_game_state = game_session
        .process_game_prompt(&request.prompt, &openai_client, &file_manager)
        .await
        .map_err(|e| {
            error!("Unable to update game state with new prompt:\n{:?}", e);
            GamePromptError::new("An error occurred processing the request.")
        })?;

    Ok(GamePromptResponse::new(updated_game_state))
}
