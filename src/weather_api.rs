use super::Config;
use reqwest::Client;
use url::Url;

pub mod darksky;
pub mod owm;

pub trait WeatherApi<'a>: Sized {
    const BASE_URL: &'static str;

    fn new(config: &'a Config) -> Self;
    fn url(&self) -> Url;
    fn current(&self, client: &Client) -> Result<serde_json::Value, failure::Error> {
        client.get(self.url()).send()?.json().map_err(|e| e.into())
    }
}
