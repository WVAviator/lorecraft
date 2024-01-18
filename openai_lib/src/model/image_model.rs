use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ImageModel {
    #[serde(rename = "dall-e-2")]
    DallE2,
    #[serde(rename = "dall-e-3")]
    DallE3,
}

impl Display for ImageModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageModel::DallE2 => write!(f, "dall-e-2"),
            ImageModel::DallE3 => write!(f, "dall-e-3"),
        }
    }
}
