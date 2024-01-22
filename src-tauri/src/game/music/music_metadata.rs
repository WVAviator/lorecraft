use std::path::PathBuf;

use anyhow::anyhow;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MusicMetadata {
    pub attribution: Attribution,
    pub keywords: String,
    pub filename: String,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attribution {
    pub title: String,
    pub author: String,
    pub from: String,
}

impl MusicMetadata {
    pub fn load_from_file(path: impl Into<String>) -> Result<Vec<Self>, anyhow::Error> {
        let path = path.into();

        let data = std::fs::read_to_string(&path)?;
        let data: Vec<Self> = serde_json::from_str(&data)?;

        info!("Loaded music metadta from file: {:?}", &path);

        Ok(data)
    }

    pub fn find_by_index(path: impl Into<String>, index: usize) -> Result<Self, anyhow::Error> {
        let metadata = MusicMetadata::load_from_file(path)?;
        let metadata = metadata
            .into_iter()
            .find(|metadata| metadata.index == index)
            .ok_or(anyhow!(
                "Could not find music metadata with index: {}",
                index
            ))?;

        Ok(metadata)
    }

    /// Appends the path in the json file to the provided path (should be from the public directory
    /// for the front end to read)
    pub fn get_src(&self, json_filepath: impl Into<String>) -> String {
        let mut path = PathBuf::from(json_filepath.into());
        path.push(&self.filename);

        format!("{}", path.display())
    }
}
