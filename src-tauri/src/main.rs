#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::character_prompt::character_prompt;
use crate::commands::game_prompt::game_prompt;
use crate::commands::setup::setup;
use crate::commands::start_game::start_game;
use crate::{
    application_state::session_state::SessionState, commands::create_new_game::create_new_game,
};

use application_state::ApplicationState;
use log::{error, info};
use nosleep::{NoSleep, NoSleepType};
use tokio::sync::{mpsc, Mutex};

use tauri::Manager;
use utils::logger::Logger;

mod application_state;
pub mod audio;
mod commands;
mod config;
mod file_manager;
mod game;
mod game_session;
mod game_state;
mod prompt_builder;
mod session_context;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let mut nosleep = NoSleep::new().unwrap();
    let _handle = nosleep.start(NoSleepType::PreventUserIdleDisplaySleep)?;

    let (updates_tx, mut updates_rx) = mpsc::channel(1);
    let updates_tx = Mutex::new(updates_tx);

    let (state_update_tx, mut state_update_rx) = mpsc::channel(1);
    let state_update_tx = Mutex::new(state_update_tx);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_new_game,
            setup,
            start_game,
            game_prompt,
            character_prompt,
        ])
        .setup(|app| {
            Logger::setup(app);

            info!("Initializing application state.");
            let application_state = ApplicationState::new(updates_tx);
            let application_state = Mutex::new(application_state);
            app.manage(application_state);

            info!("Initializing session state.");
            let session_state = SessionState::new(state_update_tx);
            let session_state = Mutex::new(session_state);
            app.manage(session_state);

            info!("Initializing updates event emitter.");
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(update) = updates_rx.recv().await {
                        if let Err(e) = app_handle.emit_all("updates", update) {
                            error!("Failed to emit update to UI:\n{:?}", e);
                        }
                    }
                }
            });

            info!("Initializing state update event emitter.");
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(update) = state_update_rx.recv().await {
                        if let Err(e) = app_handle.emit_all("state", update) {
                            error!("Failed to emit update to UI:\n{:?}", e);
                        }
                    }
                }
            });

            info!("App initialization complete.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error occurred during app initialization.");

    Ok(())
}
