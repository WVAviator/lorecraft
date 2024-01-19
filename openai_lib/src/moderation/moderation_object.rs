use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModerationObject {
    id: String,
    model: ModerationModel,
    results: Vec<ModerationResult>,
}

impl ModerationObject {
    pub fn is_flagged(&self) -> bool {
        self.results.iter().any(|r| r.flagged)
    }

    pub fn failure_reasons(&self) -> Vec<String> {
        self.results
            .iter()
            .filter(|r| r.flagged)
            .map(|r| {
                r.categories
                    .iter()
                    .filter(|(_, v)| **v == true)
                    .map(|(k, _)| k.to_string())
                    .collect::<Vec<String>>()
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
    #[serde(rename = "text-moderation-005")]
    TextModeration005,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModerationResult {
    flagged: bool,
    categories: HashMap<String, bool>,
    category_scores: HashMap<String, f64>,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn returns_all_flagged_categories() {
        let moderation_json = json!({
          "id": "modr-XXXXX",
          "model": "text-moderation-stable",
          "results": [
            {
              "flagged": true,
              "categories": {
                "sexual": false,
                "hate": false,
                "harassment": false,
                "self-harm": false,
                "sexual/minors": false,
                "hate/threatening": false,
                "violence/graphic": false,
                "self-harm/intent": false,
                "self-harm/instructions": false,
                "harassment/threatening": true,
                "violence": true,
              },
              "category_scores": {
                "sexual": 1.2282071e-06,
                "hate": 0.010696256,
                "harassment": 0.29842457,
                "self-harm": 1.5236925e-08,
                "sexual/minors": 5.7246268e-08,
                "hate/threatening": 0.0060676364,
                "violence/graphic": 4.435014e-06,
                "self-harm/intent": 8.098441e-10,
                "self-harm/instructions": 2.8498655e-11,
                "harassment/threatening": 0.63055265,
                "violence": 0.99011886,
              }
            }
          ]
        })
        .to_string();

        let moderation_object = serde_json::from_str::<ModerationObject>(&moderation_json).unwrap();
        let mut flagged_categories = moderation_object.failure_reasons();

        let mut expected = vec!["violence".to_string(), "harassment/threatening".to_string()];
        flagged_categories.sort();
        expected.sort();

        assert_eq!(flagged_categories, expected);
    }
}
