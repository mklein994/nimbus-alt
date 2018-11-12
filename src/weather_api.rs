use super::Config;
use url::Url;

pub mod darksky;
pub mod owm;

pub trait WeatherApi: Sized {
    const BASE_URL: &'static str;

    fn new_url(config: &Config) -> Url;
}
