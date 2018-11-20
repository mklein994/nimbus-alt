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
use env_logger::Builder;
use failure::Error;
use log::LevelFilter;
use reqwest::Client;

pub fn run(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    Builder::from_default_env()
        .filter(
            Some(&crate_name!().replace("-", "_")),
            match matches.occurrences_of("verbose") {
                1 => LevelFilter::Info,
                2 => LevelFilter::Debug,
                3 => LevelFilter::Trace,
                _ => LevelFilter::Off,
            },
        )
        .init();
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

    let client = Client::new();

    let darksky_current: self::weather_api::darksky::Forecast = client
        .get(darksky.url())
        .send()
        .and_then(|mut res| res.json())?;
    trace!("{:#?}", darksky_current);

    Ok(())
}
