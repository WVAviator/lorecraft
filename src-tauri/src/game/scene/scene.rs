use serde::{Deserialize, Serialize};

use crate::game::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub narrative: String,
    pub metadata: String,
    pub characters: Vec<String>,
    pub items: Vec<String>,
    pub image: Image,
}
