use serde::{Deserialize, Serialize};

use super::image_object::ImageObject;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateImageResponse {
    created: i64,
    data: Vec<ImageObject>,
}

impl Into<Vec<ImageObject>> for CreateImageResponse {
    fn into(self) -> Vec<ImageObject> {
        self.data
    }
}
