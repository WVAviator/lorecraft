use serde::{Deserialize, Serialize};

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameMetadata {
    game_id: String,
    prompt: String,
    text_content_setting: ContentSetting,
    image_content_setting: ContentSetting,
    temperature_setting: f32,
}

impl GameMetadata {
    pub fn from_request(game_id: impl Into<String>, request: CreateNewGameRequest) -> Self {
        let prompt = request.prompt.clone();

        let text_content_setting = request
            .text_content_setting
            .clone()
            .unwrap_or(ContentSetting::Moderate);

        let image_content_setting = request
            .image_content_setting
            .clone()
            .unwrap_or(ContentSetting::Moderate);

        let temperature_setting = request.get_temperature();

        GameMetadata {
            game_id: game_id.into(),
            prompt,
            text_content_setting,
            image_content_setting,
            temperature_setting,
        }
    }
}
