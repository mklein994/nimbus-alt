use crate::GenericWeatherUnit;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
}

impl From<GenericWeatherUnit> for OwmUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => OwmUnit::Metric,
            GenericWeatherUnit::Imperial => OwmUnit::Imperial,
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
            }
        )
    }
}
