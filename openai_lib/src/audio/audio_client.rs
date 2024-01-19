use bytes::Bytes;

use crate::Error;

use super::create_speech_request::CreateSpeechRequest;

#[trait_variant::make(AudioClient: Send)]
pub trait LocalAudioClient {
    async fn create_speech(
        &self,
        create_speech_request: CreateSpeechRequest,
    ) -> Result<Bytes, Error>;
}
