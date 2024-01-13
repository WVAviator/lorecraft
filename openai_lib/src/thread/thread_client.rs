use crate::Error;

use super::{CreateThreadRequest, DeleteThreadResponse, ThreadObject};

#[trait_variant::make(ThreadClient: Send)]
pub trait LocalThreadClient {
    async fn create_thread(&self, request: CreateThreadRequest) -> Result<ThreadObject, Error>;
    async fn delete_thread(&self, thread_id: &str) -> Result<DeleteThreadResponse, Error>;
}
