use serde::{Deserialize, Serialize};

use crate::config::content_setting::ContentSetting;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateNewGameRequest {
    pub prompt: String,
    pub text_content_setting: Option<ContentSetting>,
    pub image_content_setting: Option<ContentSetting>,
    pub temperature_setting: Option<String>,
}
