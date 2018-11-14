#[macro_use]
extern crate log;

mod config;
mod weather_api;

pub use self::config::*;
use self::weather_api::darksky::DarkSky;
use self::weather_api::owm::Owm;
use self::weather_api::{ForecastApi, Historical, WeatherApi};
use failure::Error;
use reqwest::Client;

pub fn run(config: &Config) -> Result<(), Error> {
    env_logger::init();
    info!("logging enabled");
    debug!("{:?}", config);

    let owm = Owm::new(&config);
    info!("{:?}", owm);
    let owm_url = owm.url();
    info!("owm url: {}", owm_url);

    let darksky = DarkSky::new(&config);
    info!("{:?}", darksky);
    let darksky_url = darksky.url();
    info!("darksky url: {}", darksky_url);

    // TODO: remove this feature once testing is setup
    if cfg!(feature = "live") == false {
        return Ok(());
    }

    let client = Client::builder().gzip(true).build()?;

    let owm_current_weather = owm.current(&client)?;
    info!("successfully retrieved owm current weather");
    trace!("{:?}", owm_current_weather);

    trace!("owm forecast url: {}", owm.forecast_url());
    let owm_forecast_weather = owm.forecast(&client)?;
    info!("successfully retrieved owm forecast weather");
    trace!("{:?}", owm_forecast_weather);

    let darksky_current_weather = darksky.current(&client)?;
    info!("successfully retrieved darksky current weather");
    trace!("{:?}", darksky_current_weather);

    if let Some(time) = std::env::args().nth(1) {
        let time = time.parse::<i64>()?;
        let darksky_historical_weather: serde_json::Value = darksky.historical(&client, time)?;
        info!("successfully retrieved darksky historical weather");
        trace!("{}", darksky_historical_weather);
    }

    Ok(())
}
