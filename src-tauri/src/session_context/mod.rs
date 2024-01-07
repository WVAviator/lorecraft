use log::error;

use crate::{game::Game, game_state::GameState, openai_client::OpenAIClient};

use self::{session_request::SessionRequest, session_state::SessionState};

pub mod session_request;
mod session_state;

#[derive(Debug)]
pub struct SessionContext {
    state: Option<SessionState>,
    openai_client: OpenAIClient,
    game: Game,
}

impl SessionContext {
    pub fn new(game: Game, openai_client: OpenAIClient) -> Self {
        let state = SessionState::PendingRunState;

        SessionContext {
            state: Some(state),
            openai_client,
            game,
        }
    }

    pub async fn process(&mut self, session_request: SessionRequest, game_state: &mut GameState) {
        self.process_state_change(session_request, game_state).await;

        loop {
            match self.state.as_ref() {
                Some(state) => {
                    if state.should_continue_processing() {
                        self.process_state_change(SessionRequest::ContinueProcessing, game_state)
                            .await;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    async fn process_state_change(
        &mut self,
        session_request: SessionRequest,
        game_state: &mut GameState,
    ) {
        let new_state = self
            .state
            .take()
            .unwrap_or(SessionState::IdleState)
            .process(session_request, &self.openai_client, game_state, &self.game)
            .await
            .unwrap_or_else(|e| {
                error!(
                    "Error occurred processing in session state: {:?}. Resetting state.",
                    e
                );

                // TODO: Add a state reset method to GameState that clears out anything stateful

                SessionState::IdleState
            });

        self.state = Some(new_state);
    }
}
