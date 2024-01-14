use anyhow::{anyhow, bail};
use log::info;
use openai_lib::{
    message::{ListMessagesRequest, MessageClient, MessageSortOrder},
    OpenAIClient,
};

use crate::{
    game_state::GameState,
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
                let list_messages_response = openai_client
                    .list_messages(
                        ListMessagesRequest::builder()
                            .limit(1)
                            .order(MessageSortOrder::Descending)
                            .build(),
                        &game_state.thread_id,
                    )
                    .await
                    .map_err(|e| anyhow!("Failed to list messages: {:?}", e))?;
                // let response = list_messages_response.data[0].content[0].text.value.clone();
                let response = list_messages_response
                    .data
                    .get(0)
                    .ok_or(anyhow!("No messages returned from thread."))?
                    .get_text_content();

                info!(
                    "Received latest response from thread: '{}'",
                    StringUtilities::truncate(&response, 20)
                );

                let response = ReadMessageState::remove_follow_up(response);

                game_state.add_narrator_message(&response);

                Ok(SessionState::IdleState)
            }
            _ => bail!(
                "Received invalid request type {:?}. Expected ContinueProcessing.",
                &session_request
            ),
        }
    }

    fn remove_follow_up(message: String) -> String {
        // Sometimes, against its instructions, the narrative response will end with a question
        // like "What will you do now?" which is repetitive and undesirable

        let original_message = message.clone();

        match message.ends_with("?") {
            true => {
                // Split into words, reverse, and remove until hitting a word with punctuation
                let initial_removal = message
                    .split_whitespace()
                    .rev()
                    .skip(1)
                    .skip_while(|word| {
                        !word.ends_with(".")
                            && !word.ends_with("?")
                            && !word.ends_with("!")
                            && !word.ends_with(".\"")
                            && !word.ends_with("?\"")
                            && !word.ends_with("!\"")
                    })
                    .collect::<Vec<&str>>();

                // This will be empty if the only sentence is a question, in that case don't alter
                if initial_removal.is_empty() {
                    return original_message;
                }

                info!("Narrator responded with a follow-up question. Removed last sentence of response.");

                // Reverse back and rebuild the remaining original sentences
                initial_removal
                    .into_iter()
                    .rev()
                    .collect::<Vec<&str>>()
                    .join(" ")
            }
            false => message,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn removes_trailing_follow_up() {
        let sentences = String::from("Hello. How are you? I'm okay. What do you want to do now?");
        let result = ReadMessageState::remove_follow_up(sentences);

        assert_eq!(result, "Hello. How are you? I'm okay.");
    }

    #[test]
    fn only_removes_last_even_if_all_questions() {
        let sentences = String::from("Hello? How are you. I'm okay? What do you want to do now?");
        let result = ReadMessageState::remove_follow_up(sentences);

        assert_eq!(result, "Hello? How are you. I'm okay?");
    }

    #[test]
    fn does_not_remove_if_not_question() {
        let sentences = String::from("Hello. How are you. I'm okay. What do you want to do now.");
        let result = ReadMessageState::remove_follow_up(sentences);

        assert_eq!(
            result,
            "Hello. How are you. I'm okay. What do you want to do now."
        );
    }

    #[test]
    fn does_not_remove_if_question_only_sentence() {
        let sentences = String::from("What do you want to do now?");
        let result = ReadMessageState::remove_follow_up(sentences);
        assert_eq!(result, "What do you want to do now?");
    }

    #[test]
    fn handles_case_where_previous_sentence_is_quote() {
        let sentences =
            String::from("Hello. How are you? \"I'm okay.\" What do you want to do now?");
        let result = ReadMessageState::remove_follow_up(sentences);

        assert_eq!(result, "Hello. How are you? \"I'm okay.\"");
    }
}
