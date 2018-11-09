#[macro_use]
extern crate log;

mod config;
mod weather_api;

pub use self::config::Config;
use self::weather_api::darksky::{DarkSkyApi, DarkSkyUnit};
use self::weather_api::owm::OwmApi;
use self::weather_api::{Location, WeatherApi};
use failure::Error;
use reqwest::Client;
use url::Url;

pub fn run(config: &Config) -> Result<(), Error> {
    env_logger::init();
    info!("logging enabled");
    debug!("{:?}", config);

    let owm_current_url = owm_current_url(&config);
    debug!("owm url: {}", owm_current_url);

    let darksky_current_url = darksky_current_url(&config);
    debug!("darksky url: {}", darksky_current_url);

    let client = Client::builder().gzip(true).build()?;

    client.get(owm_current_url).send().and_then(|mut r| {
        trace!("OWM current conditions json: {}", r.text()?);
        Ok(())
    })?;

    client.get(darksky_current_url).send().and_then(|mut r| {
        trace!("DarkSky current conditions json: {}", r.text()?);
        Ok(())
    })?;

    Ok(())
}

fn owm_current_url(config: &Config) -> Url {
    let location: Location = match &config.owm_location {
        Some(id) => Location::Id(id.to_string()),
        None => Location::Coord(config.latitude, config.longitude),
    };

    OwmApi::new(&config.owm_api_key, location, &config.owm_unit).current_url()
}

fn darksky_current_url(config: &Config) -> Url {
    let unit = Some(DarkSkyUnit::Ca);
    let darksky_api = DarkSkyApi::new(
        &config.darksky_api_key,
        Location::Coord(config.latitude, config.longitude),
        &unit,
    );
    debug!("{:?}", darksky_api);
    darksky_api.current_url()
}
