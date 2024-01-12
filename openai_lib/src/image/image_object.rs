use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageObject {
    b64_json: Option<String>,
    url: Option<String>,
    revised_prompt: Option<String>,
}
