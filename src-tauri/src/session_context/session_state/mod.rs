use anyhow::Context;

use crate::{game_state::GameState, openai_client::OpenAIClient};

use self::{idle_state::IdleState, pending_run_state::PendingRunState};

use super::session_request::SessionRequest;

mod idle_state;
mod pending_run_state;

pub enum SessionState {
    IdleState,
    PendingRunState,
}

impl SessionState {
    pub async fn process(
        &self,
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match self {
            SessionState::IdleState => IdleState::process(request, openai_client, game_state)
                .await
                .context("Failed to process state change from IdleState."),
            SessionState::PendingRunState => {
                PendingRunState::process(request, openai_client, game_state)
                    .await
                    .context("Failed to process state change from PendingRunState")
            }
        }
    }

    pub fn should_continue_processing(&self) -> bool {
        match self {
            SessionState::IdleState => false,
            SessionState::PendingRunState => true,
        }
    }
}
