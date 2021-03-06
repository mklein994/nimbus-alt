use super::ArgEnum;
use super::GenericWeatherUnit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OwmConfig {
    pub key: String,
    pub location_id: Option<String>,
    pub unit: Option<OwmUnit>,
}

#[derive(
    Debug, Display, PartialEq, Eq, Copy, Clone, EnumString, EnumIter, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "kebab_case")]
pub enum OwmUnit {
    Metric,
    Imperial,
}

impl ArgEnum for OwmUnit {
    const VARIANTS: &'static [&'static str] = &["metric", "imperial"];
}

impl From<GenericWeatherUnit> for OwmUnit {
    fn from(unit: GenericWeatherUnit) -> Self {
        match unit {
            GenericWeatherUnit::Metric => OwmUnit::Metric,
            GenericWeatherUnit::Imperial => OwmUnit::Imperial,
        }
    }
}
