use anyhow::{anyhow, bail};
use log::info;

use crate::{
    game_state::GameState,
    openai_client::{
        list_messages::list_messages_query::ListMessagesQueryBuilder,
        retrieve_run::retrieve_run_response::ToolCall, OpenAIClient,
    },
    session_context::{session_request::SessionRequest, session_state::SessionState},
    utils::string_utilities::StringUtilities,
};

pub struct ReadMessageState {}

impl ReadMessageState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => {
                info!("Fetching latest message from thread");
                let list_messages_query = ListMessagesQueryBuilder::new(&game_state.thread_id)
                    .limit(1)
                    .order("desc")
                    .build();
                let list_messages_response = openai_client
                    .list_messages(list_messages_query)
                    .await
                    .map_err(|e| anyhow!("Failed to list messages: {:?}", e))?;
                let response = list_messages_response.data[0].content[0].text.value.clone();

                info!(
                    "Received latest response from thread: '{}'",
                    StringUtilities::truncate(&response, 20)
                );

                game_state.add_narrator_message(&response);

                Ok(SessionState::IdleState)
            }
            _ => bail!(
                "Received invalid request type {:?}. Expected ContinueProcessing.",
                &session_request
            ),
        }
    }
}
