use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageObject {
    id: String,
    object: String,
    created_at: i64,
    thread_id: String,
    role: MessageRole,
    content: Vec<MessageContent>,
    assistant_id: Option<String>,
    run_id: Option<String>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>,
}

impl MessageObject {
    pub fn get_text_content(&self) -> String {
        if let Some(message_content) = self.content.get(0) {
            return match message_content {
                MessageContent::Text { text } => text.value.clone(),
                _ => "".to_string(),
            };
        }
        String::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "image_file")]
    ImageFile { image_file: MessageImageFile },
    #[serde(rename = "text")]
    Text { text: MessageText },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageImageFile {
    file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageText {
    value: String,
    annotations: Vec<MessageTextAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MessageTextAnnotation {
    #[serde(rename = "file_citation")]
    FileCitation {
        text: String,
        start_index: usize,
        end_index: usize,
        file_citation: MessageFileCitation,
    },
    #[serde(rename = "file_path")]
    FilePath {
        text: String,
        start_index: usize,
        end_index: usize,
        file_path: MessageFilePath,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageFileCitation {
    file_id: String,
    quote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageFilePath {
    file_id: String,
}
