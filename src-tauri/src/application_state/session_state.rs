use crate::{game_session::GameSession, game_state::GameState};
use tokio::sync::{mpsc::Sender, Mutex};

pub struct SessionState {
    game_session: Option<GameSession>,
}

impl SessionState {
    pub fn new(state_update_tx: Mutex<Sender<GameState>>) -> Self {
        Self { game_session: None }
    }

    pub fn get_game_session(&mut self) -> Option<&mut GameSession> {
        self.game_session.as_mut()
    }

    pub async fn set_game_session(&mut self, mut game_session: GameSession) {
        self.game_session = Some(game_session);
    }
}
