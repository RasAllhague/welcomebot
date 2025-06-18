use std::fmt::Debug;

use reqwest::RequestBuilder;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiscordClientError {
    #[error("web request to discord failed: {0}")]
    Web(#[from] reqwest::Error),
    #[error("web request failed with status {0}")]
    RequestError(reqwest::StatusCode),
    #[error("failed to parse json: {0}")]
    Serde(#[from] serde_json::Error),
}

pub trait ResourceRequest {
    type Response: for<'de> Deserialize<'de> + Clone + Debug + Send;

    fn token(&self) -> &String;
    fn url(&self) -> &url::Url;
    fn build(
        &self,
        req_builder: RequestBuilder,
    ) -> impl std::future::Future<Output = Result<RequestBuilder, DiscordClientError>>;
}

#[derive(Clone, Debug)]
pub struct DiscordClient {
    client: reqwest::Client,
}

impl DiscordClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_resource<R: ResourceRequest>(
        &self,
        req: R,
    ) -> Result<R::Response, DiscordClientError> {
        let req_builder = self.client.get(req.url().clone()).bearer_auth(req.token());

        let resp = req.build(req_builder)
            .await?
            .send()
            .await?;

        let data = match resp.status() {
            reqwest::StatusCode::OK => resp.json::<R::Response>().await?,
            _ => return Err(DiscordClientError::RequestError(resp.status())),
        };

        Ok(data)
    }
}
