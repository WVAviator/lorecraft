#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::create_new_game::create_new_game;
use crate::commands::game_prompt::game_prompt;
use crate::commands::setup::setup;
use crate::commands::start_game::start_game;

use application_state::ApplicationState;
use log::{error, info};
use tokio::sync::{mpsc, Mutex};

use tauri::Manager;
use utils::logger::Logger;

mod application_state;
mod commands;
mod config;
mod file_manager;
mod game;
mod game_session;
mod game_state;
mod openai_client;
mod prompt_builder;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let (updates_tx, mut updates_rx) = mpsc::channel(1);
    let updates_tx = Mutex::new(updates_tx);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_new_game,
            setup,
            start_game,
            game_prompt
        ])
        .setup(|app| {
            Logger::setup(app);

            info!("Initializing application state.");
            let state = ApplicationState::new(updates_tx);
            let state = Mutex::new(state);
            app.manage(state);

            info!("Initializing updates event emitter.");
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(update) = updates_rx.recv().await {
                        info!("Sending update to UI: {:?}", update);
                        if let Err(e) = app_handle.emit_all("updates", update) {
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
