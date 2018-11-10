#[macro_use]
extern crate log;

mod config;
mod weather_api;

pub use self::config::Config;
use self::weather_api::darksky::DarkSkyApi;
use self::weather_api::owm::OwmApi;
use self::weather_api::WeatherApi;
use failure::Error;
use reqwest::Client;

pub fn run(config: &Config) -> Result<(), Error> {
    env_logger::init();
    info!("logging enabled");
    debug!("{:?}", config);

    let owm = OwmApi::new(&config);

    let owm_url = owm.current_url();
    info!("{}", owm_url);

    let darksky = DarkSkyApi::new(&config);
    let darksky_url = darksky.current_url();
    info!("{}", darksky_url);

    // TODO: remove this feature once testing is setup
    if cfg!(feature = "live") == false {
        return Ok(());
    }

    let client = Client::builder().gzip(true).build()?;

    client.get(owm_url).send().and_then(|mut r| {
        trace!("OWM current conditions json: {}", r.text()?);
        Ok(())
    })?;

    client.get(darksky_url).send().and_then(|mut r| {
        trace!("DarkSky current conditions json: {}", r.text()?);
        Ok(())
    })?;

    Ok(())
}
