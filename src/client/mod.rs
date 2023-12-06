mod billing;
mod instance;
mod dns;
mod account;

use std::time::Duration;
use reqwest::header::AUTHORIZATION;
use anyhow::Result;
use serde::de::DeserializeOwned;

pub struct VultrClient {
    http_client: reqwest::Client,
    url_prefix: String,
    auth_header: String,
}

impl VultrClient {
    pub fn new(url_prefix: String, token: String, timeout: Duration) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .unwrap();

        let auth_header = format!("Bearer {}", token);
        Self {http_client, url_prefix, auth_header}
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str, params: &[(&str, Option<&str>)]) -> Result<T> {
        let param_vec = params.into_iter().filter(|(_, v)| v.is_some())
            .copied().collect::<Vec<(&str, Option<&str>)>>();
        let url = format!("{}{}", self.url_prefix, path);
        let res = self.http_client
            .get(url)
            .query(&param_vec[..])
            .header(AUTHORIZATION, &self.auth_header)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }
}