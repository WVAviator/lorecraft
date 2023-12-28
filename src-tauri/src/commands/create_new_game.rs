use crate::application_state::ApplicationState;
use crate::game::Game;
use crate::utils::string_utilities::StringUtilities;
use log::info;

use tauri::State;

#[tauri::command]
pub async fn create_new_game(
    prompt: String,
    state: State<'_, ApplicationState>,
) -> Result<Game, String> {
    let prompt = {
        match prompt.as_str() {
            "" => String::from("choose any random unique game idea you can think of"),
            _ => StringUtilities::truncate(&prompt, 497),
        }
    };

    info!("Creating new game from user prompt:\n{}.", &prompt);
    let game = Game::create_new(prompt, &state).await;

    info!(
        "Game with id '{}' created. Serializing and saving game to file.",
        game.id
    );
    let game_serialized = serde_json::to_string(&game).expect("Failed to serialize game.");
    state
        .file_manager
        .write_to_file(format!("{}/game.json", game.id).as_str(), &game_serialized)
        .expect("Failed to write game to file.");

    info!("Game '{}' saved and ready to play.", game.id);
    return Ok(game);
}
