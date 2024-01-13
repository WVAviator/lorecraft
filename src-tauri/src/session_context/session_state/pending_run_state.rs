use anyhow::{anyhow, bail};
use log::info;
use openai_lib::{OpenAIClient, run::{CreateRunRequest, RunClient}};

use crate::{
    game_state::GameState, 
    session_context::session_request::SessionRequest,
};

use super::SessionState;

pub struct PendingRunState {}

impl PendingRunState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => {
                info!("Creating new run on thread.");

                let run_request = CreateRunRequest::builder()
                    .assistant_id(&game_state.assistant_id)
                    .additional_instructions(format!(
                        "Current player inventory: [{}]",
                        game_state.get_player_inventory().join(", ")
                    ))
                    .build();
                let create_run_response = openai_client
                    .create_run(run_request, &game_state.thread_id)
                    .await
                    .map_err(|e| anyhow!("Failed to create run: {:?}", e))?;
                let run_id = create_run_response.id;

                info!("Run triggered with id: {}", &run_id);

                Ok(SessionState::PollingRunState { run_id })
            }
            _ => bail!(
                    "Received invalid session request for pending run: {:?}. Expected ContinueProcessing.",
                    session_request
                ),
            
        }
    }
}
