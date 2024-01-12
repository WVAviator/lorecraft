use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    Vivid,
    Natural,
}

impl Display for ImageStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageStyle::Vivid => write!(f, "vivid"),
            ImageStyle::Natural => write!(f, "natural"),
        }
    }
}
