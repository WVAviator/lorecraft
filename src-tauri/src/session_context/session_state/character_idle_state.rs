use anyhow::{anyhow, bail};
use log::{info, trace};
use openai_lib::{
    message::{CreateMessageRequest, MessageClient},
    OpenAIClient,
};

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct CharacterIdleState {}

impl CharacterIdleState {
    pub async fn process(
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::PlayerEntry(prompt) => {
                info!("Appending message to character thread.");
                let thread_id = game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction to read messages from."))?
                    .thread_id
                    .clone();
                let create_message_response = openai_client
                    .create_message(
                        CreateMessageRequest::builder().content(&prompt).build(),
                        &thread_id,
                    )
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "Failed to create new character message and add to thread: {:?}",
                            e
                        )
                    })?;

                info!("New message appended to character thread. Adding to UI state.");
                trace!("{:#?}", create_message_response);

                game_state
                    .character_interaction
                    .as_mut()
                    .ok_or(anyhow!("No character interaction to read messages from."))?
                    .add_message(&create_message_response.get_text_content());

                info!("Transitioning to pending run state.");
                Ok(SessionState::CharacterRunRequestState)
            }
            SessionRequest::CharacterEndInteraction => {
                return Ok(SessionState::CharacterEndInteractionState { summary: None })
            }
            _ => bail!(
                "Received invalid session request for idle state: {:?}. Expected PlayerEntry.",
                request
            ),
        }
    }
}
