use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct ClientConfig {
    pub api_key: String,
}
