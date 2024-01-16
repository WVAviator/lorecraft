use futures::{Future, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

use super::image::{async_image_transformer::AsyncImageTransformer, Image};

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
        C: Fn(Image) -> F + Send + 'static,
        F: Future<Output = Result<Image, anyhow::Error>> + Send + 'static,
    {
        let mut futures = Vec::new();

        for page in &mut self.pages {
            let mut page = page.clone();
            let future = async {
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
