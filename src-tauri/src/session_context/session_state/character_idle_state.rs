use anyhow::{anyhow, bail};
use log::{info, trace};

use crate::{
    game_state::GameState,
    openai_client::{create_message::create_message_request::CreateMessageRequest, OpenAIClient},
    session_context::session_request::SessionRequest,
};

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
                    .create_message(CreateMessageRequest::new(&prompt), &thread_id)
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
                    .add_message(&create_message_response.content[0].text.value);

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
