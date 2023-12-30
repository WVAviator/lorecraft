#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum ImageGenerationModel {
    Dall_E_2,
    Dall_E_3,
}

impl ImageGenerationModel {
    pub fn to_string(&self) -> String {
        match self {
            ImageGenerationModel::Dall_E_2 => "dall-e-2",
            ImageGenerationModel::Dall_E_3 => "dall-e-3",
        }
        .to_string()
    }
}
