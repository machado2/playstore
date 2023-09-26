use crate::errors::{Error, ErrorKind};
use serde_json::Value as JsonValue;
use std::env;
pub struct GooglePlayApi {
    client: reqwest::Client,
    base_url: String,
}

impl GooglePlayApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: env::var("GPLAY_API_URL").expect("GPLAY_API_URL must be set"),
        }
    }

    async fn get_json(&self, url: &str) -> Result<JsonValue, Error> {
        let response = self.client.get(url).send().await?;
        match response.status() {
            reqwest::StatusCode::NOT_FOUND => Err(Error::from_kind(ErrorKind::HttpNotFound)),
            _ => Ok(response
                .json()
                .await?)
        }
    }

    pub async fn get_category(&self, category: &str) -> Result<Vec<String>, Error> {
        let url = format!("{}/apps?category={}", self.base_url, category);
        let json = self.get_json(&url).await?;
        let results = json["results"].as_array();
        let ids = results
            .ok_or_else(|| format!("No results found on category search for {}", category))?
            .iter()
            .filter_map(|x| x["appId"].as_str().map(String::from))
            .collect::<Vec<String>>();
        Ok(ids)
    }

    pub async fn get_similar(&self, id: &str) -> Result<Vec<String>, Error> {
        let url = format!("{}/apps/{}/similar", self.base_url, id);
        let json = self.get_json(&url).await?;
        let results = json["results"].as_array();
        let ids = results
            .ok_or_else(|| format!("No results found on similar search for {}", id))?
            .iter()
            .filter_map(|x| x["appId"].as_str().map(String::from))
            .collect::<Vec<String>>();
        Ok(ids)
    }

    pub async fn get_from_developer(&self, dev_id: &str) -> Result<Vec<String>, Error> {
        let url = format!("{}/developers/{}", self.base_url, dev_id);
        let json = self.get_json(&url).await?;
        let results = json["apps"].as_array();
        let ids = results
            .ok_or_else(|| format!("No results found on developer search for {}", dev_id))?
            .iter()
            .filter_map(|x| x["appId"].as_str().map(String::from))
            .collect::<Vec<String>>();
        Ok(ids)
    }

    pub async fn get_app(&self, id: &str) -> Result<JsonValue, Error> {
        self.get_json(&format!("{}/apps/{}", self.base_url, id))
            .await
    }
}
