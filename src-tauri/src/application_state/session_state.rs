use crate::game_session::GameSession;

pub struct SessionState {
    pub game_session: Option<GameSession>,
}

impl SessionState {
    pub fn new() -> Self {
        Self { game_session: None }
    }

    pub fn get_game_session(&mut self) -> Option<&mut GameSession> {
        self.game_session.as_mut()
    }

    pub fn set_game_session(&mut self, game_session: GameSession) {
        self.game_session = Some(game_session);
    }
}
