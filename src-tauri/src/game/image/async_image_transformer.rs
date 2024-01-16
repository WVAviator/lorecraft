use futures::Future;

use super::Image;

/// This trait, when implemented for a struct that contains image prompts that need to be converted
/// into created images, allows the images to be visited and populated with the necessary data.
pub trait AsyncImageTransformer {
    async fn visit_images<C, F>(&mut self, transformer: C) -> Result<(), anyhow::Error>
    where
        C: Fn(Image) -> F + Send,
        F: Future<Output = Result<Image, anyhow::Error>> + Send;
}

#[cfg(test)]
mod test {
    use anyhow::bail;
    use futures::{StreamExt, TryStreamExt};

    use super::*;

    struct TestA {
        image: Image,
    }

    impl AsyncImageTransformer for TestA {
        async fn visit_images<C, F>(&mut self, transformer: C) -> Result<(), anyhow::Error>
        where
            C: Fn(Image) -> F + Send,
            F: Future<Output = Result<Image, anyhow::Error>> + Send,
        {
            self.image = transformer(self.image.clone()).await?;
            Ok(())
        }
    }

    struct TestB {
        images: Vec<Image>,
    }

    impl AsyncImageTransformer for TestB {
        async fn visit_images<C, F>(&mut self, transformer: C) -> Result<(), anyhow::Error>
        where
            C: Fn(Image) -> F + Send,
            F: Future<Output = Result<Image, anyhow::Error>> + Send,
        {
            let mut futures = Vec::new();
            for image in &mut self.images {
                let mut image = image.clone();
                let future = async {
                    image = transformer(image).await?;
                    Ok(image) as Result<Image, anyhow::Error>
                };
                futures.push(future);
            }
            let stream = futures::stream::iter(futures).buffered(3);
            let images = stream.try_collect::<Vec<_>>().await?;
            self.images = images;
            Ok(())
        }
    }

    #[tokio::test]
    async fn allows_visiting_single_image() {
        let transformer = |old_image| async move {
            Ok(match old_image {
                Image::Prompt(desc) => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

                    Image::Created {
                        src: String::from("abc.png"),
                        alt: desc,
                    }
                }
                _ => bail!("Image already created."),
            })
        };

        let mut test_a = TestA {
            image: Image::Prompt(String::from("Test")),
        };

        test_a.visit_images(transformer).await.unwrap();

        assert_eq!(
            test_a.image,
            Image::Created {
                src: String::from("abc.png"),
                alt: String::from("Test"),
            }
        );
    }

    #[tokio::test]
    async fn allows_visiting_multiple_images() {
        let transformer = |old_image| async move {
            Ok(match old_image {
                Image::Prompt(desc) => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                    Image::Created {
                        src: String::from("abc.png"),
                        alt: desc,
                    }
                }
                _ => bail!("Image already created."),
            })
        };

        let mut test_b = TestB {
            images: vec![
                Image::Prompt(String::from("Test")),
                Image::Prompt(String::from("Test2")),
                Image::Prompt(String::from("Test3")),
            ],
        };

        test_b.visit_images(transformer).await.unwrap();

        assert_eq!(
            test_b.images,
            vec![
                Image::Created {
                    src: String::from("abc.png"),
                    alt: String::from("Test"),
                },
                Image::Created {
                    src: String::from("abc.png"),
                    alt: String::from("Test2"),
                },
                Image::Created {
                    src: String::from("abc.png"),
                    alt: String::from("Test3"),
                },
            ]
        );
    }
}
