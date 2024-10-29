//! Galaxy API client module
//!
//! This module provides a client for interacting with the Galaxy API.

use crate::galaxy::config;
use std::future::Future;

/// A client for interacting with the Galaxy API
///
/// # Examples
///
/// ```
/// use crate::galaxy::config::Config;
/// use crate::galaxy::client::Client;
///
/// let config = Config {
///     galaxy_url: "https://api.example.com".to_string(),
///     api_key: "your-api-key".to_string(),
/// };
/// let client = Client::new(config);
/// ```
pub struct Client {
    client: reqwest::Client,
    config: config::Config,
}

impl Client {
    /// Creates a new `Client` instance
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the Galaxy API
    pub fn new(config: config::Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    /// Sends a GET request to the specified endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::galaxy::config::Config;
    /// # use crate::galaxy::client::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = Config {
    /// #     galaxy_url: "https://api.example.com".to_string(),
    /// #     api_key: "your-api-key".to_string(),
    /// # };
    /// # let client = Client::new(config);
    /// let response = client.get("/users").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, endpoint: &str) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        self.client.get(format!("{}{}", self.config.galaxy_url, endpoint))
            .header("x-api-key", self.config.api_key.clone())
            .send()
    }

    /// Sends a POST request to the specified endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to
    /// * `body` - The request body to send
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::galaxy::config::Config;
    /// # use crate::galaxy::client::Client;
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = Config {
    /// #     galaxy_url: "https://api.example.com".to_string(),
    /// #     api_key: "your-api-key".to_string(),
    /// # };
    /// # let client = Client::new(config);
    /// let body = json!({
    ///     "name": "John Doe",
    ///     "email": "john@example.com"
    /// });
    /// let response = client.post("/users", body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn post(&self, endpoint: &str, body: impl serde::Serialize) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        self.client.post(format!("{}{}", self.config.galaxy_url, endpoint))
            .header("x-api-key", self.config.api_key.clone())
            .json(&body)
            .send()
    }

    /// Sends a PUT request to the specified endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to
    /// * `body` - The request body to send
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::galaxy::config::Config;
    /// # use crate::galaxy::client::Client;
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = Config {
    /// #     galaxy_url: "https://api.example.com".to_string(),
    /// #     api_key: "your-api-key".to_string(),
    /// # };
    /// # let client = Client::new(config);
    /// let body = json!({
    ///     "name": "John Doe Updated",
    ///     "email": "john.updated@example.com"
    /// });
    /// let response = client.put("/users/1", body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn put(&self, endpoint: &str, body: impl serde::Serialize) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        self.client.put(format!("{}{}", self.config.galaxy_url, endpoint))
            .header("x-api-key", self.config.api_key.clone())
            .json(&body)
            .send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::runtime::Runtime;

    fn setup_test() -> (mockito::ServerGuard, Client, Runtime) {
        let server = mockito::Server::new();
        let config = config::Config {
            galaxy_url: server.url(),
            api_key: "test-api-key".to_string(),
        };
        let client = Client::new(config);
        let runtime = Runtime::new().unwrap();
        (server, client, runtime)
    }

    #[test]
    fn test_get() {
        let (mut server, client, runtime) = setup_test();
        let mock = server.mock("GET", "/users")
            .match_header("x-api-key", "test-api-key")
            .with_status(200)
            .with_body(r#"{"users": []}"#)
            .create();

        runtime.block_on(async {
            let response = client.get("/users").await.unwrap();
            assert_eq!(response.status(), 200);
        });

        mock.assert();
    }

    #[test]
    fn test_post() {
        let (mut server, client, runtime) = setup_test();
        let mock = server.mock("POST", "/users")
            .match_header("x-api-key", "test-api-key")
            .match_body(mockito::Matcher::Json(json!({"name":"John Doe","email":"john@example.com"})))
            .with_status(201)
            .with_body(r#"{"id": 1, "name": "John Doe", "email": "john@example.com"}"#)
            .create();

        runtime.block_on(async {
            let body = json!({
                "name": "John Doe",
                "email": "john@example.com"
            });
            let response = client.post("/users", body).await.unwrap();
            assert_eq!(response.status(), 201);
        });

        mock.assert();
    }

    #[test]
    fn test_put() {
        let (mut server, client, runtime) = setup_test();
        let mock = server.mock("PUT", "/users/1")
            .match_header("x-api-key", "test-api-key")
            .match_body(mockito::Matcher::Json(json!({"name":"John Doe Updated","email":"john.updated@example.com"})))
            .with_status(200)
            .with_body(r#"{"id": 1, "name": "John Doe Updated", "email": "john.updated@example.com"}"#)
            .create();

        runtime.block_on(async {
            let body = json!({
                "name": "John Doe Updated",
                "email": "john.updated@example.com"
            });
            let response = client.put("/users/1", body).await.unwrap();
            assert_eq!(response.status(), 200);
        });

        mock.assert();
    }
}
