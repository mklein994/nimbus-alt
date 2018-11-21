use super::Config;
use super::{ForecastApi, WeatherApi};
use crate::config::{GenericWeatherUnit, OwmUnit};
use clap::ArgMatches;
use url::Url;

mod models;

pub use self::models::{Current, Forecast, OwmError};

#[derive(Debug, PartialEq)]
pub struct Owm<'a> {
    pub key: &'a str,
    pub location: Location<'a>,
    pub unit: Option<OwmUnit>,
}

#[derive(Debug, PartialEq)]
pub enum Location<'a> {
    Id(&'a str),
    Coord(f64, f64),
}

impl<'a, 'c: 'a> WeatherApi<'c> for Owm<'a> {
    const BASE_URL: &'static str = "https://api.openweathermap.org/data/2.5";
    type Current = Current;
    type ApiError = OwmError;

    fn new(config: &'c Config, m: &'c ArgMatches) -> Self {
        let owm = config
            .owm
            .as_ref()
            .unwrap_or_else(|| panic!("Tried to create Owm without api key."));

        let key: &str = &owm.key;

        // NOTE: There must always be a location.
        let location = values_t!(m.values_of("coordinates"), f64)
            .and_then(|coordinates| Ok(Location::Coord(coordinates[0], coordinates[1])))
            .ok()
            .or_else(|| owm.location_id.as_ref().map(|id| Location::Id(id)))
            .or_else(|| {
                config
                    .coordinates
                    .map(|(lat, lon)| Location::Coord(lat, lon))
            })
            .expect("location required. Must be coordinates or a location id.");

        let unit = value_t!(m.value_of("units"), GenericWeatherUnit)
            .map(OwmUnit::from)
            .ok()
            .or(owm.unit)
            .or_else(|| config.unit.map(OwmUnit::from));

        Self {
            key,
            location,
            unit,
        }
    }

    fn url(&self) -> Url {
        let mut url = Url::parse_with_params(
            &format!("{}/weather", Self::BASE_URL),
            &[("appid", self.key)],
        )
        .unwrap();

        match self.location {
            Location::Id(id) => url.query_pairs_mut().append_pair("id", id).finish(),
            Location::Coord(lat, lon) => url
                .query_pairs_mut()
                .extend_pairs(&[("lat", lat.to_string()), ("lon", lon.to_string())])
                .finish(),
        };

        // Determine the unit by checking the owm-specific config first, then the global one. The
        // default is to leave it blank.
        if let Some(unit) = self.unit {
            url.query_pairs_mut()
                .append_pair("units", &unit.to_string())
                .finish();
        }

        url
    }

    fn current_url(&self) -> Url {
        self.url()
    }
}

impl<'a, 'f: 'a> ForecastApi<'f> for Owm<'a> {
    type Forecast = Forecast;

    fn forecast_url(&self) -> Url {
        let mut url = self.url();
        url.path_segments_mut().unwrap().pop().push("forecast");

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, OwmConfig};

    impl<'a> Default for Owm<'a> {
        fn default() -> Self {
            Self {
                key: "",
                location: Location::Id(""),
                unit: None,
            }
        }
    }

    #[test]
    fn it_creates_a_new_owm_with_only_location_id() {
        let config = Config {
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                location_id: Some(String::from("a1b2c3d4")),
                ..Default::default()
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        let api = Owm::new(&config, &matches);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&id=a1b2c3d4",
        )
        .unwrap();
        assert_eq!(
            Owm {
                key: "owm_key",
                location: Location::Id("a1b2c3d4"),
                unit: None,
            },
            api
        );
        assert_eq!(expected_url, api.url());
    }

    #[test]
    fn it_creates_a_new_owm_with_location_id_and_owm_unit() {
        let config = Config {
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                location_id: Some(String::from("a1b2c3d4")),
                unit: Some(OwmUnit::Imperial),
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        let api = Owm::new(&config, &matches);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&id=a1b2c3d4&units=imperial",
        )
        .unwrap();
        assert_eq!(
            Owm {
                key: "owm_key",
                location: Location::Id("a1b2c3d4"),
                unit: Some(OwmUnit::Imperial),
            },
            api
        );
        assert_eq!(expected_url, api.url());
    }

    #[test]
    fn it_creates_a_new_owm_with_coordinates() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                ..Default::default()
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        let api = Owm::new(&config, &matches);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&lat=12.345&lon=-54.321",
        )
        .unwrap();
        assert_eq!(
            Owm {
                key: "owm_key",
                location: Location::Coord(12.345, -54.321),
                unit: None,
            },
            api
        );
        assert_eq!(expected_url, api.url());
    }

    #[test]
    fn it_creates_a_new_owm_with_coordinates_and_owm_unit() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                unit: Some(OwmUnit::Metric),
                ..Default::default()
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        let api = Owm::new(&config, &matches);

        let expected_url = Url::parse(
            "https://api.openweathermap.org/data/2.5/weather?\
             appid=owm_key&lat=12.345&lon=-54.321&units=metric",
        )
        .unwrap();
        assert_eq!(
            Owm {
                key: "owm_key",
                location: Location::Coord(12.345, -54.321),
                unit: Some(OwmUnit::Metric),
            },
            api
        );
        assert_eq!(expected_url, api.url());
    }

    #[test]
    #[should_panic]
    fn it_creates_a_new_owm_without_a_location() {
        let config = Config {
            owm: Some(OwmConfig {
                key: String::from("owm_key"),
                ..Default::default()
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        Owm::new(&config, &matches);
    }
}
