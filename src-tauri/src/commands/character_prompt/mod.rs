use log::{error, info};
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    application_state::{session_state::SessionState, ApplicationState},
    file_manager, openai_client,
};

use self::{
    character_prompt_error::CharacterPromptError, character_prompt_request::CharacterPromptRequest,
    character_prompt_response::CharacterPromptResponse,
};

pub mod character_prompt_error;
pub mod character_prompt_request;
pub mod character_prompt_response;

#[tauri::command]
pub async fn character_prompt(
    request: CharacterPromptRequest,
    application_state: State<'_, Mutex<ApplicationState>>,
    session_state: State<'_, Mutex<SessionState>>,
) -> Result<CharacterPromptResponse, CharacterPromptError> {
    info!(
        "Character prompt command triggered with request: {:?}",
        &request
    );

    let application_state = application_state.lock().await;

    let openai_client = &application_state.openai_client.as_ref();
    let openai_client =
        openai_client.ok_or(CharacterPromptError::new("Unable to access OpenAI client."))?;

    let file_manager = &application_state.file_manager.as_ref();
    let file_manager =
        file_manager.ok_or(CharacterPromptError::new("Unable to access file manager."))?;

    let mut session_state = session_state.lock().await; // TODO: Getting stuck here, need main session thread to release lock.

    let game_session = session_state
        .get_game_session()
        .ok_or(CharacterPromptError::new("Unable to access game session."))?;

    info!("Loaded client, file manager, and game session for character prompt processing.");

    let updated_game_state = game_session
        .process_character_prompt(&request, &openai_client, &file_manager)
        .await
        .map_err(|e| {
            error!(
                "Error occurred attempting to process character prompt:\n{:?}",
                e
            );
            CharacterPromptError::new("Error occurred attempting to process character prompt.")
        })?;

    info!("Character prompt processed, returning updated state.");

    Ok(CharacterPromptResponse::new(updated_game_state.clone()))
}
