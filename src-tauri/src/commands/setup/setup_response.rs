use super::setup_error::SetupError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupSuccessResponse {
    success: bool,
}

impl SetupSuccessResponse {
    pub fn new() -> Self {
        SetupSuccessResponse { success: true }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupFailureResponse {
    success: bool,
    error: String,
    message: String,
}

impl SetupFailureResponse {
    pub fn new(setup_error: SetupError) -> SetupFailureResponse {
        SetupFailureResponse {
            success: false,
            error: setup_error.get_type(),
            message: setup_error.get_message(),
        }
    }
}
