use std::io::Write;
use std::{fs::OpenOptions, path::PathBuf};

use anyhow::Context;
use log::info;
use tauri::PathResolver;

pub struct FileManager {
    data_dir: PathBuf,
}

impl FileManager {
    pub fn new(path_resolver: &PathResolver) -> Result<FileManager, anyhow::Error> {
        let data_dir = path_resolver
            .app_local_data_dir()
            .context("Unable to resolve app local data directory.")?;

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir).with_context(|| {
                format!(
                    "Unable to create game directory at {} for local data files.",
                    &data_dir.display()
                )
            })?;
        }

        info!("Verified data directory: {:?}", data_dir);

        Ok(FileManager { data_dir })
    }

    pub fn write_to_file(&self, file_name: &str, contents: &str) -> std::io::Result<String> {
        let mut open_options = OpenOptions::new();
        open_options.write(true).create(true).truncate(true);

        let file_path: PathBuf = self.data_dir.join(file_name);
        if let Some(dir_path) = file_path.parent() {
            std::fs::create_dir_all(dir_path)?;
        }

        info!("Writing to file: {:?}", file_path);
        let file_path_string = file_path.to_str().unwrap().to_string();

        let mut file = open_options.open(file_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(file_path_string)
    }

    pub fn write_bytes_to_file(
        &self,
        file_name: &str,
        contents: Vec<u8>,
    ) -> std::io::Result<String> {
        let mut open_options = OpenOptions::new();
        open_options.write(true).create(true).truncate(true);

        let file_path: PathBuf = self.data_dir.join(file_name);
        if let Some(dir_path) = file_path.parent() {
            std::fs::create_dir_all(dir_path)?;
        }

        info!("Writing to file: {:?}", file_path);
        let file_path_string = file_path.to_str().unwrap().to_string();

        let mut file = open_options.open(file_path)?;
        file.write_all(&contents)?;

        Ok(file_path_string)
    }

    pub fn append_to_file(&self, file_name: &str, contents: &str) -> std::io::Result<()> {
        let mut open_options = OpenOptions::new();
        open_options.write(true).create(true).append(true);

        let file_path: PathBuf = self.data_dir.join(file_name);

        let mut file = open_options.open(file_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }

    pub fn read_from_file(&self, file_name: &str) -> std::io::Result<String> {
        let file_path: PathBuf = self.data_dir.join(file_name);

        let contents = std::fs::read_to_string(file_path)?;

        Ok(contents)
    }

    pub fn delete_file(&self, file_name: &str) -> std::io::Result<()> {
        let file_path: PathBuf = self.data_dir.join(file_name);

        std::fs::remove_file(file_path)?;

        Ok(())
    }

    pub fn delete_dir(&self, dir_name: &str) -> std::io::Result<()> {
        let dir_path: PathBuf = self.data_dir.join(dir_name);

        std::fs::remove_dir_all(dir_path)?;

        Ok(())
    }
}
