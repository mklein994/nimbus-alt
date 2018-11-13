use super::Config;
use super::WeatherApi;
use crate::config::OwmUnit;
use url::Url;

#[derive(Debug, PartialEq)]
pub struct OwmApi<'a> {
    pub url: Url,
    pub key: &'a str,
    pub location: Location<'a>,
    pub unit: Option<OwmUnit>,
}

#[derive(Debug, PartialEq)]
pub enum Location<'a> {
    Id(&'a str),
    Coord(f64, f64),
}

impl<'a, 'c: 'a> WeatherApi<'c> for OwmApi<'a> {
    const BASE_URL: &'static str = "https://api.openweathermap.org/data/2.5";

    fn new(config: &'c Config) -> Self {
        let owm = config
            .owm
            .as_ref()
            .unwrap_or_else(|| panic!("Tried to create OwmApi without api key."));

        let mut url = Url::parse_with_params(
            &format!("{}/weather", Self::BASE_URL),
            &[("appid", &owm.key)],
        )
        .unwrap();

        let key: &str = &owm.key;

        let location: Location = if let Some(ref id) = owm.location_id {
            Location::Id(id)
        } else if let Some((lat, lon)) = config.coordinates {
            Location::Coord(lat, lon)
        } else {
            panic!("location required. May be coordinates or a location id.");
        };

        // NOTE: There must always be a location.
        let location_query = match location {
            Location::Id(id) => vec![("id", id.to_string())],
            Location::Coord(lat, lon) => vec![("lat", lat.to_string()), ("lon", lon.to_string())],
        };

        url.query_pairs_mut().extend_pairs(location_query).finish();

        let unit: Option<OwmUnit> = owm.unit.or_else(|| config.unit.map(OwmUnit::from));

        // Determine the unit by checking the owm-specific config first, then the global
        // one. The default is to leave it blank.
        if let Some(unit) = owm.unit.or_else(|| config.unit.map(OwmUnit::from)) {
            url.query_pairs_mut()
                .append_pair("units", &unit.to_string())
                .finish();
        }

        Self {
            key,
            location,
            unit,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, OwmConfig};

    impl<'a> Default for OwmApi<'a> {
        fn default() -> Self {
            Self {
                key: "",
                location: Location::Id(""),
                unit: None,
                url: Url::parse("https://example.com").unwrap(),
            }
        }
    }

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

        let api = OwmApi::new(&config);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&id=a1b2c3d4",
        )
        .unwrap();
        assert_eq!(
            OwmApi {
                url: expected_url,
                key: "owm_key",
                location: Location::Id("a1b2c3d4"),
                unit: None,
            },
            api
        );
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

        let api = OwmApi::new(&config);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&id=a1b2c3d4&units=imperial",
        )
        .unwrap();
        assert_eq!(
            OwmApi {
                key: "owm_key",
                location: Location::Id("a1b2c3d4"),
                unit: Some(OwmUnit::Imperial),
                url: expected_url,
            },
            api
        );
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

        let api = OwmApi::new(&config);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&lat=12.345&lon=-54.321",
        )
        .unwrap();
        assert_eq!(
            OwmApi {
                key: "owm_key",
                location: Location::Coord(12.345, -54.321),
                unit: None,
                url: expected_url,
            },
            api
        );
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

        let api = OwmApi::new(&config);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&lat=12.345&lon=-54.321&units=metric",
        )
        .unwrap();
        assert_eq!(
            OwmApi {
                key: "owm_key",
                location: Location::Coord(12.345, -54.321),
                unit: Some(OwmUnit::Metric),
                url: expected_url,
                ..Default::default()
            },
            api
        );
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

        OwmApi::new(&config);
    }
}
