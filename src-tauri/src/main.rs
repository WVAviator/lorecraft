#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::file_manager::FileManager;
use crate::game::Game;
use application_state::ApplicationState;
use tokio::sync::{mpsc, Mutex};

use tauri::{Manager, State};

mod application_state;
mod file_manager;
mod game;
mod openai_client;
mod prompt_builder;

#[tauri::command]
async fn create_new_game(
    prompt: String,
    state: State<'_, ApplicationState>,
) -> Result<Game, String> {
    println!("Creating new game.");

    let prompt = prompt.to_string();

    println!("Spawning new thread to handle game creation.");

    let prompt = match prompt.as_str() {
        "" => None,
        _ => Some(prompt.as_str()),
    };
    let game = game::Game::create_new(prompt, &state).await;
    println!("Game created: {:?}", &game.name);
    let game_serialized = serde_json::to_string(&game).expect("Failed to serialize game.");
    println!("Game serialized. Saving to file...");
    state
        .file_manager
        .write_to_file(format!("{}/game.json", game.id).as_str(), &game_serialized)
        .expect("Failed to write game to file.");
    println!("Game saved to file.");

    println!("Emitting created event for UI at 'create:{}'.", game.id);

    return Ok(game);
}

fn main() {
    let (updates_tx, mut updates_rx) = mpsc::channel(1);
    let updates_tx = Mutex::new(updates_tx);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_new_game])
        .setup(|app| {
            let file_manager = FileManager::new(&app.path_resolver());
            let state = ApplicationState::new(updates_tx, file_manager);
            app.manage(state);

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(update) = updates_rx.recv().await {
                        app_handle
                            .emit_all("updates", update)
                            .expect("Unable to send update message.")
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error occurred during app initialization.");
}
