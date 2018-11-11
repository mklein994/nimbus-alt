use crate::weather_api::GenericWeatherUnit;
use serde_derive::Deserialize;
use std::fmt;

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
        }
    }
}
