use super::GenericWeatherUnit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DarkSkyConfig {
    pub key: String,
    pub unit: Option<DarkSkyUnit>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DarkSkyUnit {
    Auto,
    Ca,
    Si,
    Uk2,
    Us,
}
forward_from_str_to_serde!(DarkSkyUnit);
forward_display_to_serde!(DarkSkyUnit);

impl From<GenericWeatherUnit> for DarkSkyUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => DarkSkyUnit::Si,
            GenericWeatherUnit::Imperial => DarkSkyUnit::Us,
        }
    }
}
