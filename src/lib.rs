#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate strum_macros;

pub mod app;
mod config;
mod weather_api;

pub use self::config::*;
use self::weather_api::darksky::DarkSky;
use self::weather_api::owm::Owm;
pub use self::weather_api::{ForecastApi, HistoricalApi, WeatherApi};
use clap::ArgMatches;
use env_logger::Builder;
use failure::Error;
use log::LevelFilter;
use reqwest::Client;

pub fn run(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let arg_filter = if std::env::var("RUST_LOG").is_ok() || matches.occurrences_of("verbose") == 0
    {
        Builder::from_default_env().build().filter()
    } else {
        match matches.occurrences_of("verbose") {
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            3 => LevelFilter::Trace,
            _ => {
                clap::Error::with_description(
                    "--verbose (-v) can be passed up to 3 times.",
                    clap::ErrorKind::TooManyValues,
                )
                .exit();
            }
        }
    };

    Builder::from_default_env()
        .filter(Some(&crate_name!().replace("-", "_")), arg_filter)
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

    darksky.current(&client).map(|forecast| {
        println!("{:?}", forecast);
    })?;

    owm.current(&client).map(|forecast| {
        println!("{:?}", forecast);
    })?;

    Ok(())
}
