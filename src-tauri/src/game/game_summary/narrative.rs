use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
  pub narrative: String,
  pub image: String,
}