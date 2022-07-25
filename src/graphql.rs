use anyhow::Result;
use reqwest::blocking::Client;

#[derive(Clone)]
pub struct GraphqlClient {
    pub url: &'static str,
    pub client: Client,
}

impl GraphqlClient {
    pub fn new(url: &'static str) -> Result<GraphqlClient, anyhow::Error> {
        let client = Client::builder()
            .user_agent("graphql-rust/0.10.0")
            .build()?;
        Ok(GraphqlClient { url, client })
    }
}
