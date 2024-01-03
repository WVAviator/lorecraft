use crate::{application_state::ApplicationState, openai_client};

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
    state: State<'_, Mutex<ApplicationState>>,
) -> Result<GamePromptResponse, GamePromptError> {
    // TODO: Moderation on request prompt
    let mut game_session = {
        let mut state = state.lock().await;
        let game_session = state.game_session.as_mut();
        let game_session = game_session.ok_or(GamePromptError::new(
            "Unable to submit prompt: No active game session.",
        ))?;
        game_session.clone()
    };

    let updated_game_session = {
        let state = state.lock().await;
        let openai_client = &state.openai_client.as_ref();
        let openai_client = openai_client.ok_or(GamePromptError::new(
            "Unable to submit prompt: No OpenAI client.",
        ))?;

        let file_manager = &state.file_manager.as_ref();
        let file_manager = file_manager.ok_or(GamePromptError::new(
            "Unable to submit prompt: No file manager.",
        ))?;

        let updated_game_state = game_session
            .process_game_prompt(&request.prompt, &openai_client, &file_manager)
            .await
            .map_err(|e| {
                error!("Unable to update game state with new prompt:\n{:?}", e);
                GamePromptError::new("An error occurred processing the request.")
            })?;

        game_session
    };

    let mut state = state.lock().await;
    state.game_session = Some(updated_game_session);
    let game_state = &state.game_session.as_ref().unwrap().game_state;

    Ok(GamePromptResponse::new(game_state))
}
