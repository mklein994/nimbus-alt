use super::Config;
use super::{Location, WeatherApi};
use serde_derive::Deserialize;
use std::fmt;
use url::Url;

#[derive(Debug)]
pub struct OwmApi {
    key: String,
    location: Option<Location>,
    unit: Option<OwmUnit>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OwmUnit {
    Metric,
    Imperial,
    Si,
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

    fn new(config: &Config) -> Self {
        config.owm.as_ref().map_or_else(
            || panic!("can't create an OwmApi without an api key."),
            |owm| Self {
                key: owm.owm_api_key.to_string(),
                location: match &owm.owm_location_id {
                    Some(id) => Some(Location::Id(id.to_string())),
                    None => Some(Location::Coord(config.latitude, config.longitude)),
                },
                unit: owm.owm_unit.or_else(|| config.unit.map(OwmUnit::from)),
            },
        )
    }

    fn current_url(&self) -> Url {
        let mut url = format!(
            "{base}/weather?appid={key}",
            base = Self::BASE_URL,
            key = &self.key
        )
        .parse::<Url>()
        .unwrap();

        let pairs = if let Some(location) = &self.location {
            match location {
                Location::Coord(lat, lon) => {
                    vec![("lat", lat.to_string()), ("lon", lon.to_string())]
                }
                Location::Id(id) => vec![("id", id.to_string())],
            }
        } else {
            panic!("Couldn't get owm location");
        };

        url.query_pairs_mut().extend_pairs(&pairs);
        if let Some(unit) = &self.unit {
            url.query_pairs_mut()
                .append_pair("units", &unit.to_string())
                .finish();
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_current_weather_url() {
        let owm = OwmApi {
            key: String::from("my_key"),
            location: Some(Location::Id(String::from("a1b2c3d4"))),
            unit: Some(OwmUnit::Imperial),
        };

        let expected_url = Url::parse("http://api.openweathermap.org/data/2.5/weather?appid=my_key&id=a1b2c3d4&units=imperial").unwrap();
        let url = owm.current_url();
        assert_eq!(expected_url, url);
    }

    #[test]
    fn it_gets_current_weather_url_with_coordinates() {
        let owm = OwmApi {
            key: String::from("my_key"),
            location: Some(Location::Coord(12.345, -54.321)),
            unit: Some(OwmUnit::Imperial),
        };

        let expected_url = Url::parse("http://api.openweathermap.org/data/2.5/weather?appid=my_key&lat=12.345&lon=-54.321&units=imperial").unwrap();
        let url = owm.current_url();
        assert_eq!(expected_url, url);
    }
}
