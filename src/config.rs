use crate::Error;
use failure::ResultExt;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub owm_api_key: String,
    pub darksky_api_key: String,
    pub owm_location: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}

impl Config {
    pub fn from_file() -> Result<Self, Error> {
        let config_path = match dirs::config_dir() {
            Some(path) => path.join("nimbus-alt").join("config.toml"),
            None => panic!("Couldn't find XDG_CONFIG_HOME"),
        };

        let mut file = File::open(&config_path)
            .with_context(|e| format!("could not read file {}: {}", config_path.display(), e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn from_env() -> Result<Self, Error> {
        let owm_api_key = dotenv::var("NIMBUS_OWM_KEY")
            .expect("OpenWeatherMap API key missing (NIMBUS_OWM_KEY).");
        let darksky_api_key = dotenv::var("NIMBUS_DARKSKY_KEY")
            .expect("Darksky API key missing (NIMBUS_DARKSKY_KEY).");
        let latitude = dotenv::var("NIMBUS_LATITUDE").expect("Missing latitude.");
        let longitude = dotenv::var("NIMBUS_LONGITUDE").expect("Missing longitude.");
        let owm_location = match dotenv::var("NIMBUS_OWM_LOCATION") {
            Ok(location) => Some(location),
            Err(ref e) if e.not_found() => None,
            Err(e) => return Err(e.into()),
        };

        Ok(Self {
            owm_api_key,
            darksky_api_key,
            owm_location,
            latitude: latitude.parse()?,
            longitude: longitude.parse()?,
        })
    }
}
