use super::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub message: Vec<Message>,
}