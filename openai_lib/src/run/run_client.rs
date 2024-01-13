use crate::Error;

use super::{CreateRunRequest, RunObject, SubmitToolOutputsRequest};

#[trait_variant::make(RunClient: Send)]
pub trait LocalRunClient {
    async fn create_run(
        &self,
        request: CreateRunRequest,
        thread_id: &str,
    ) -> Result<RunObject, Error>;

    async fn retrieve_run(&self, thread_id: &str, run_id: &str) -> Result<RunObject, Error>;

    async fn submit_tool_outputs(
        &self,
        request: SubmitToolOutputsRequest,
        thread_id: &str,
        run_is: &str,
    ) -> Result<RunObject, Error>;
}
