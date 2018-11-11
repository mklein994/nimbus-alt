use crate::weather_api::GenericWeatherUnit;
use crate::Error;
use failure::{Fail, ResultExt};
use serde_derive::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub coordinates: Option<(f64, f64)>,
    pub unit: Option<GenericWeatherUnit>,
    pub darksky: Option<DarkSkyConfig>,
    pub owm: Option<OwmConfig>,
}

#[derive(Debug, Deserialize)]
pub struct DarkSkyConfig {
    pub key: String,
    pub unit: Option<DarkSkyUnit>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DarkSkyUnit {
    Auto,
    Ca,
    Si,
    Uk2,
    Us,
}

impl fmt::Display for DarkSkyUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DarkSkyUnit::Auto => "auto",
                DarkSkyUnit::Ca => "ca",
                DarkSkyUnit::Si => "si",
                DarkSkyUnit::Uk2 => "uk2",
                DarkSkyUnit::Us => "us",
            }
        )
    }
}

impl From<GenericWeatherUnit> for DarkSkyUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => DarkSkyUnit::Si,
            GenericWeatherUnit::Imperial => DarkSkyUnit::Us,
            GenericWeatherUnit::Si => DarkSkyUnit::Si,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OwmConfig {
    pub key: String,
    pub location_id: Option<String>,
    pub unit: Option<OwmUnit>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OwmUnit {
    Metric,
    Imperial,
    Si,
}

impl From<GenericWeatherUnit> for OwmUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => OwmUnit::Metric,
            GenericWeatherUnit::Imperial => OwmUnit::Imperial,
            GenericWeatherUnit::Si => OwmUnit::Si,
        }
    }
}

impl fmt::Display for OwmUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OwmUnit::Metric => "metric",
                OwmUnit::Imperial => "imperial",
                OwmUnit::Si => panic!("Can't print OWM SI unit"),
            }
        )
    }
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

        // Ensure that if the OWM config is specified, that there is always a location.
        if let Some(owm) = &config.owm {
            if owm.location_id.is_none() && config.coordinates.is_none() {
                return Err(LocationMissingError.into());
            }
        }

        Ok(config)
    }
}

#[derive(Fail, Debug)]
#[fail(
    display = "Could not determine the location. Ensure that a location is\
               specified in the configuration."
)]
pub struct LocationMissingError;

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Config {
        fn default() -> Self {
            Self {
                coordinates: None,
                unit: None,
                owm: None,
                darksky: None,
            }
        }
    }

    impl Default for OwmConfig {
        fn default() -> Self {
            Self {
                key: String::new(),
                location_id: None,
                unit: None,
            }
        }
    }

    impl Default for DarkSkyConfig {
        fn default() -> Self {
            Self {
                key: String::new(),
                unit: None,
            }
        }
    }
}
