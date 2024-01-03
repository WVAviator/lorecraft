use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMessagesResponse {
    pub object: String,
    pub data: Vec<MessageData>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub thread_id: String,
    pub role: String,
    pub content: Vec<MessageContent>,
    pub file_ids: Vec<String>,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: MessageText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageText {
    pub value: String,
    pub annotations: Vec<String>,
}

// {
//   "object": "list",
//   "data": [
//     {
//       "id": "msg_abc123",
//       "object": "thread.message",
//       "created_at": 1699016383,
//       "thread_id": "thread_abc123",
//       "role": "user",
//       "content": [
//         {
//           "type": "text",
//           "text": {
//             "value": "How does AI work? Explain it in simple terms.",
//             "annotations": []
//           }
//         }
//       ],
//       "file_ids": [],
//       "assistant_id": null,
//       "run_id": null,
//       "metadata": {}
//     },
//     {
//       "id": "msg_abc456",
//       "object": "thread.message",
//       "created_at": 1699016383,
//       "thread_id": "thread_abc123",
//       "role": "user",
//       "content": [
//         {
//           "type": "text",
//           "text": {
//             "value": "Hello, what is AI?",
//             "annotations": []
//           }
//         }
//       ],
//       "file_ids": [
//         "file-abc123"
//       ],
//       "assistant_id": null,
//       "run_id": null,
//       "metadata": {}
//     }
//   ],
//   "first_id": "msg_abc123",
//   "last_id": "msg_abc456",
//   "has_more": false
// }
