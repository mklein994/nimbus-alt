use super::Config;
use clap::ArgMatches;
use failure::Error;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt;
use url::Url;

pub mod darksky;
pub mod owm;

pub trait WeatherApi<'a> {
    const BASE_URL: &'static str;
    type Current: std::fmt::Debug + DeserializeOwned;

    fn new(config: &'a Config, m: &'a ArgMatches) -> Self;
    fn url(&self) -> Url;
    fn current_url(&self) -> Url;
    fn current(&self, client: &Client) -> Result<Self::Current, Error> {
        client.get(self.url()).send()?.json().map_err(Error::from)
    }
}

pub trait HistoricalApi<'a>: WeatherApi<'a> {
    fn historical_url(&self, time: i64) -> Url;

    fn historical(&self, client: &Client, time: i64) -> Result<serde_json::Value, Error> {
        client
            .get(self.historical_url(time))
            .send()?
            .json()
            .map_err(Error::from)
    }
}

pub trait ForecastApi<'a>: WeatherApi<'a> {
    type Forecast: fmt::Debug + DeserializeOwned;

    fn forecast_url(&self) -> Url;

    fn forecast(&self, client: &Client) -> Result<Self::Forecast, Error> {
        client
            .get(self.forecast_url())
            .send()?
            .json()
            .map_err(Error::from)
    }
}
