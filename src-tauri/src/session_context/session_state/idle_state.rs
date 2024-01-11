use anyhow::{anyhow, bail};
use log::{error, info, trace};

use crate::{
    game_state::GameState,
    openai_client::{create_message::create_message_request::CreateMessageRequest, OpenAIClient},
    session_context::session_request::SessionRequest,
};

use super::SessionState;

pub struct IdleState {}

impl IdleState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::PlayerEntry(prompt) => {
                info!("Appending message to game thread.");
                let create_message_response = openai_client
                    .create_message(CreateMessageRequest::new(&prompt), &game_state.thread_id)
                    .await
                    .map_err(|e| {
                        anyhow!("Failed to create new message and add to thread: {:?}", e)
                    })?;

                info!("New message appended to game thread. Adding to UI state.");
                trace!("{:#?}", create_message_response);

                game_state.add_player_message(&create_message_response.content[0].text.value);

                info!("Transitioning to pending run state.");
                Ok(SessionState::PendingRunState)
            }
            _ => bail!(
                "Received invalid session request for idle state: {:?}. Expected PlayerEntry.",
                session_request
            ),
        }
    }
}
