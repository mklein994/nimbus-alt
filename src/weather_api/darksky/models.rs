use super::DarkSkyUnit;
use failure::Fail;
use serde_derive::Deserialize;
use std::fmt;

#[derive(Fail, Debug, Deserialize)]
pub struct DarkSkyError {
    code: u16,
    error: String,
}

impl fmt::Display for DarkSkyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "darksky error {}: {}", self.code, self.error)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    latitude: f64,
    longitude: f64,
    timezone: String,
    // deprecated
    offset: Option<i32>,
    currently: Option<DataPoint>,
    minutely: Option<DataBlock>,
    hourly: Option<DataBlock>,
    daily: Option<DataBlock>,
    alerts: Option<Vec<Alerts>>,
    flags: Option<Flags>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    apparent_temperature: Option<f64>,
    apparent_temperature_high: Option<f64>,
    apparent_temperature_high_time: Option<i64>,
    apparent_temperature_low: Option<f64>,
    apparent_temperature_low_time: Option<i64>,
    // deprecated
    apparent_temperature_max: Option<f64>,
    // deprecated
    apparent_temperature_max_time: Option<i64>,
    // deprecated
    apparent_temperature_min: Option<f64>,
    // deprecated
    apparent_temperature_min_time: Option<i64>,
    cloud_cover: Option<f64>,
    dew_point: Option<f64>,
    humidity: Option<f64>,
    icon: Option<Icon>,
    moon_phase: Option<f64>,
    nearest_storm_bearing: Option<i32>,
    nearest_storm_distance: Option<i32>,
    ozone: Option<f64>,
    precip_accumulation: Option<f64>,
    precip_intensity: Option<f64>,
    precip_intensity_error: Option<f64>,
    precip_intensity_max: Option<f64>,
    precip_intensity_max_time: Option<i64>,
    precip_probability: Option<f64>,
    precip_type: Option<PrecipitationType>,
    pressure: Option<f64>,
    summary: Option<String>,
    sunrise_time: Option<i64>,
    sunset_time: Option<f64>,
    temperature: Option<f64>,
    temperature_high: Option<f64>,
    temperature_high_time: Option<i64>,
    temperature_low: Option<f64>,
    temperature_low_time: Option<i64>,
    // deprecated
    temperature_max: Option<f64>,
    // deprecated
    temperature_max_time: Option<i64>,
    // deprecated
    temperature_min: Option<f64>,
    // deprecated
    temperature_min_time: Option<i64>,
    time: i64,
    uv_index: Option<i32>,
    uv_index_time: Option<i64>,
    visibility: Option<f64>,
    wind_bearing: Option<i32>,
    wind_gust: Option<f64>,
    wind_gust_time: Option<i64>,
    wind_speed: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataBlock {
    data: Vec<DataPoint>,
    summary: Option<String>,
    icon: Option<Icon>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alerts {
    description: String,
    expires: i64,
    regions: Vec<String>,
    severity: Severity,
    time: i64,
    title: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Flags {
    darksky_unavailable: Option<String>,
    nearest_station: Option<f64>,
    sources: Vec<String>,
    // undocumented
    meteoalarm_license: Option<String>,
    units: DarkSkyUnit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum Icon {
    ClearDay,
    ClearNight,
    Rain,
    Snow,
    Sleet,
    Wind,
    Fog,
    Cloudy,
    PartlyCloudyDay,
    PartlyCloudyNight,
    #[doc(hide)]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum PrecipitationType {
    Rain,
    Snow,
    Sleet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum Severity {
    Advisory,
    Watch,
    Warning,
}
