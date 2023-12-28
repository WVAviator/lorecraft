use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub mod application_settings;

pub struct ApplicationState {
    pub updates_tx: Mutex<mpsc::Sender<String>>,
}
