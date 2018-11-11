use super::Config;
use serde_derive::Deserialize;
use url::Url;

pub mod darksky;
pub mod owm;

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GenericWeatherUnit {
    Metric,
    Imperial,
}

pub trait WeatherApi: Sized {
    const BASE_URL: &'static str;

    fn new_url(config: &Config) -> Url;
}
