use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModerationObject {
    id: String,
    model: ModerationModel,
    results: Vec<ModerationResult>,
}

impl ModerationObject {
    pub fn has_failed(&self) -> bool {
        self.results.iter().any(|r| r.flagged)
    }

    pub fn failure_reasons(&self) -> Vec<String> {
        self.results
            .iter()
            .filter(|r| r.flagged)
            .map(|r| {
                r.categories
                    .iter()
                    .filter(|(k, v)| v == true)
                    .map(|(k, v)| k.to_string())
                    .collect()
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModerationModel {
    #[serde(rename = "text-moderation-latest")]
    TextModerationLatest,
    #[serde(rename = "text-moderation-stable")]
    TextModerationStable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModerationResult {
    flagged: bool,
    categories: HashMap<String, bool>,
    category_scores: HashMap<String, f64>,
}
