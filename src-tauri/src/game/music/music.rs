use serde::{Deserialize, Serialize};

use crate::game::selection_factory::Selectable;

use super::MusicMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Music {
    Selected {
        src: String,
        metadata: MusicMetadata,
    },
    #[serde(skip)]
    None,
}

impl Music {
    pub fn is_none(&self) -> bool {
        match self {
            Music::None => true,
            _ => false,
        }
    }

    pub fn get_selection_input(meta_path: &str) -> Result<Vec<String>, anyhow::Error> {
        let meta_location = Music::get_meta_location(meta_path);
        let music_themes = MusicMetadata::load_from_file(meta_location)?;
        let music_themes = music_themes
            .into_iter()
            .map(|meta| format!("{}: {}", meta.index, meta.keywords))
            .collect();
        Ok(music_themes)
    }

    fn get_meta_location(meta_path: &str) -> String {
        // Because I know I'll end up forgetting the pattern
        match (meta_path.starts_with("/"), meta_path.ends_with("/")) {
            (true, true) => format!("../public{}meta.json", meta_path),
            (true, false) => format!("../public{}/meta.json", meta_path),
            (false, true) => format!("../public/{}meta.json", meta_path),
            (false, false) => format!("../public/{}/meta.json", meta_path),
        }
    }
}

impl Default for Music {
    fn default() -> Self {
        Music::None
    }
}

impl Selectable for Music {
    fn select_from_response(response: &String, meta_path: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        // Because I know I'll end up forgetting the pattern
        let meta_location = Music::get_meta_location(meta_path);
        // The find_by_index method uses pathbufs so the format doesn't matter
        let metadata = MusicMetadata::find_by_index(&meta_location, response.parse()?)?;
        Ok(Music::Selected {
            src: metadata.get_src(meta_path),
            metadata,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::game::music::music_metadata::Attribution;

    use super::*;
    use assert_json_diff::assert_json_include;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        test: String,
        #[serde(default, skip_serializing_if = "Music::is_none")]
        music: Music,
    }

    #[test]
    fn deserializes_nothing_into_none() {
        let json = json!({
            "test": "abc"
        })
        .to_string();

        let test = serde_json::from_str::<Test>(&json).unwrap();

        assert_eq!(test.music, Music::None);
    }

    #[test]
    fn serialize_none_into_nothing() {
        let test = Test {
            test: "abc".to_string(),
            music: Music::None,
        };

        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, r#"{"test":"abc"}"#);
    }

    #[test]
    fn deserializes_selected() {
        let json = json!({
            "test": "abc",
            "music": {
                "src": "src",
                "metadata": {
                    "index": 0,
                    "keywords": "test",
                    "filename": "src",
                    "attribution": {
                        "title": "title",
                        "author": "author",
                        "from": "from"
                    }
                }
            }
        })
        .to_string();

        let test = serde_json::from_str::<Test>(&json).unwrap();

        match test.music {
            Music::None => assert!(false),
            Music::Selected { .. } => assert!(true),
        }
    }

    #[test]
    fn serializes_selected() {
        let test = Test {
            test: "abc".to_string(),
            music: Music::Selected {
                src: "src".to_string(),
                metadata: MusicMetadata {
                    index: 0,
                    keywords: "test".to_string(),
                    filename: "src".to_string(),
                    attribution: Attribution {
                        title: "title".to_string(),
                        author: "author".to_string(),
                        from: "from".to_string(),
                    },
                },
            },
        };

        let json = serde_json::to_value(&test).unwrap();

        let expected = json!({
            "test": "abc",
            "music": {
                "src": "src",
                "metadata": {
            "index": 0,
            "keywords": "test",
            "filename": "src",
            "attribution": {
                "title": "title",
                "author": "author",
                "from": "from"
            }
                }
            }
        });

        assert_json_include!(actual: json, expected: expected);
    }
}
