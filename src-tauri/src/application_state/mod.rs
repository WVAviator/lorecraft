use tokio::sync::mpsc;
use tokio::sync::Mutex;

use crate::file_manager::FileManager;

pub mod application_settings;

pub struct ApplicationState {
    pub updates_tx: Mutex<mpsc::Sender<String>>,
    pub file_manager: FileManager,
}

impl ApplicationState {
    pub fn new(updates_tx: Mutex<mpsc::Sender<String>>, file_manager: FileManager) -> Self {
        Self {
            updates_tx,
            file_manager,
        }
    }

    pub async fn send_update(&self, update: String) {
        let updates_tx = self.updates_tx.lock().await;
        if let Err(_) = updates_tx.send(update).await {
            println!("Failed to send update to UI.");
        }
    }
}
