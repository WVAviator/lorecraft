use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Size256x256,
    #[serde(rename = "512x512")]
    Size512x512,
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "1792x1024")]
    Size1792x1024,
    #[serde(rename = "1024x1792")]
    Size1024x1792,
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageSize::Size256x256 => write!(f, "256x256"),
            ImageSize::Size512x512 => write!(f, "512x512"),
            ImageSize::Size1024x1024 => write!(f, "1024x1024"),
            ImageSize::Size1792x1024 => write!(f, "1792x1024"),
            ImageSize::Size1024x1792 => write!(f, "1024x1792"),
        }
    }
}
