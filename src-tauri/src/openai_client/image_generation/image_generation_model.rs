#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum ImageGenerationModel {
    DallE2,
    DallE3,
}

impl ImageGenerationModel {
    pub fn to_string(&self) -> String {
        match self {
            ImageGenerationModel::DallE2 => "dall-e-2",
            ImageGenerationModel::DallE3 => "dall-e-3",
        }
        .to_string()
    }
}
