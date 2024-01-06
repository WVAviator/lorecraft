use crate::{game_session::GameSession, game_state::GameState};
use tokio::sync::{mpsc::Sender, Mutex};

pub struct SessionState {
    game_session: Option<GameSession>,
    state_update_tx: Mutex<Sender<GameState>>,
}

impl SessionState {
    pub fn new(state_update_tx: Mutex<Sender<GameState>>) -> Self {
        Self {
            game_session: None,
            state_update_tx,
        }
    }

    pub fn get_game_session(&mut self) -> Option<&mut GameSession> {
        self.game_session.as_mut()
    }

    pub async fn set_game_session(&mut self, mut game_session: GameSession) {
        let state_update_tx = self.state_update_tx.lock().await.clone();
        game_session.add_state_tx(state_update_tx);
        self.game_session = Some(game_session);
    }
}
