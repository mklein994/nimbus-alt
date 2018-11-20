#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

pub mod app;
mod config;
mod weather_api;

pub use self::config::*;
use self::weather_api::darksky::DarkSky;
use self::weather_api::owm::Owm;
use self::weather_api::{ForecastApi, HistoricalApi, WeatherApi};
use clap::ArgMatches;
use failure::Error;
use reqwest::Client;

pub fn run(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    env_logger::init();
    info!("logging enabled");
    debug!("{:?}", config);

    let owm = Owm::new(&config, &matches);
    info!("{:?}", owm);
    info!("owm url: {}", owm.url());

    let darksky = DarkSky::new(&config, &matches);
    debug!("{:?}", darksky);
    debug!("darksky url: {}", darksky.url());

    // TODO: maybe remove this flag once testing is setup
    if !matches.is_present("live") {
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

    if matches.is_present("time") {
        if let Ok(time) = value_t!(matches.value_of("time"), i64) {
            let darksky_historical_weather: serde_json::Value =
                darksky.historical(&client, time)?;
            info!("successfully retrieved darksky historical weather");
            trace!("{}", darksky_historical_weather);
        }
    }

    Ok(())
}
