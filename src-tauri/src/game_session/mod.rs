pub mod character_session;
pub mod game_functions;
pub mod game_session_error;

use anyhow::{anyhow, bail, Context};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    commands::character_prompt::character_prompt_request::CharacterPromptRequest,
    file_manager::FileManager,
    game::Game,
    game_state::GameState,
    openai_client::{
        assistant::assistant_create_request::AssistantCreateRequest,
        assistant_tool::function::Function,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSession {
    pub id: String,
    pub game_id: String,
    pub narrator_assistant_id: String,
    pub thread_id: String,
    pub game_state: GameState,
    pub character_session: Option<CharacterSession>,
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

        let functions = vec![
            Function::from_file("./prompts/narrator/add_item_function.json")?,
            Function::from_file("./prompts/narrator/remove_item_function.json")?,
            Function::from_file("./prompts/narrator/new_scene_function.json")?,
            Function::from_file("./prompts/narrator/character_interact_function.json")?,
            Function::from_file("./prompts/narrator/end_game_function.json")?,
        ];

        let assistant_response = openai_client
            .create_assistant(AssistantCreateRequest::new(
                instructions,
                game_id.to_string(),
                ChatCompletionModel::Gpt3_5Turbo1106,
                functions,
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

        let mut game_session = GameSession {
            id,
            game_id,
            narrator_assistant_id,
            thread_id,
            game_state: GameState::new(),
            character_session: None,
        };

        game_session.save(file_manager)?;

        let narrator_response = game_session.process_run(openai_client, game).await?;
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

    pub async fn process_game_prompt(
        &mut self,
        prompt: &str,
        openai_client: &OpenAIClient,
        file_manager: &FileManager,
    ) -> Result<&GameState, anyhow::Error> {
        let game = Game::load(&self.game_id, file_manager)?;
        let create_message_response = openai_client
            .create_message(CreateMessageRequest::new(prompt), &self.thread_id)
            .await
            .map_err(|e| anyhow!("Failed to create new message and add to thread: {:?}", e))?;

        self.game_state
            .add_player_message(&create_message_response.content[0].text.value);

        let narrator_response = self.process_run(openai_client, game).await?;
        self.game_state.add_narrator_message(&narrator_response);

        Ok(&self.game_state)
    }

    async fn process_run(
        &mut self,
        openai_client: &OpenAIClient,
        game: Game,
    ) -> Result<String, anyhow::Error> {
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
        loop {
            if let Ok(retrieve_run_response) =
                openai_client.retrieve_run(&self.thread_id, &run_id).await
            {
                match retrieve_run_response.status.as_str() {
                    "requires_action" => {
                        let tool_calls: Vec<ToolCall> = retrieve_run_response.required_action.ok_or(anyhow!("Received requires_action status with no required_action field on run response."))?.submit_tool_outputs.tool_calls;

                        let mut submit_tool_outputs_request = SubmitToolOutputsRequest::new();

                        for tool_call in tool_calls {
                            let function_name = tool_call.function.name.as_str();
                            let arguments =
                                serde_json::from_str::<Value>(&tool_call.function.arguments)?;
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
                                }
                                "character_interact" => {
                                    // TODO: Set character interaction here and on game state
                                    todo!("Set up character interaction function.")
                                }
                                "end_game" => {
                                    let reason = arguments["reason"]
                                        .as_str()
                                        .ok_or(anyhow!(
                                            "Unable to interpret arguments for end_game function."
                                        ))?
                                        .to_string();
                                    self.game_state.end_game = Some(reason);

                                    submit_tool_outputs_request.add_output(
                                        &tool_call.id,
                                        &json!({
                                            "success": "true"
                                        })
                                        .to_string(),
                                    )
                                }
                                _ => bail!(
                                    "Received invalid function call request for narrator: {}.",
                                    tool_call.function.name
                                ),
                            }
                        }

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
                        bail!("Unable to complete request - assistant run failed or expired.")
                    }
                    "completed" => break,
                    _ => {}
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await
        }
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
        self.character_session
            .as_mut()
            .ok_or(anyhow!("No active character session."))?
            .process_prompt(request, openai_client, file_manager, &mut self.game_state)
            .await?;

        Ok(&self.game_state)
    }
}
