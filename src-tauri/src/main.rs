// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::file_manager::FileManager;
use rand::Rng;
use tokio::sync::mpsc;

use tauri::Window;

mod application_state;
mod file_manager;
mod game;
mod openai;

#[tauri::command]
async fn create_new_game(prompt: &str, window: Window) -> Result<Game, String> {
    println!("Creating new game.");

    let game_id = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(7)
        .map(char::from)
        .collect::<String>();
    let game_id_clone = game_id.clone();
    println!("Game ID: {}", game_id);

    let prompt = prompt.to_string();

    println!("Spawning new thread to handle game creation.");

    let prompt = match prompt.as_str() {
        "" => None,
        _ => Some(prompt.as_str()),
    };
    let game = game::Game::create_new(prompt);
    println!("Game created: {:?}", &game.name);
    let game_serialized = serde_json::to_string(&game).expect("Failed to serialize game.");
    println!("Game serialized. Saving to file...");
    FileManager::new()
        .write_to_file(
            format!("{}/game.json", game_id_clone).as_str(),
            &game_serialized,
        )
        .expect("Failed to write game to file.");
    println!("Game saved to file.");

    println!(
        "Emitting created event for UI at 'create:{}'.",
        game_id_clone
    );
    window
        .emit(format!("create:{}", game_id_clone).as_str(), game)
        .expect("Error occurred while emitting read event.");


    println!("Returning game ID to UI.");
    return Ok(game);
}

fn main() {
    let (updates_tx, updates_rx) = mpsc::channel(1);
    tauri::Builder::default()
        .manage(ApplicationState {
            updates_tx
        })
        .invoke_handler(tauri::generate_handler![create_new_game])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(update) = updates_rx.recv().await {
                        app_handle
                            .emit_all("updates", update)
                            .expect("Unable to send update message.")
                    }
                }
            })
        })
        .run(tauri::generate_context!())
        .expect("Error occurred during app initialization.");
}