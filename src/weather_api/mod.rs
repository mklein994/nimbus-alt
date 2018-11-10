use super::Config;
use serde_derive::Deserialize;
use url::Url;

pub mod darksky;
pub mod owm;

#[derive(Debug, PartialEq)]
pub enum Location {
    Coord(f64, f64),
    Id(String),
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GenericWeatherUnit {
    Metric,
    Imperial,
    Si,
}

pub trait WeatherApi: Sized {
    const BASE_URL: &'static str;

    fn new(config: &Config) -> Self;
    fn current_url(&self) -> Url;
}
