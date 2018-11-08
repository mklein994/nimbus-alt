#[macro_use]
extern crate log;

mod config;
mod weather_api;

pub use self::config::Config;
use self::weather_api::owm::{OwmApi, OwmUnit};
use self::weather_api::{DarkSkyApi, DarkSkyUnit, Location, UnitLike, WeatherApi};
use failure::Error;
use reqwest::Client;
use url::Url;

pub fn run(config: &Config) -> Result<(), Error> {
    env_logger::init();
    info!("logging enabled");
    debug!("{:?}", config);

    let client = Client::builder().gzip(true).build()?;

    let location = config.owm_location.as_ref().unwrap();
    let unit: Option<OwmUnit> = Some(UnitLike::metric());

    let owm_api = OwmApi::new(
        &config.owm_api_key,
        Location::Id(location.to_string()),
        &unit,
    );
    debug!("{:?}", owm_api);
    let owm_url = owm_api.current_url();
    debug!("owm url: {}", owm_url);
    let res = client.get(owm_url).send();

    match res {
        Ok(mut r) => println!("All good: {}", r.text()?),
        Err(e) => println!("error: {}", e),
    }

    let unit = Some(DarkSkyUnit::Ca);
    let darksky_api = DarkSkyApi::new(
        &config.darksky_api_key,
        Location::Coord(config.latitude, config.longitude),
        &unit,
    );
    debug!("{:?}", darksky_api);
    let darksky_url = darksky_api.current_url();

    debug!("darksky url: {}", darksky_url);
    let res = client.get(darksky_url).send();

    match res {
        Ok(mut r) => println!("All good: {}", r.text()?),
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}
