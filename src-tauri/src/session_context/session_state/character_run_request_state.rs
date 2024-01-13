use anyhow::{anyhow, bail};
use log::{info, trace};
use openai_lib::{
    run::{CreateRunRequest, RunClient},
    OpenAIClient,
};

use crate::{game_state::GameState, session_context::session_request::SessionRequest};

use super::SessionState;

pub struct CharacterRunRequestState {}

impl CharacterRunRequestState {
    pub async fn process(
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                let assistant_id = &game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction available in game state."))?
                    .assistant_id
                    .clone();
                let thread_id = &game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction available in game state."))?
                    .thread_id
                    .clone();
                info!("Initiating new run on thread {}", thread_id);

                let run_response = openai_client
                    .create_run(
                        CreateRunRequest::builder()
                            .assistant_id(assistant_id)
                            .build(),
                        thread_id,
                    )
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "Unable to create run in thread of character session: {:?}",
                            e
                        )
                    })?;
                trace!("Received run response:\n{:?}", &run_response);

                let run_id = run_response.id;

                Ok(SessionState::CharacterPollingRunState { run_id })
            }
            _ => bail!(
                "Unexpected request received for CharacterRunRequestState: {:?}.",
                request
            ),
        }
    }
}
