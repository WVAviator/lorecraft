use anyhow::{anyhow, bail};
use log::{info, trace};
use openai_lib::{
    message::{CreateMessageRequest, MessageClient},
    OpenAIClient,
};

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

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
                let message = openai_client
                    .create_message(
                        CreateMessageRequest::builder().content(&prompt).build(),
                        &game_state.thread_id,
                    )
                    .await
                    .map_err(|e| {
                        anyhow!("Failed to create new message and add to thread: {:?}", e)
                    })?;

                info!("New message appended to game thread. Adding to UI state.");
                trace!("{:#?}", message);

                game_state.add_player_message(message.get_text_content().as_str());

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
