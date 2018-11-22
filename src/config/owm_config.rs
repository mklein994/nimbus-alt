use super::GenericWeatherUnit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwmConfig {
    pub key: String,
    pub location_id: Option<String>,
    pub unit: Option<OwmUnit>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OwmUnit {
    Metric,
    Imperial,
}
forward_from_str_to_serde!(OwmUnit);
forward_display_to_serde!(OwmUnit);

impl From<GenericWeatherUnit> for OwmUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => OwmUnit::Metric,
            GenericWeatherUnit::Imperial => OwmUnit::Imperial,
        }
    }
}
