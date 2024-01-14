use serde::{Deserialize, Serialize};

use crate::config::content_setting::ContentSetting;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateNewGameRequest {
    pub prompt: String,
    pub text_content_setting: Option<ContentSetting>,
    pub image_content_setting: Option<ContentSetting>,
    pub temperature_setting: Option<String>,
}

impl CreateNewGameRequest {
    pub fn get_temperature(&self) -> f32 {
        self.temperature_setting
            .clone()
            .unwrap_or(String::from("1.0"))
            .parse()
            .unwrap_or(1.0)
    }
}
