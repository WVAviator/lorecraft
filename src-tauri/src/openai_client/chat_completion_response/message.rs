#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}