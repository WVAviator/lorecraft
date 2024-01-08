pub mod game_session_error;

use std::sync::Arc;

use anyhow::anyhow;
use log::{error, info};
use tokio::sync::{mpsc::Sender, Mutex};

use crate::{
    file_manager::FileManager,
    game::Game,
    game_state::GameState,
    openai_client::{
        assistant_create::assistant_create_request::AssistantCreateRequest,
        assistant_tool::{function::Function, AssistantTool},
        chat_completion::chat_completion_model::ChatCompletionModel,
        OpenAIClient,
    },
    prompt_builder::PromptBuilder,
    session_context::{session_request::SessionRequest, SessionContext},
};

#[derive(Debug)]
pub struct GameSession {
    pub game: Game,
    pub game_state: GameState,
    session_context: SessionContext,
}

impl GameSession {
    pub async fn start_new(
        game_id: String,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
        state_update_tx: Arc<Mutex<Sender<GameState>>>,
    ) -> Result<Self, anyhow::Error> {
        info!("Starting new game session for game id {}.", &game_id);
        let game = Game::load(&game_id, file_manager)?;

        let summary_text = format!("Game Summary:\n{}", &game.summary.summary);
        let scene_list_text = format!(
            "Scene List:\n[{}]",
            game.scenes
                .iter()
                .map(|scene| format!("{},", scene.name))
                .collect::<String>()
        );

        let instructions = PromptBuilder::new()
            .add_prompt("./prompts/narrator/main.txt")
            .add_plain_text(&summary_text)
            .add_plain_text(&scene_list_text)
            .build();

        let tools = vec![
            AssistantTool::new_function(Function::from_file(
                "./prompts/narrator/add_item_function.json",
            )?),
            AssistantTool::new_function(Function::from_file(
                "./prompts/narrator/remove_item_function.json",
            )?),
            AssistantTool::new_function(Function::from_file(
                "./prompts/narrator/new_scene_function.json",
            )?),
            AssistantTool::new_function(Function::from_file(
                "./prompts/narrator/character_interact_function.json",
            )?),
            AssistantTool::new_function(Function::from_file(
                "./prompts/narrator/end_game_function.json",
            )?),
        ];

        let assistant_response = openai_client
            .create_assistant(AssistantCreateRequest::new(
                instructions,
                game_id.to_string(),
                ChatCompletionModel::Gpt3_5Turbo1106,
                tools,
            ))
            .await
            .expect("Failed to create assistant.");

        let narrator_assistant_id = assistant_response.id;

        let thread_response = openai_client.create_thread().await.map_err(|e| {
            error!("Failed to create thread for Assistant API:\n{:?}", e);
            anyhow!("Failed to start thread.")
        })?;

        let thread_id = thread_response.id;

        let mut game_state = GameState::new(&game, &narrator_assistant_id, &thread_id);
        let openai_client = openai_client.clone();
        let mut session_context = SessionContext::new(game.clone(), openai_client, state_update_tx);

        session_context.process(SessionRequest::ContinueProcessing, &mut game_state).await;

        let game_session = GameSession {
            game,
            game_state,
            session_context,
        };

        Ok(game_session)
    }

    pub async fn receive_player_message(
        &mut self,
        message: String,
    ) -> Result<GameState, anyhow::Error> {
        info!("Received player message: {}", &message);

        self.session_context
            .process(SessionRequest::PlayerEntry(message), &mut self.game_state)
            .await;

        Ok(self.game_state.clone())
    }

    pub async fn receive_trade_response(
        &mut self,
        accepted: bool,
    ) -> Result<GameState, anyhow::Error> {
        info!("Received trade response: {}", &accepted);

        self.session_context
            .process(
                SessionRequest::CharacterTradeResponse(accepted),
                &mut self.game_state,
            )
            .await;

        Ok(self.game_state.clone())
    }

    pub async fn end_character_interaction(&mut self) -> Result<GameState, anyhow::Error> {
        info!("Ending character interaction.");

        self.session_context
            .process(
                SessionRequest::CharacterEndInteraction,
                &mut self.game_state,
            )
            .await;

        Ok(self.game_state.clone())
    }
}
