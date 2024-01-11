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

    let mut session_state = session_state.lock().await;

    let game_session = session_state
        .get_game_session()
        .ok_or(CharacterPromptError::new("Unable to access game session."))?;

    match request {
        CharacterPromptRequest {
            message: Some(message),
            ..
        } => {
            let updated_game_state =
                game_session
                    .receive_player_message(message)
                    .await
                    .map_err(|e| {
                        error!("unable to update game state with new prompt:\n{:?}", e);
                        CharacterPromptError::new("an error occurred processing the request.")
                    })?;

            let response = CharacterPromptResponse::new(updated_game_state);
            Ok(response)
        }
        CharacterPromptRequest {
            trade_accept: Some(accepted),
            ..
        } => {
            let updated_game_state = game_session
                .receive_trade_response(accepted)
                .await
                .map_err(|e| {
                    error!("unable to update game state with new prompt:\n{:?}", e);
                    CharacterPromptError::new("an error occurred processing the request.")
                })?;

            let response = CharacterPromptResponse::new(updated_game_state);
            Ok(response)
        }
        CharacterPromptRequest {
            end_conversation: Some(_),
            ..
        } => {
            let updated_game_state =
                game_session
                    .end_character_interaction()
                    .await
                    .map_err(|e| {
                        error!("unable to update game state with new prompt:\n{:?}", e);
                        CharacterPromptError::new("an error occurred processing the request.")
                    })?;

            let response = CharacterPromptResponse::new(updated_game_state);
            Ok(response)
        }
        _ => {
            return Err(CharacterPromptError::new(
                "Invalid character prompt request.",
            ))
        }
    }
}
