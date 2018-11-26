use super::ArgEnum;
use super::GenericWeatherUnit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DarkSkyConfig {
    pub key: String,
    pub unit: Option<DarkSkyUnit>,
}

#[derive(Display, Debug, PartialEq, Eq, Copy, Clone, EnumString, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "kebab_case")]
pub enum DarkSkyUnit {
    Auto,
    Ca,
    Si,
    Uk2,
    Us,
}

impl ArgEnum for DarkSkyUnit {
    const VARIANTS: &'static [&'static str] = &["auto", "ca", "si", "uk2", "us"];
}

impl From<GenericWeatherUnit> for DarkSkyUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => DarkSkyUnit::Si,
            GenericWeatherUnit::Imperial => DarkSkyUnit::Us,
        }
    }
}
