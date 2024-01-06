pub mod character_session;
pub mod game_functions;
pub mod game_session_error;

use anyhow::{anyhow, bail, Context};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::{mpsc::Sender, Mutex};

use crate::{
    commands::character_prompt::character_prompt_request::CharacterPromptRequest,
    file_manager::FileManager,
    game::Game,
    game_state::GameState,
    openai_client::{
        assistant_create::assistant_create_request::AssistantCreateRequest,
        assistant_tool::{function::Function, AssistantTool},
        chat_completion::chat_completion_model::ChatCompletionModel,
        create_message::create_message_request::CreateMessageRequest,
        create_run::create_run_request::CreateRunRequest,
        list_messages::list_messages_query::{ListMessagesQuery, ListMessagesQueryBuilder},
        retrieve_run::retrieve_run_response::ToolCall,
        submit_tool_outputs::submit_tool_outputs_request::SubmitToolOutputsRequest,
        OpenAIClient,
    },
    prompt_builder::PromptBuilder,
    utils::random::Random,
};

use self::{
    character_session::CharacterSession,
    game_functions::{item_update::ItemUpdate, scene_update::SceneUpdate},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSession {
    pub id: String,
    pub game_id: String,
    pub narrator_assistant_id: String,
    pub thread_id: String,
    pub game_state: GameState,
    pub character_session: Option<CharacterSession>,
    #[serde(skip)]
    game_state_update_tx: Option<Sender<GameState>>,
    #[serde(skip)]
    character_end_tx: Option<tokio::sync::oneshot::Sender<String>>,
}

impl GameSession {
    pub async fn start_new(
        game_id: String,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
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

        let id = Random::generate_id();

        let game_state = GameState::new(&game, &narrator_assistant_id, &thread_id);

        let mut game_session = GameSession {
            id,
            game_id,
            narrator_assistant_id,
            thread_id,
            game_state,
            character_session: None,
            game_state_update_tx: None,
            character_end_tx: None,
        };

        game_session.save(file_manager)?;

        let narrator_response = game_session
            .process_run(openai_client, game, file_manager)
            .await?;
        game_session
            .game_state
            .add_narrator_message(&narrator_response);

        Ok(game_session)
    }

    pub fn save(&self, file_manager: &FileManager) -> Result<(), anyhow::Error> {
        let filepath = format!("save_data/{}/{}.json", self.game_id, self.id);
        let json =
            serde_json::to_string(&self).context("Error serializing game session to json.")?;
        file_manager
            .write_to_file(&filepath, &json)
            .context("Error writing game session to file.")?;

        Ok(())
    }

    pub fn add_state_tx(&mut self, game_state_update_tx: Sender<GameState>) {
        self.game_state_update_tx = Some(game_state_update_tx);
    }

    async fn send_state_update(&self) -> Result<(), anyhow::Error> {
        if let Some(game_state_update_tx) = &self.game_state_update_tx {
            let game_state = self.game_state.clone();
            game_state_update_tx
                .send(game_state)
                .await
                .context("Error sending game state update.")?;
        }

        Ok(())
    }

    pub async fn process_game_prompt(
        &mut self,
        prompt: &str,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<&GameState, anyhow::Error> {
        let game = Game::load(&self.game_id, file_manager)?;

        info!("Loaded game from file.");

        let create_message_response = openai_client
            .create_message(CreateMessageRequest::new(prompt), &self.thread_id)
            .await
            .map_err(|e| anyhow!("Failed to create new message and add to thread: {:?}", e))?;

        info!("Message appended to thread.");
        self.game_state
            .add_player_message(&create_message_response.content[0].text.value);

        self.send_state_update().await?;

        let narrator_response = self.process_run(openai_client, game, file_manager).await?;
        self.game_state.add_narrator_message(&narrator_response);

        self.send_state_update().await?;

        Ok(&self.game_state)
    }

    async fn process_run(
        &mut self,
        openai_client: &OpenAIClient,
        game: Game,
        file_manager: &FileManager,
    ) -> Result<String, anyhow::Error> {
        info!("Creating new run on thread.");

        let run_request = CreateRunRequest::builder()
            .assistant_id(&self.narrator_assistant_id)
            .additional_instructions(format!(
                "Current player inventory: [{}]",
                self.game_state.get_inventory().join(", ")
            ))
            .build();
        let create_run_response = openai_client
            .create_run(run_request, &self.thread_id)
            .await
            .map_err(|e| anyhow!("Failed to create run: {:?}", e))?;
        let run_id = create_run_response.id;

        info!("Run triggered with id: {}", &run_id);

        loop {
            info!("Polling for run response...");
            if let Ok(retrieve_run_response) =
                openai_client.retrieve_run(&self.thread_id, &run_id).await
            {
                match retrieve_run_response.status.as_str() {
                    "requires_action" => {
                        info!("Assistant requested function tool invocation.");
                        let tool_calls: Vec<ToolCall> = retrieve_run_response.required_action.ok_or(anyhow!("Received requires_action status with no required_action field on run response."))?.submit_tool_outputs.tool_calls;

                        let mut submit_tool_outputs_request = SubmitToolOutputsRequest::new();

                        for tool_call in tool_calls {
                            let function_name = tool_call.function.name.as_str();
                            let arguments =
                                serde_json::from_str::<Value>(&tool_call.function.arguments)?;

                            info!(
                                "Processing arguments to function {}: {}",
                                &function_name, &tool_call.function.arguments
                            );

                            match function_name {
                                "new_scene" => {
                                    let scene_info = SceneUpdate::new_scene(
                                        arguments,
                                        &game,
                                        &mut self.game_state,
                                    )
                                    .context("Unable to create response object for new_scene")?;
                                    let output = serde_json::to_string(&scene_info).context(
                                        "Unable to serialize response object for new_scene.",
                                    )?;
                                    submit_tool_outputs_request.add_output(&tool_call.id, &output);

                                    info!("Processed new scene function with output: {}", &output);
                                }
                                "add_item" => {
                                    let item_update =
                                        ItemUpdate::add_item(arguments, &mut self.game_state)
                                            .context(
                                                "Unable to create response object for add_item.",
                                            )?;
                                    let output = serde_json::to_string(&item_update).context(
                                        "Unable to serialize response object for add_item",
                                    )?;
                                    submit_tool_outputs_request.add_output(&tool_call.id, &output);

                                    info!("Processed add_item function with output: {}", &output);
                                }
                                "remove_item" => {
                                    let item_update =
                                        ItemUpdate::remove_item(arguments, &mut self.game_state)
                                            .context(
                                                "Unable to create response object for remove_item.",
                                            )?;
                                    let output = serde_json::to_string(&item_update).context(
                                        "Unable to serialize response object for remove_item.",
                                    )?;
                                    submit_tool_outputs_request.add_output(&tool_call.id, &output);

                                    info!(
                                        "Processed remove_item function with output: {}",
                                        &output
                                    );
                                }
                                "character_interact" => {
                                    let name = arguments["character"].as_str().ok_or(anyhow!("Unable to interpret arguments for character_interact function."))?.to_string();

                                    let name = {
                                        if name.contains(":") {
                                            name.split(":").next().unwrap().to_string()
                                        } else {
                                            name
                                        }
                                    };

                                    info!("Establishing character session for {}", &name);

                                    let character_session = CharacterSession::new(
                                        &name,
                                        &self.game_id,
                                        openai_client,
                                        file_manager,
                                        &mut self.game_state,
                                    )
                                    .await
                                    .context("Failed to establish character session.")?;

                                    self.character_session = Some(character_session);
                                    let (tx, rx) = tokio::sync::oneshot::channel();
                                    self.character_end_tx = Some(tx);

                                    self.send_state_update().await?;

                                    info!("Set oneshot transmitter, awaiting character summary.");

                                    let conversation_summary = rx.await.context("Error receiving continution response for character interaction.")?;

                                    info!(
                                        "Conversation summary received: {}",
                                        &conversation_summary
                                    );

                                    let output = json!({
                                        "conversation_summary": &conversation_summary,
                                        "updated_player_inventory": &self.game_state.get_inventory()
                                    })
                                    .to_string();

                                    submit_tool_outputs_request.add_output(&tool_call.id, &output);
                                }
                                "end_game" => {
                                    let reason = arguments["reason"]
                                        .as_str()
                                        .ok_or(anyhow!(
                                            "Unable to interpret arguments for end_game function."
                                        ))?
                                        .to_string();
                                    self.game_state.end_game = Some(reason);
                                    let output = json!({ "success": "true" }).to_string();

                                    submit_tool_outputs_request.add_output(&tool_call.id, &output);

                                    info!("Processed end_game function with output: {}", &output)
                                }
                                _ => bail!(
                                    "Received invalid function call request for narrator: {}.",
                                    tool_call.function.name
                                ),
                            }
                        }

                        info!("Sending tool outputs to run {}", &run_id);
                        openai_client
                            .submit_tool_outputs(
                                submit_tool_outputs_request,
                                &self.thread_id,
                                &run_id,
                            )
                            .await
                            .map_err(|e| anyhow!("Unable to submit tool outputs: {:?}", e))?;
                    }
                    "cancelling" | "cancelled" | "failed" | "expired" => {
                        error!("Run {} was cancelled or timed out.", &run_id);
                        bail!("Unable to complete request - assistant run failed or expired.")
                    }
                    "completed" => {
                        info!("Run response shows completed.");
                        break;
                    }
                    _ => {}
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await
        }

        let narrator_response = self.get_last_message(openai_client).await?;

        info!("Received response from assistant: {}", &narrator_response);

        Ok(narrator_response)
    }

    async fn get_last_message(
        &mut self,
        openai_client: &OpenAIClient,
    ) -> Result<String, anyhow::Error> {
        info!("Fetching latest message from thread");
        let list_messages_query = ListMessagesQueryBuilder::new(&self.thread_id)
            .limit(1)
            .order("desc")
            .build();
        let list_messages_response = openai_client
            .list_messages(list_messages_query)
            .await
            .map_err(|e| anyhow!("Failed to list messages: {:?}", e))?;
        let narrator_response = list_messages_response.data[0].content[0].text.value.clone();
        Ok(narrator_response)
    }

    pub async fn process_character_prompt(
        &mut self,
        request: &CharacterPromptRequest,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<&GameState, anyhow::Error> {
        if let Some(true) = &request.end_conversation {
            info!("Processing end of conversation.");
            return self.end_character_prompt(openai_client, file_manager).await;
        }

        self.character_session
            .as_mut()
            .ok_or(anyhow!("No active character session."))?
            .process_prompt(request, openai_client, file_manager, &mut self.game_state)
            .await?;

        info!("Finished processing character prompt.");

        Ok(&self.game_state)
    }

    pub async fn end_character_prompt(
        &mut self,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<&GameState, anyhow::Error> {
        info!("Player requested to end the conversation.");
        let conversation_summary = self
            .character_session
            .as_mut()
            .ok_or(anyhow!("No active character session."))?
            .end_session(openai_client, file_manager, &mut self.game_state)
            .await?;

        self.character_end_tx
            .take()
            .ok_or(anyhow!(
                "Cannot transmit summary to rusume message processing."
            ))?
            .send(conversation_summary)
            .map_err(|_| anyhow!("Unable to unblock state update."))?;

        self.character_session = None;

        Ok(&self.game_state)
    }
}
