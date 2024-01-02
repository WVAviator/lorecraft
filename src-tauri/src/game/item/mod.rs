use serde::{Deserialize, Serialize};

use super::image::Image;

pub mod item_factory;
pub mod item_input;
pub mod item_output;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: Image,
}

impl Item {
    pub fn new(id: String, name: String, description: String, image: Image) -> Self {
        Self {
            id,
            name,
            description,
            image,
        }
    }
}
