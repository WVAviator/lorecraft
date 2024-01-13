use log::{error, info};
use tauri::State;

use crate::config::Config;
use crate::utils::string_utilities::StringUtilities;
use crate::{application_state::ApplicationState, file_manager::FileManager};

use openai_lib::{ClientConfig, OpenAIClient};

use self::setup_error::SetupError;
use self::setup_request::SetupRequest;
use self::setup_response::{SetupFailureResponse, SetupSuccessResponse};

use tokio::sync::Mutex;

mod setup_error;
mod setup_request;
mod setup_response;

#[tauri::command]
pub async fn setup(
    request: SetupRequest,
    state: State<'_, Mutex<ApplicationState>>,
    app: tauri::AppHandle,
) -> Result<SetupSuccessResponse, SetupFailureResponse> {
    info!("Beginning app setup process.");

    info!("Initializing file manager.");
    let file_manager = FileManager::new(&app.path_resolver()).map_err(|e| {
        error!("Failed to initialize file manager:\n{:?}", e);
        SetupFailureResponse::new(SetupError::FileSystemError(String::from(
            "Unable to create or access game files in local data directory.",
        )))
    })?;

    info!("Initializing app config.");
    let mut config = Config::load(&file_manager).map_err(|e| {
        error!("Failed to initialize config:\n{:?}", e);
        SetupFailureResponse::new(SetupError::FileSystemError(String::from(
            "Unable to create or access game config file in local data directory.",
        )))
    })?;

    info!("Verifying API key.");
    let api_key = match request.openai_api_key {
        Some(api_key) => {
            info!(
                "API key received from user: {}",
                StringUtilities::truncate(&api_key, 8)
            );
            config.openai_api_key = Some(api_key.clone());
            config.save(&file_manager).map_err(|e| {
                error!("Error attempting to save new API key to config:\n{:?}", e);
                SetupFailureResponse::new(SetupError::FileSystemError(String::from(
                    "Error saving config.",
                )))
            })?;
            api_key
        }
        None => match config.openai_api_key {
            Some(api_key) => {
                info!(
                    "API key loaded from config: {}",
                    StringUtilities::truncate(&api_key, 8)
                );
                api_key
            }
            None => {
                error!("No API key found in config.");
                return Err(SetupFailureResponse::new(SetupError::MissingOpenAIKey(
                    String::from("Please provide an API key for OpenAI."),
                )));
            }
        },
    };

    info!("Initializing OpenAI client.");
    let openai_client = OpenAIClient::new(ClientConfig { api_key }).map_err(|_| {
        SetupFailureResponse::new(SetupError::BadOpenAIKey(String::from(
            "An error occurred in setting up OpenAI client.",
        )))
    })?;

    info!("Verifying connection and authorization with OpenAI.");
    if let Err(error) = openai_client.verify_connection().await {
        match error {
            openai_lib::Error::ResponseFailure(_) => {
                error!("OpenAI rejected authorization:\n{:?}", error);
                return Err(SetupFailureResponse::new(SetupError::BadOpenAIKey(
                    String::from("OpenAI API key is expired or incorrect."),
                )));
            }
            _ => {
                error!("Unable to connect to OpenAI:\n{:?}", error);
                return Err(SetupFailureResponse::new(SetupError::ConnectionFailed(
                    String::from("Unable to connect to the OpenAI API."),
                )));
            }
        }
    }

    info!("Initializing app state.");
    {
        let mut state = state.lock().await;
        state.set_file_manager(file_manager);
        state.set_openai_client(openai_client);
    }

    info!("Initialization complete.");
    Ok(SetupSuccessResponse::new())
}
