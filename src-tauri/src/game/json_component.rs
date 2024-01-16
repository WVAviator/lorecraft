use crate::file_manager::FileManager;

pub trait JsonComponent {
    fn save(&self, game_id: &str, file_manager: &FileManager) -> Result<(), anyhow::Error>;
}
