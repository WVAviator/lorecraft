use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummarizedScene {
    name: String,
    description: String,
    actions: String,
}
