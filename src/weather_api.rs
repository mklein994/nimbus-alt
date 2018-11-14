use super::Config;
use failure::Error;
use reqwest::Client;
use url::Url;

pub mod darksky;
pub mod owm;

pub trait WeatherApi<'a>: Sized {
    const BASE_URL: &'static str;
    type Current: std::fmt::Debug + serde::de::DeserializeOwned;

    fn new(config: &'a Config) -> Self;
    fn url(&self) -> Url;
    fn current(&self, client: &Client) -> Result<Self::Current, Error> {
        client.get(self.url()).send()?.json().map_err(Error::from)
    }
}

pub trait Historical<'a>: WeatherApi<'a> {
    fn historical_url(&self, time: i64) -> Url;

    fn historical(&self, client: &Client, time: i64) -> Result<serde_json::Value, Error> {
        client
            .get(self.historical_url(time))
            .send()?
            .json()
            .map_err(Error::from)
    }
}
