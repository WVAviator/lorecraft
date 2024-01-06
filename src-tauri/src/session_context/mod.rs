use log::error;

use crate::{game_state::GameState, openai_client::OpenAIClient};

use self::{session_request::SessionRequest, session_state::SessionState};

pub mod session_request;
mod session_state;

pub struct SessionContext {
    state: SessionState,
}

impl SessionContext {
    pub fn new() -> Self {
        let state = SessionState::PendingRunState;

        SessionContext { state }
    }

    pub async fn process(
        &mut self,
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) {
        self.process_state_change(session_request, openai_client, game_state)
            .await;

        while self.state.should_continue_processing() {
            self.process_state_change(
                SessionRequest::ContinueProcessing,
                openai_client,
                game_state,
            )
            .await;
        }
    }

    async fn process_state_change(
        &mut self,
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) {
        self.state = self
            .state
            .process(session_request, openai_client, game_state)
            .await
            .unwrap_or_else(|e| {
                error!(
                    "Error occurred processing in session state: {:?}. Resetting state.",
                    e
                );
                SessionState::IdleState
            });
    }
}
