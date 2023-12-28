use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub narrative: String,
    pub image: String,
}
