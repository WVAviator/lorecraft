use anyhow::ensure;
use log::error;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use crate::file_manager::FileManager;
use crate::game_session::GameSession;
use crate::openai_client::OpenAIClient;

pub struct ApplicationState {
    pub updates_tx: Mutex<mpsc::Sender<String>>,
    pub file_manager: Option<FileManager>,
    pub openai_client: Option<OpenAIClient>,
    pub game_session: Option<GameSession>,
}

impl ApplicationState {
    pub fn new(updates_tx: Mutex<mpsc::Sender<String>>) -> Self {
        Self {
            updates_tx,
            file_manager: None,
            openai_client: None,
            game_session: None,
        }
    }

    pub fn set_file_manager(&mut self, file_manager: FileManager) {
        self.file_manager = Some(file_manager);
    }

    pub fn set_openai_client(&mut self, openai_client: OpenAIClient) {
        self.openai_client = Some(openai_client);
    }

    pub fn verify_setup(&self) -> Result<(), anyhow::Error> {
        ensure!(self.file_manager.is_some(), "File system not set up.");
        ensure!(self.openai_client.is_some(), "OpenAI client not set up.");

        Ok(())
    }

    pub async fn send_update(&self, update: String) {
        let updates_tx = self.updates_tx.lock().await;
        if let Err(_) = updates_tx.send(update).await {
            error!("Failed to send update to UI.");
        }
    }
}
