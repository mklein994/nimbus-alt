use super::Config;
use url::Url;

pub mod darksky;
pub mod owm;

pub trait WeatherApi<'a>: Sized {
    const BASE_URL: &'static str;

    fn new(config: &'a Config) -> Self;
    fn url(self) -> Url;
}
