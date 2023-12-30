#[derive(Debug, Clone)]
pub enum ImageGenerationSize {
    Size256x256,
    Size512x512,
    Size1024x1024,
    Size1024x1792,
    Size1792x1024,
}

impl ImageGenerationSize {
    pub fn to_string(&self) -> String {
        match self {
            ImageGenerationSize::Size256x256 => "256x256",
            ImageGenerationSize::Size512x512 => "512x512",
            ImageGenerationSize::Size1024x1024 => "1024x1024",
            ImageGenerationSize::Size1024x1792 => "1024x1792",
            ImageGenerationSize::Size1792x1024 => "1792x1024",
        }
        .to_string()
    }
}
