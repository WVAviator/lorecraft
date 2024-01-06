use anyhow::{anyhow, bail};
use log::error;

use crate::{
    game_state::GameState, openai_client::OpenAIClient,
    session_context::session_request::SessionRequest,
};

use super::SessionState;

pub struct PendingRunState {}

impl PendingRunState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => {
                todo!("Process run.");
            }
            _ => bail!(
                    "Received invalid session request for pending run: {:?}. Expected ContinueProcessing.",
                    session_request
                ),
            
        }
    }
}
