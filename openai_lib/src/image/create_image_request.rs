use crate::{model::image_model::ImageModel, Error};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{
    image_quality::ImageQuality, image_size::ImageSize, image_style::ImageStyle,
    response_format::ResponseFormat,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TypedBuilder)]
pub struct CreateImageRequest {
    #[builder(setter(into))]
    prompt: String,
    #[builder(default = ImageModel::DallE2)]
    model: ImageModel,
    #[builder(default, setter(strip_option))]
    n: Option<u8>,
    #[builder(default, setter(strip_option))]
    quality: Option<ImageQuality>,
    #[builder(default, setter(strip_option), mutators(
        pub fn b64_json(&mut self) {
            self.response_format = Some(ResponseFormat::B64Json);
        }
    ), via_mutators)]
    response_format: Option<ResponseFormat>,
    #[builder(default, setter(strip_option))]
    size: Option<ImageSize>,
    #[builder(default, setter(strip_option))]
    style: Option<ImageStyle>,
    #[builder(default, setter(strip_option))]
    user: Option<String>,
}

impl CreateImageRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        self.validate()?;

        let json =
            serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))?;

        Ok(json)
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }

    pub fn modify_prompt<T>(&mut self, transformer: T)
    where
        T: Fn(&str) -> String,
    {
        self.prompt = transformer(&self.prompt);
    }

    pub fn modify_response_format(&mut self, response_format: ResponseFormat) {
        self.response_format = Some(response_format);
    }

    fn validate(&self) -> Result<(), Error> {
        match self {
            CreateImageRequest {
                model: ImageModel::DallE2,
                quality: Some(ImageQuality::HD),
                ..
            } => Err(Error::InvalidRequestField(String::from(
                "DallE2 model not compatible with HD quality option.",
            ))),
            CreateImageRequest {
                model: ImageModel::DallE2,
                size: Some(ImageSize::Size1792x1024) | Some(ImageSize::Size1024x1792),
                ..
            } => Err(Error::InvalidRequestField(String::from(
                "DallE2 is not compatible with image sizes larger than 1024x1024.",
            ))),
            CreateImageRequest {
                model: ImageModel::DallE3,
                size: Some(ImageSize::Size256x256) | Some(ImageSize::Size512x512),
                ..
            } => Err(Error::InvalidRequestField(String::from(
                "DallE3 is not compatible with image sizes smaller than 1024x1024.",
            ))),
            CreateImageRequest {
                model: ImageModel::DallE2,
                style: Some(_),
                ..
            } => Err(Error::InvalidRequestField(String::from(
                "DallE2 is not compatible with style option.",
            ))),
            CreateImageRequest {
                model: ImageModel::DallE2,
                prompt,
                ..
            } => {
                if prompt.chars().count() > 1000 {
                    return Err(Error::InvalidRequestField(String::from(
                        "DallE2 prompt is limited to 1000 characters.",
                    )));
                }

                Ok(())
            }
            CreateImageRequest {
                model: ImageModel::DallE3,
                prompt,
                ..
            } => {
                if prompt.chars().count() > 4000 {
                    return Err(Error::InvalidRequestField(String::from(
                        "DallE3 prompt is limited to 4000 characters.",
                    )));
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;
    #[test]
    fn image_request_builder() {
        let image_request = CreateImageRequest::builder()
            .prompt("Hello")
            .model(ImageModel::DallE3)
            .build();

        assert_eq!(image_request.prompt, "Hello");
        assert_eq!(image_request.model, ImageModel::DallE3);
        assert_eq!(image_request.n, None);
    }

    #[test]
    fn request_serializes_correctly() {
        let expected = json!({
          "model": "dall-e-3",
          "prompt": "A cute baby sea otter",
          "n": 1,
          "size": "1024x1024"
        });
        let actual = CreateImageRequest::builder()
            .model(ImageModel::DallE3)
            .prompt("A cute baby sea otter")
            .n(1)
            .size(ImageSize::Size1024x1024)
            .build();
        let actual = serde_json::to_value(&actual).unwrap();

        assert_json_include!(actual: actual, expected: expected);
    }

    #[test]
    fn detects_invalid_image_size() {
        let request = CreateImageRequest::builder()
            .prompt("A cute baby sea otter")
            .model(ImageModel::DallE2)
            .size(ImageSize::Size1792x1024)
            .build();

        assert!(request.validate().is_err());
    }

    #[test]
    fn detects_prompt_too_long() {
        let request = CreateImageRequest::builder()
            .prompt("a".repeat(1001))
            .model(ImageModel::DallE2)
            .build();

        assert!(request.validate().is_err());
    }

    #[test]
    fn allows_valid_request() {
        let request = CreateImageRequest::builder()
            .prompt("A cute baby sea otter")
            .model(ImageModel::DallE2)
            .size(ImageSize::Size1024x1024)
            .build();

        assert!(request.validate().is_ok());
    }
}
