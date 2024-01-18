use anyhow::{bail, anyhow};
use log::{info, error, trace};
use openai_lib::{OpenAIClient, assistant::{AssistantClient, CreateAssistantRequest}, model::ChatModel, tool::Tool, thread::{ThreadClient, CreateThreadRequest}};
use serde_json::json;

use crate::{session_context::session_request::SessionRequest, game_state::{GameState, character_interaction::CharacterInteraction, character_save_data::CharacterSaveData, character_profile::CharacterProfile}, game::Game, prompt_builder::PromptBuilder};

use super::SessionState;

pub struct ProcessCharacterInteractState {}

impl ProcessCharacterInteractState {
    pub async fn process(request: SessionRequest, openai_client: &OpenAIClient, game_state: &mut GameState, run_id: String, tool_call_id: String, arguments: serde_json::Value, game: &Game) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                
                let character_name = arguments["character"].as_str().ok_or(anyhow!("Unable to interpret arguments for character_interact function."))?.to_string();

                let character_name = {
                    if character_name.contains(":") {
                        character_name.split(":").next().ok_or(anyhow!("Narrator prefixed character name with colon."))?.to_string()
                    } else {
                        character_name
                    }
                };

                let character = game
                    .characters
                    .iter()
                    .find(|c| c.name == character_name)
                    .ok_or(anyhow!("Unable to find character with provided name."))?;

                // TODO: Make sure character is in the active scene.

                info!(
                    "Found character information for {}. Generating profile and context for assistant.",
                    &character_name
                );

                let profile = CharacterProfile::from_character(&character)?;
                let profile = serde_json::to_string(&profile)?;

                let character_save_data = game_state.character_save_data.entry(character.name.clone())
                    .or_insert(CharacterSaveData::new(character.inventory.clone()));

                let additional_context = json!({
                    "previous_conversations": &character_save_data.previous_conversations,
                    "character_inventory": &character_save_data.character_inventory,
                })
                .to_string();

                let instructions = PromptBuilder::new()
                    .add_prompt("./prompts/character_actor/main.txt")
                    .add_plain_text(&profile)
                    .add_plain_text(&additional_context)
                    .build();

                info!("Loaded instructions for character assistant.");
                trace!("{}", &instructions);

                info!("Creating character assistant.");

                let assistant_response = openai_client
                    .create_assistant(
                    CreateAssistantRequest::builder()
                        .instructions(instructions)
                        .model(ChatModel::Gpt_35_Turbo_1106)
                        .add_tool(Tool::function().from_file("./prompts/character_actor/give_function.json")?)
                            .add_tool(Tool::function().from_file("./prompts/character_actor/trade_function.json")?)
                            .build())
                    .await
                    .map_err(|e| anyhow!("Failed to create assistant for character: {}", e))?;

                let character_assistant_id = assistant_response.id;

                info!(
                    "Created assistant with id {} for character.",
                    &character_assistant_id
                );

                let thread_response = openai_client.create_thread(CreateThreadRequest::builder().build()).await.map_err(|e| {
                    error!("Failed to create thread for character assistant:\n{:?}", e);
                    anyhow!("Failed to start thread.")
                })?;

                let thread_id = thread_response.id;

                info!(
                    "Created thread for character conversation with id {}.",
                    &thread_id
                );

                let character_interaction = CharacterInteraction::builder()
                    .character_name(&character.name)
                    .assistant_id(&character_assistant_id)
                    .thread_id(&thread_id)
                    .initiating_run_id(&run_id)
                    .initiating_tool_call_id(&tool_call_id)
                    .build()?;

                game_state.character_interact(character_interaction);

                Ok(SessionState::CharacterRunRequestState)
            }
            _ => bail!("Received invalid request for processing character interaction: {:?}. Expected ContinueProcessing.", &request)
        }
    }
}
