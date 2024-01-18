use std::sync::Arc;

use anyhow::ensure;
use log::error;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use crate::file_manager::FileManager;
use crate::game::game_generation_update::GameGenerationUpdate;
use openai_lib::OpenAIClient;

pub mod session_state;

pub struct ApplicationState {
    pub updates_tx: Arc<Mutex<mpsc::Sender<GameGenerationUpdate>>>,
    pub file_manager: Option<FileManager>,
    pub openai_client: Option<OpenAIClient>,
}

impl ApplicationState {
    pub fn new(updates_tx: Mutex<mpsc::Sender<GameGenerationUpdate>>) -> Self {
        Self {
            updates_tx: Arc::new(updates_tx),
            file_manager: None,
            openai_client: None,
        }
    }

    pub fn set_file_manager(&mut self, file_manager: FileManager) {
        self.file_manager = Some(file_manager);
    }

    pub fn set_openai_client(&mut self, openai_client: OpenAIClient) {
        self.openai_client = Some(openai_client);
    }

    #[allow(dead_code)]
    pub fn verify_setup(&self) -> Result<(), anyhow::Error> {
        ensure!(self.file_manager.is_some(), "File system not set up.");
        ensure!(self.openai_client.is_some(), "OpenAI client not set up.");

        Ok(())
    }
}
