use crate::config::config::core_config;
use crate::error::Result;
use reqwest::{header, Client, ClientBuilder};

pub struct ResManager(Client);

impl ResManager {
    pub async fn new() -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_static(&core_config().API_KEY),
        );
        Ok(ResManager(
            ClientBuilder::new().default_headers(headers).build()?,
        ))
    }
    pub fn store(&self) -> &Client {
        &self.0
    }
}
