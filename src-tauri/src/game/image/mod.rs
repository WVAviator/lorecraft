use serde::{Deserialize, Serialize};

pub mod image_factory;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub src: String,
    pub alt: String,
}
