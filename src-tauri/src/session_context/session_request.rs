#[derive(Debug, Clone)]
pub enum SessionRequest {
    ContinueProcessing,
    PlayerEntry(String),
    CharacterTradeResponse(bool),
    CharacterEndInteraction,
}
