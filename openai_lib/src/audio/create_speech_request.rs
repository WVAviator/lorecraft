use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct CreateSpeechRequest {
    model: TTSModel,
    #[builder(setter(into))]
    input: String,
    voice: TTSVoice,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    response_format: Option<TTSResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    speed: Option<f32>,
}

impl CreateSpeechRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TTSModel {
    #[serde(rename = "tts-1")]
    TTS1,
    #[serde(rename = "tts-1-hd")]
    TTS1_HD,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TTSVoice {
    Alloy,
    Echo,
    Fable,
    Onyx,
    Nova,
    Shimmer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TTSResponseFormat {
    Mp3,
    Opus,
    Aac,
    Flac,
}
