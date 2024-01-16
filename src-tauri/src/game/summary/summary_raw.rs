use serde::{Deserialize, Serialize};

use crate::{
    game::{
        chat_completion_factory::{ChatCompletionFactory, ChatCompletionFactoryArgs},
        image::image_factory::ImageFactory,
    },
    prompt_builder::PromptBuilder,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryRaw {
    pub name: String,
    pub description: String,
    pub art_style: String,
    pub art_theme: String,
    pub cover_art: String,
    pub summary: String,
    pub win_condition: String,
}

impl SummaryRaw {
    pub async fn create(
        factory: &ChatCompletionFactory<'_>,
        user_message: &str,
    ) -> Result<Self, anyhow::Error> {
        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/summary/main.txt")
            .add_plain_text("Example Input: Make a game about mystical forests and ancient ruins")
            .add_example_output("./prompts/summary/example1.json")
            .add_plain_text("Example Input: I want to wake up on an abandoned spaceship infested with alien life")
            .add_example_output("./prompts/summary/example2.json")
            .build();

        let user_message = String::from(user_message);

        factory
            .try_create(
                ChatCompletionFactoryArgs::builder()
                    .name("Summary")
                    .system_message(system_message)
                    .user_message(user_message)
                    .file_name("summary.json")
                    .build(),
            )
            .await
    }

    // pub async fn populate_images(
    //     self,
    //     image_factory: &ImageFactory<'_>,
    // ) -> Result<Summary, anyhow::Error> {
    // }
}
