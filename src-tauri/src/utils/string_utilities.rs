pub struct StringUtilities {}

impl StringUtilities {
    pub fn truncate(s: &str, max_chars: usize) -> String {
        match s.char_indices().nth(max_chars) {
            None => s.to_string(),
            Some((idx, _)) => format!("{}...", &s[..idx]),
        }
    }
}
