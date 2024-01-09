#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ToolChoice {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    Function { name: String },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tool_choice_serializes() {
        let tool_choice = ToolChoice::None;
        let tool_choice_json = serde_json::to_string(&tool_choice).unwrap();
        assert_eq!(tool_choice_json, r#""none""#);

        let tool_choice = ToolChoice::Function {
            name: String::from("test"),
        };
        let tool_choice_json = serde_json::to_string(&tool_choice).unwrap();
        assert_eq!(tool_choice_json, r#"{"name":"test"}"#);
    }
}
