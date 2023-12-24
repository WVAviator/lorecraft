// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use std::fs;
use std::io::Write;

use tauri::Window;

mod application_state;
mod game;
mod openai;

#[tauri::command]
fn create_new_game(prompt: &str, window: Window) -> Result<String, String> {
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
    std::thread::spawn(move || {
        let prompt = match prompt.as_str() {
            "" => None,
            _ => Some(prompt.as_str()),
        };
        let game = game::Game::create_new(prompt);
        println!("Game created: {:?}", &game.name);
        let game_serialized = serde_json::to_string(&game).expect("Failed to serialize game.");
        println!("Game serialized. Saving to file...");
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("games/{}/game.json", game.name))
            .expect("Failed to open game.json.")
            .write_all(game_serialized.as_bytes())
            .expect("Failed to write to game.json.");
        println!("Game saved to file.");

        println!("Emitting created event for UI.");
        window
            .emit(format!("created:{}", game_id_clone).as_str(), game)
            .expect("Error occurred while emitting read event.");
    });

    println!("Returning game ID to UI.");
    return Ok(game_id);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_new_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
