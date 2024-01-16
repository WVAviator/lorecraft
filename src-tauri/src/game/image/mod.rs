use serde::{Deserialize, Serialize};

pub mod async_image_transformer;
pub mod image_factory;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Image {
    Prompt(String),
    Created { src: String, alt: String },
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn can_be_either_prompt_or_image() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct A {
            test: Image,
        }

        let json_as_str = json!({
            "test": "string"
        })
        .to_string();

        let json_as_image = json!({
            "test": {
            "src": "string",
            "alt": "string"
            }
        })
        .to_string();

        let a: A = serde_json::from_str(&json_as_str).unwrap();
        let b: A = serde_json::from_str(&json_as_image).unwrap();

        assert_eq!(a.test, Image::Prompt(String::from("string")));
        assert_eq!(
            b.test,
            Image::Created {
                src: String::from("string"),
                alt: String::from("string")
            }
        );
    }
}
