use crate::weather_api::owm::OwmUnit;
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
    pub owm_unit: Option<OwmUnit>,
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
}
