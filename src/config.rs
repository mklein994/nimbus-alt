use crate::Error;
use failure::{Fail, ResultExt};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

mod darksky_config;
mod owm_config;

pub use self::darksky_config::*;
pub use self::owm_config::*;

#[derive(Debug, Deserialize, EnumString, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "kebab_case")]
pub enum GenericWeatherUnit {
    Metric,
    Imperial,
}

#[derive(Fail, Debug)]
#[fail(display = "invalid unit passed")]
pub struct InvalidUnit;

/// Global configuration for the app.
///
/// # Example
///
/// `~/.config/nimbus-alt/config.toml`:
/// ```
/// # #[macro_use] extern crate toml;
/// # use nimbus_alt::{DarkSkyConfig, Config, DarkSkyUnit, GenericWeatherUnit, OwmConfig, OwmUnit};
/// # fn main() {
/// # let config = toml! {
/// coordinates = [ 12.345, -54.321 ]
/// unit = "metric"
///
/// [owm]
/// key = "a1b2c3d4"
/// location_id = "1234567"
/// unit = "imperial"
///
/// [darksky]
/// key = "n1o2p3q4"
/// unit = "ca"
/// # }
/// # .try_into::<Config>();
/// # assert!(config.is_ok());
/// # }
/// ```
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub coordinates: Option<(f64, f64)>,
    pub unit: Option<GenericWeatherUnit>,
    pub darksky: Option<DarkSkyConfig>,
    pub owm: Option<OwmConfig>,
}

impl Config {
    pub fn from_file() -> Result<Self, Error> {
        let config_path = match dirs::config_dir() {
            Some(path) => path.join(crate_name!()).join("config.toml"),
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

        if config.darksky.is_none() && config.coordinates.is_none() {
            return Err(LocationMissingError.into());
        }

        Ok(config)
    }

    pub fn merge_args(mut self, args: &clap::ArgMatches) -> Result<Self, Error> {
        match args.subcommand() {
            ("owm", Some(owm_m)) => {
                if let Some(ref mut owm) = self.owm {
                    if let Some(unit) = owm_m
                        .value_of("unit")
                        .and_then(|u| OwmUnit::from_str(u).ok())
                    {
                        owm.unit = Some(unit);
                    }
                }
            }
            ("darksky", Some(darksky_m)) => {}
            _ => {}
        }
        Ok(self)
    }
}

#[derive(Fail, Debug)]
#[fail(
    display = "Could not determine the location. Ensure that a location is \
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
