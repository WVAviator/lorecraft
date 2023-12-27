use tokio::sync::{mpsc, Mutex}

pub mod application_settings;

pub struct ApplicationState {
    updates_tx: Mutex<mpsc::Sender<String>>
}
