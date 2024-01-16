use futures::{Future, StreamExt, TryStreamExt};
use log::info;
use serde::{Deserialize, Serialize};

use super::{
    image::{async_image_transformer::AsyncImageTransformer, Image},
    json_component::JsonComponent,
};

pub mod narrative_factory;
mod narrative_output;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Narrative {
    pages: Vec<Page>,
}

impl Narrative {
    pub fn new(pages: Vec<Page>) -> Self {
        Self { pages }
    }
}

impl AsyncImageTransformer for Narrative {
    async fn visit_images<C, F>(&mut self, transformer: C) -> Result<(), anyhow::Error>
    where
        C: Fn(Image) -> F + Send,
        F: Future<Output = Result<Image, anyhow::Error>> + Send,
    {
        let mut futures = Vec::new();

        for page in &mut self.pages {
            let mut page = page.clone();
            let future = async {
                if let Image::Created { .. } = page.image {
                    info!("Image already created and saved.");
                    return Ok(page);
                }
                page.image = transformer(page.image.clone()).await?;
                Ok(page) as Result<Page, anyhow::Error>
            };
            futures.push(future);
        }

        let stream = futures::stream::iter(futures).buffered(3);
        let pages = stream.try_collect::<Vec<_>>().await?;

        self.pages = pages;

        Ok(())
    }
}

impl JsonComponent for Narrative {
    fn save(
        &self,
        game_id: &str,
        file_manager: &crate::file_manager::FileManager,
    ) -> Result<(), anyhow::Error> {
        let file_path = format!("{}/tmp/narrative.json", game_id);
        file_manager.write_json(&file_path, self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub narrative: String,
    pub image: Image,
}

impl Page {
    pub fn new(narrative: String, image: Image) -> Self {
        Self { narrative, image }
    }
}
