use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    Standard,
    HD,
}

impl Display for ImageQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageQuality::Standard => write!(f, "standard"),
            ImageQuality::HD => write!(f, "hd"),
        }
    }
}
