use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::{fs::OpenOptions, path::PathBuf};

use anyhow::Context;
use fs2::FileExt;
use futures::Future;
use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::PathResolver;

#[derive(Debug, Clone)]
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

    pub fn new_custom(directory_path: impl AsRef<Path>) -> Result<FileManager, anyhow::Error> {
        let data_dir = directory_path.as_ref().to_path_buf();

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir).with_context(|| {
                format!(
                    "Unable to create game directory at {} for local data files.",
                    &data_dir.display()
                )
            })?;
        }

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

    pub async fn json_transaction<T, F>(
        &self,
        file_name: impl Into<String>,
        transaction: F,
    ) -> std::io::Result<()>
    where
        F: FnOnce(T) -> T + Send + 'static,
        T: DeserializeOwned + Serialize + Send + 'static,
    {
        let data_dir = self.data_dir.clone();
        let file_path: PathBuf = data_dir.join(file_name.into());

        tokio::task::spawn_blocking(move || {
            FileManager::json_transaction_blocking(file_path, transaction)
        })
        .await?
    }

    pub fn write_json<T>(
        &self,
        file_name: impl Into<String>,
        content: &T,
    ) -> Result<(), anyhow::Error>
    where
        T: Serialize,
    {
        let file_path: PathBuf = self.data_dir.join(file_name.into());

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)
            .context("Unable to open the specified JSON file.")?;

        serde_json::to_writer(&file, content).context("Unable to serialize and write JSON to file.")
    }

    pub fn read_json<T>(&self, file_name: impl Into<String>) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let file_path: PathBuf = self.data_dir.join(file_name.into());

        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(&file_path)
            .context("Unable to open the specified JSON file.")?;

        serde_json::from_reader::<&File, T>(&file)
            .context("Unable to deserialize the requested JSON file.")
    }

    fn json_transaction_blocking<T, F>(
        file_path: impl AsRef<Path>,
        transaction: F,
    ) -> std::io::Result<()>
    where
        F: FnOnce(T) -> T,
        T: DeserializeOwned + Serialize,
    {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(&file_path)?;

        file.lock_exclusive()?;

        let data = serde_json::from_reader::<&File, T>(&file)?;

        let modified_data = transaction(data);

        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        serde_json::to_writer(&file, &modified_data)?;

        file.unlock()?;

        Ok(())
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

    pub fn file_exists(&self, file_name: &str) -> Result<bool, anyhow::Error> {
        let file_path: PathBuf = self.data_dir.join(file_name);

        file_path
            .try_exists()
            .context("File existence could not be verfied.")
    }

    pub fn read_from_file(&self, file_name: &str) -> Result<String, anyhow::Error> {
        let file_path: PathBuf = self.data_dir.join(file_name);

        let contents = std::fs::read_to_string(file_path).context("Unable to read from file.")?;

        Ok(contents)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Test {
        pub a: Option<String>,
        pub b: Option<String>,
    }

    fn get_fm() -> FileManager {
        FileManager::new_custom("./test_data").unwrap()
    }

    fn before_each() {
        let file_manager = get_fm();

        let initial_test = Test { a: None, b: None };

        file_manager
            .write_to_file(
                "transaction.json",
                &serde_json::to_string(&initial_test).unwrap(),
            )
            .unwrap();
    }

    fn after_each() {
        let file_manager = get_fm();
        file_manager.write_to_file("transaction.json", "").unwrap();
    }

    #[tokio::test]
    async fn json_transaction_expected_api() {
        let file_manager = get_fm();

        before_each();

        file_manager
            .json_transaction::<Test, _>("transaction.json", |mut test| {
                test.a = Some(String::from("hello"));
                test.b = Some(String::from("world"));
                test
            })
            .await
            .unwrap();

        let result = file_manager.read_from_file("transaction.json").unwrap();
        let result: Test = serde_json::from_str(&result).unwrap();

        assert_eq!(result.a, Some(String::from("hello")));
        assert_eq!(result.b, Some(String::from("world")));

        after_each();
    }

    #[tokio::test]
    async fn json_transaction_concurrent_writes() {
        let file_manager = get_fm();

        before_each();

        let future1 = file_manager.json_transaction::<Test, _>("transaction.json", |mut test| {
            test.a = Some(String::from("hello"));
            test
        });
        let future2 = file_manager.json_transaction::<Test, _>("transaction.json", |mut test| {
            test.b = Some(String::from("world"));
            test
        });

        let (result1, result2) = futures::join!(future1, future2);
        result1.unwrap();
        result2.unwrap();

        let result = file_manager.read_from_file("transaction.json").unwrap();
        let result: Test = serde_json::from_str(&result).unwrap();

        assert_eq!(result.a, Some(String::from("hello")));
        assert_eq!(result.b, Some(String::from("world")));
    }
}
