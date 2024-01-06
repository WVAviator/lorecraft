use log::error;

use crate::{
    file_manager::FileManager, game::Game, game_state::GameState, openai_client::OpenAIClient,
};

use self::{session_request::SessionRequest, session_state::SessionState};

pub mod session_request;
mod session_state;

#[derive(Debug)]
pub struct SessionContext<'a> {
    state: Option<SessionState>,
    openai_client: &'a OpenAIClient,
    game: Game,
}

impl<'a> SessionContext<'a> {
    pub fn new(game: Game, openai_client: &'a OpenAIClient) -> Self {
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
                        self.process_state_change(SessionRequest::ContinueProcessing, game_state);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        // while self
        //     .state
        //     .unwrap_or(SessionState::IdleState)
        //     .should_continue_processing()
        // {
        //     self.process_state_change(SessionRequest::ContinueProcessing, game_state)
        //         .await;
        // }
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
            .process(session_request, self.openai_client, game_state)
            .await
            .unwrap_or_else(|e| {
                error!(
                    "Error occurred processing in session state: {:?}. Resetting state.",
                    e
                );
                SessionState::IdleState
            });
        self.state = Some(new_state);
    }
}
