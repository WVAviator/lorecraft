pub struct ListMessagesQuery {
    pub url: String,
}

impl ListMessagesQuery {
    pub fn builder(thread_id: &str) -> ListMessagesQueryBuilder {
        ListMessagesQueryBuilder::new(thread_id)
    }
}

pub struct ListMessagesQueryBuilder {
    url: String,
    query_params_count: u32,
}

impl ListMessagesQueryBuilder {
    pub fn new(thread_id: &str) -> Self {
        ListMessagesQueryBuilder {
            url: format!("https://api.openai.com/v1/threads/{}/messages", thread_id),
            query_params_count: 0,
        }
    }

    fn add_param(&mut self, key: &str, value: &str) {
        let prefix = match self.query_params_count {
            0 => "?",
            _ => "&",
        };
        self.query_params_count += 1;
        self.url = format!("{}{}{}={}", self.url, prefix, key, value);
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.add_param("limit", format!("{}", limit).as_str());
        self
    }

    pub fn order(mut self, order: &str) -> Self {
        self.add_param("order", order);
        self
    }

    pub fn after(mut self, after: &str) -> Self {
        self.add_param("after", after);
        self
    }

    pub fn before(mut self, before: &str) -> Self {
        self.add_param("before", before);
        self
    }

    pub fn build(self) -> ListMessagesQuery {
        ListMessagesQuery { url: self.url }
    }
}
