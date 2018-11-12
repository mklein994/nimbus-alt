#[macro_use]
extern crate log;

mod config;
mod weather_api;

pub use self::config::*;
use self::weather_api::darksky::DarkSkyApi;
use self::weather_api::owm::OwmApi;
use self::weather_api::WeatherApi;
use failure::Error;
use reqwest::Client;

pub fn run(config: &Config) -> Result<(), Error> {
    env_logger::init();
    info!("logging enabled");
    debug!("{:?}", config);

    let owm_url = OwmApi::new_url(&config);

    info!("{}", owm_url);

    let darksky_url = DarkSkyApi::new_url(&config);
    info!("{}", darksky_url);

    // TODO: remove this feature once testing is setup
    if cfg!(feature = "live") == false {
        return Ok(());
    }

    let client = Client::builder().gzip(true).build()?;

    let owm_current_weather: serde_json::Value = client.get(owm_url).send()?.json()?;
    info!("successfully retrieved owm current weather");
    trace!("{}", owm_current_weather);

    let darksky_current_weather: serde_json::Value = client.get(darksky_url).send()?.json()?;
    info!("successfully retrieved darksky current weather");
    trace!("{}", darksky_current_weather);

    Ok(())
}
