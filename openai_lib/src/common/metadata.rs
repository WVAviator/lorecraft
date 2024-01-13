use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Metadata {
    #[serde(flatten)]
    metadata: HashMap<String, String>,
}

impl Metadata {
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self
            .metadata
            .iter()
            .any(|(key, value)| key.chars().count() > 64 || value.chars().count() > 512)
        {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'metadata' must have keys of 64 characters or less and values of 512 characters or less.",
            )));
        }

        if self.metadata.len() > 16 {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'metadata' must have 16 or fewer items.",
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;

    #[test]
    fn metadata_flattens_during_serialization() {
        #[derive(Serialize)]
        struct Test {
            a: Metadata,
        }

        let test = Test {
            a: Metadata {
                metadata: HashMap::from([("b".into(), "c".into()), ("d".into(), "e".into())]),
            },
        };

        let actual = serde_json::to_value(&test).unwrap();
        let expected = json!({
            "a": {
                "b": "c",
                "d": "e"
            }
        });

        assert_json_include!(actual: actual, expected: expected);
    }

    #[test]
    fn metadata_unflattens_during_deserialization() {
        let json = json!({
            "a": "b",
            "c": "d"
        })
        .to_string();

        let metadata = serde_json::from_str::<Metadata>(&json).unwrap();

        assert_eq!(metadata.metadata.len(), 2);
        assert_eq!(metadata.metadata.get("a").unwrap(), "b");
        assert_eq!(metadata.metadata.get("c").unwrap(), "d");
    }
}
