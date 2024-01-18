use std::fmt::Display;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct ListMessagesRequest {
    limit: Option<u8>,
    order: Option<MessageSortOrder>,
    #[builder(setter(into))]
    after: Option<String>,
    #[builder(setter(into))]
    before: Option<String>,
}

impl ListMessagesRequest {
    // pub fn to_json_body(self) -> Result<String, Error> {
    //     self.validate()?;
    //     serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    // }

    pub fn build_url(self, base_url: impl Into<String>) -> Result<String, Error> {
        self.validate()?;

        let mut url =
            Url::parse(&base_url.into()).map_err(|e| Error::SerializationFailure(e.into()))?;

        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        if let Some(order) = self.order {
            url.query_pairs_mut()
                .append_pair("order", &order.to_string());
        }

        if let Some(after) = self.after {
            url.query_pairs_mut().append_pair("after", &after);
        }

        if let Some(before) = self.before {
            url.query_pairs_mut().append_pair("before", &before);
        }

        Ok(url.to_string())
    }

    fn validate(&self) -> Result<(), Error> {
        if let Some(limit) = self.limit {
            if limit > 100 || limit < 1 {
                return Err(Error::InvalidRequestField(String::from(
                    "Limit must be between 1 and 100.",
                )));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageSortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Display for MessageSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageSortOrder::Ascending => write!(f, "asc"),
            MessageSortOrder::Descending => write!(f, "desc"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builds_url_properly() {
        let base_url = format!("https://api.openai.com/v1/threads/abc/messages");
        let list_messages_request = ListMessagesRequest::builder()
            .limit(25)
            .order(MessageSortOrder::Ascending)
            .after("ab3c3")
            .build();

        let url = list_messages_request.build_url(base_url).unwrap();

        assert_eq!(
            url,
            "https://api.openai.com/v1/threads/abc/messages?limit=25&order=asc&after=ab3c3"
        );
    }
}
