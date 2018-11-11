use super::Config;
use super::{GenericWeatherUnit, WeatherApi};
use serde_derive::Deserialize;
use std::fmt;
use url::Url;

#[derive(Debug)]
pub struct OwmApi;

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

impl WeatherApi for OwmApi {
    const BASE_URL: &'static str = "http://api.openweathermap.org/data/2.5";

    fn new_url(config: &Config) -> Url {
        config.owm.as_ref().map_or_else(
            || panic!("Tried to create OwmApi without api key."),
            |owm| {
                let mut url = Url::parse_with_params(
                    &format!("{}/weather", Self::BASE_URL),
                    &[("appid", &owm.key)],
                )
                .unwrap();

                // NOTE: There must always be a location.
                if let Some(id) = &owm.location_id {
                    url.query_pairs_mut().append_pair("id", id).finish();
                } else if let Some((lat, lon)) = config.coordinates {
                    url.query_pairs_mut()
                        .extend_pairs(&[("lat", lat.to_string()), ("lon", lon.to_string())])
                        .finish();
                } else {
                    panic!("location required. May be coordinates or a location id.");
                }

                // Determine the unit by checking the owm-specific config first, then the global
                // one. The default is to leave it blank.
                if let Some(unit) = owm.unit.or_else(|| config.unit.map(OwmUnit::from)) {
                    url.query_pairs_mut()
                        .append_pair("units", &unit.to_string())
                        .finish();
                }

                url
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, OwmConfig};

    #[test]
    fn it_creates_a_new_owm_api_with_only_location_id() {
        let config = Config {
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                location_id: Some(String::from("a1b2c3d4")),
                ..Default::default()
            }),
            ..Default::default()
        };

        let api_url = OwmApi::new_url(&config);

        let expected_url =
            Url::parse("http://api.openweathermap.org/data/2.5/weather?appid=owm_key&id=a1b2c3d4")
                .unwrap();
        assert_eq!(expected_url, api_url);
    }

    #[test]
    fn it_creates_a_new_owm_api_with_location_id_and_owm_unit() {
        let config = Config {
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                location_id: Some(String::from("a1b2c3d4")),
                unit: Some(OwmUnit::Imperial),
            }),
            ..Default::default()
        };

        let api_url = OwmApi::new_url(&config);

        let expected_url = Url::parse("http://api.openweathermap.org/data/2.5/weather?appid=owm_key&id=a1b2c3d4&units=imperial").unwrap();
        assert_eq!(expected_url, api_url);
    }

    #[test]
    fn it_creates_a_new_owm_api_with_coordinates() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                ..Default::default()
            }),
            ..Default::default()
        };

        let api_url = OwmApi::new_url(&config);

        let expected_url = Url::parse(
            "http://api.openweathermap.org/data/2.5/weather?appid=owm_key&lat=12.345&lon=-54.321",
        )
        .unwrap();
        assert_eq!(expected_url, api_url);
    }

    #[test]
    fn it_creates_a_new_owm_api_with_coordinates_and_owm_unit() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                unit: Some(OwmUnit::Metric),
                ..Default::default()
            }),
            ..Default::default()
        };

        let api_url = OwmApi::new_url(&config);

        let expected_url = Url::parse("http://api.openweathermap.org/data/2.5/weather?appid=owm_key&lat=12.345&lon=-54.321&units=metric").unwrap();
        assert_eq!(expected_url, api_url);
    }

    #[test]
    #[should_panic]
    fn it_creates_a_new_owm_api_without_a_location() {
        let config = Config {
            coordinates: None,
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                ..Default::default()
            }),
            ..Default::default()
        };

        OwmApi::new_url(&config);
    }
}
