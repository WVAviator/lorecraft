#[derive(Debug, Clone)]
pub enum SessionRequest {
    ContinueProcessing,
    PlayerEntry(String),
    ReturnFunctionCall(String),
    CharacterTradeResponse(bool),
    CharacterEndInteraction,
}
