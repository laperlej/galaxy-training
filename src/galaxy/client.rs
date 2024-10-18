use crate::galaxy::config;
use std::future::Future;


pub struct Client {
    client: reqwest::Client,
    config: config::Config,
}

impl Client {
    pub fn new(config: config::Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }
    pub fn get(&self, endpoint: &str) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        self.client.get(format!("{}{}", self.config.galaxy_url, endpoint))
            .header("x-api-key", self.config.api_key.clone())
            .send()
    }

    pub fn post(&self, endpoint: &str, body: impl serde::Serialize) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        self.client.post(format!("{}{}", self.config.galaxy_url, endpoint))
            .header("x-api-key", self.config.api_key.clone())
            .json(&body)
            .send()
    }

    pub fn put(&self, endpoint: &str, body: impl serde::Serialize) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        self.client.put(format!("{}{}", self.config.galaxy_url, endpoint))
            .header("x-api-key", self.config.api_key.clone())
            .json(&body)
            .send()
    }
}
