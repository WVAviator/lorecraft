use rand::Rng;

pub struct Random {}

impl Random {
    pub fn generate_id() -> String {
        let id = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>();
        return id;
    }
}
