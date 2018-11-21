use super::Config;
use clap::ArgMatches;
use failure::Error;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt;
use url::Url;

pub mod darksky;
pub mod owm;

fn fetch_json<T, E>(client: &Client, url: Url) -> Result<T, failure::Error>
where
    T: DeserializeOwned,
    E: DeserializeOwned + failure::Fail,
{
    client
        .get(url)
        .send()
        .map_err(Error::from)
        .and_then(|mut res| match res.status() {
            status if status.is_success() => Ok(res.json::<T>()?),
            status if status.is_client_error() => Err(res.json::<E>().map(Error::from)?),
            _ => Err(Error::from(res.error_for_status().unwrap_err())),
        })
}

pub trait WeatherApi<'a> {
    const BASE_URL: &'static str;
    type Current: std::fmt::Debug + DeserializeOwned;
    type ApiError: std::fmt::Debug + failure::Fail + DeserializeOwned;

    fn new(config: &'a Config, m: &'a ArgMatches) -> Self;
    fn url(&self) -> Url;
    fn current_url(&self) -> Url;

    fn current(&self, client: &Client) -> Result<Self::Current, Error> {
        fetch_json::<Self::Current, Self::ApiError>(client, self.current_url())
    }
}

pub trait HistoricalApi<'a>: WeatherApi<'a> {
    fn historical_url(&self, time: i64) -> Url;

    fn historical(&self, client: &Client, time: i64) -> Result<serde_json::Value, Error> {
        fetch_json::<serde_json::Value, Self::ApiError>(client, self.historical_url(time))
    }
}

pub trait ForecastApi<'a>: WeatherApi<'a> {
    type Forecast: fmt::Debug + DeserializeOwned;

    fn forecast_url(&self) -> Url;

    fn forecast(&self, client: &Client) -> Result<Self::Forecast, Error> {
        fetch_json::<Self::Forecast, Self::ApiError>(client, self.forecast_url())
    }
}
