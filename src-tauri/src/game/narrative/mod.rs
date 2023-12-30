use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    openai_client::{
        chat_completion_model::ChatCompletionModel, chat_completion_request::ChatCompletionRequest,
        openai_client_error::OpenAIClientError, OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

use super::image::Image;

pub mod narrative_factory;
mod narrative_output;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Narrative {
    pages: Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub narrative: String,
    pub image: Image,
}

impl Narrative {
    pub fn new(pages: Vec<Page>) -> Self {
        Self { pages }
    }
}

impl Page {
    pub fn new(narrative: String, image: Image) -> Self {
        Self { narrative, image }
    }
}
