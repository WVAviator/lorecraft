use crate::{file_manager::FileManager, game::game_metadata::GameMetadata};

use super::image_factory::ImageFactory;

pub trait ImageMultiprocessor {
    async fn generate_images(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error>;
}
