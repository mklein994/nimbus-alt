use super::Config;

pub mod darksky;
pub mod owm;

pub trait WeatherApi: Sized {
    const BASE_URL: &'static str;

    fn new(config: &Config) -> Self;
}
