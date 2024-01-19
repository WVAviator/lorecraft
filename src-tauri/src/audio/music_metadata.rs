use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MusicMetadata {
    attribution: Attribution,
    pub keywords: String,
    pub filename: String,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attribution {
    title: String,
    author: String,
    from: String,
}

impl MusicMetadata {
    pub fn load_from_file(path: impl Into<String>) -> Result<Vec<Self>, anyhow::Error> {
        let path = path.into();

        let data = std::fs::read_to_string(&path)?;
        let data: Vec<Self> = serde_json::from_str(&data)?;

        info!("Loaded music metadta from file: {:?}", &path);

        Ok(data)
    }
}
