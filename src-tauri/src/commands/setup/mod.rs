use log::{error, info};
use tauri::State;

use crate::{application_state::ApplicationState, file_manager::FileManager};

use self::setup_error::SetupError;
use self::setup_response::{SetupFailureResponse, SetupSuccessResponse};

use tokio::sync::Mutex;

mod setup_error;
mod setup_request;
mod setup_response;

#[tauri::command]
pub async fn setup(
    state: State<'_, Mutex<ApplicationState>>,
    app: tauri::AppHandle,
) -> Result<SetupSuccessResponse, SetupFailureResponse> {
    info!("Initializing file manager.");
    let file_manager = FileManager::new(&app.path_resolver()).map_err(|e| {
        error!("Failed to initialize file manager:\n{:?}", e);
        SetupFailureResponse::new(SetupError::FileSystemError(String::from(
            "Unable to create or access game files in local data directory.",
        )))
    })?;

    {
        let mut state = state.lock().await;
        state.set_file_manager(file_manager);
    }

    // TODO: Setup and verify OpenAI client and connectivity
    // TODO: Setup and verify app config

    Ok(SetupSuccessResponse::new())
}
