use super::Config;
use super::GenericWeatherUnit;
// use super::KeyNotFound;
use super::{Location, WeatherApi};
use serde_derive::Deserialize;
use std::fmt;
use url::Url;

#[derive(Debug)]
pub struct DarkSkyApi {
    key: String,
    location: Location,
    unit: Option<DarkSkyUnit>,
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
            GenericWeatherUnit::Si => DarkSkyUnit::Si,
        }
    }
}

impl WeatherApi for DarkSkyApi {
    const BASE_URL: &'static str = "https://api.darksky.net/forecast";

    fn new(config: &Config) -> Self {
        config.darksky.as_ref().map_or_else(
            || panic!("Tried to create DarkSkyApi without api key."),
            |darksky| Self {
                key: darksky.key.to_string(),
                location: Location::Coord(config.latitude, config.longitude),
                unit: darksky.unit.or_else(|| config.unit.map(DarkSkyUnit::from)),
            },
        )
    }

    fn current_url(&self) -> Url {
        let mut url = format!("{base}/{key}", base = Self::BASE_URL, key = &self.key)
            .parse::<Url>()
            .unwrap();

        if let Location::Coord(lat, lon) = self.location {
            url.path_segments_mut()
                .unwrap()
                .push(&format!("{lat},{lon}", lat = lat, lon = lon));
        }

        match self.unit {
            Some(u) if u != DarkSkyUnit::Us => {
                url.query_pairs_mut()
                    .append_pair("units", &u.to_string())
                    .finish();
            }
            _ => {}
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_current_weather_url() {
        let darksky = DarkSkyApi {
            key: String::from("my_key"),
            location: Location::Coord(12.345, -54.321),
            unit: Some(DarkSkyUnit::Ca),
        };

        let expected_url =
            Url::parse("https://api.darksky.net/forecast/my_key/12.345,-54.321?units=ca").unwrap();
        let url = darksky.current_url();
        assert_eq!(expected_url, url);
    }

    #[test]
    fn it_gets_current_weather_url_no_units() {
        let darksky = DarkSkyApi {
            key: String::from("my_key"),
            location: Location::Coord(12.345, -54.321),
            unit: None,
        };

        let expected_url =
            Url::parse("https://api.darksky.net/forecast/my_key/12.345,-54.321").unwrap();
        let url = darksky.current_url();
        assert_eq!(expected_url, url);
    }
}
