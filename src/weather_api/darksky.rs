use super::Config;
use super::{HistoricalApi, WeatherApi};
use clap::ArgMatches;
use crate::config::{DarkSkyUnit, GenericWeatherUnit};
use url::Url;

mod models;

pub use self::models::Forecast;

#[derive(Debug, PartialEq)]
pub struct DarkSky<'a> {
    pub key: &'a str,
    pub coordinates: (f64, f64),
    pub unit: Option<DarkSkyUnit>,
}

impl<'a, 'c: 'a> WeatherApi<'c> for DarkSky<'a> {
    const BASE_URL: &'static str = "https://api.darksky.net/forecast";
    type Current = Forecast;

    fn new(config: &'a Config, m: &'a ArgMatches) -> Self {
        let darksky = config
            .darksky
            .as_ref()
            .unwrap_or_else(|| panic!("Tried to create DarkSky without api key."));

        let key: &str = &darksky.key;

        let (latitude, longitude) = values_t!(m.values_of("coordinates"), f64)
            .and_then(|coordinates| Ok((coordinates[0], coordinates[1])))
            .ok()
            .or(config.coordinates)
            .expect("tried creating darksky api without coordinates in config");

        let unit = value_t!(m.value_of("units"), GenericWeatherUnit)
            .map(DarkSkyUnit::from)
            .ok()
            .or(darksky.unit)
            .or_else(|| config.unit.map(DarkSkyUnit::from));

        Self {
            key,
            coordinates: (latitude, longitude),
            unit,
        }
    }

    fn url(&self) -> Url {
        let mut url = Url::parse(&format!(
            "{base}/{key}/{lat},{lon}",
            base = Self::BASE_URL,
            key = self.key,
            lat = self.coordinates.0.to_string(),
            lon = self.coordinates.1.to_string()
        ))
        .unwrap();

        if let Some(unit) = self.unit {
            url.query_pairs_mut()
                .append_pair("units", &unit.to_string())
                .finish();
        }

        url
    }

    fn current_url(&self) -> Url {
        let mut url = self.url();
        url.query_pairs_mut()
            .append_pair("exclude", "minutely,hourly,daily,alerts,flags")
            .finish();

        url
    }
}

impl<'a> HistoricalApi<'a> for DarkSky<'a> {
    fn historical_url(&self, time: i64) -> Url {
        let mut url = self.url();

        url.path_segments_mut().unwrap().pop().push(&format!(
            "{lat},{lon},{time}",
            lat = self.coordinates.0,
            lon = self.coordinates.1,
            time = time
        ));

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, DarkSkyConfig};

    #[test]
    fn it_creates_new_darksky_with_coordinates() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            darksky: Some(DarkSkyConfig {
                key: String::from("my_key"),
                ..Default::default()
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        let api = DarkSky::new(&config, &matches);

        let expected_url =
            Url::parse("https://api.darksky.net/forecast/my_key/12.345,-54.321").unwrap();
        assert_eq!(
            DarkSky {
                key: "my_key",
                coordinates: (12.345, -54.321),
                unit: None,
            },
            api
        );
        assert_eq!(expected_url, api.url());
    }

    #[test]
    fn it_creates_new_darksky_with_darksky_unit() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            darksky: Some(DarkSkyConfig {
                key: String::from("my_key"),
                unit: Some(DarkSkyUnit::Uk2),
            }),
            ..Default::default()
        };

        let matches = clap::App::new("name").get_matches_from(vec!["name"]);
        let api = DarkSky::new(&config, &matches);

        let expected_url = Url::parse(
            "https://api.darksky.net/forecast/my_key/12.345,-54.321?\
             units=uk2",
        )
        .unwrap();
        assert_eq!(
            DarkSky {
                key: "my_key",
                coordinates: (12.345, -54.321),
                unit: Some(DarkSkyUnit::Uk2),
            },
            api
        );
        assert_eq!(expected_url, api.url());
    }

    #[test]
    fn it_gets_darksky_historical_weather() {
        let api = DarkSky {
            key: "my_key",
            coordinates: (12.345, -54.321),
            unit: None,
        };
        let time = 1542143061;

        let expected_url = Url::parse(&format!(
            "https://api.darksky.net/forecast/my_key/12.345,-54.321,{}",
            time
        ))
        .unwrap();
        let actual_url = api.historical_url(time);

        assert_eq!(expected_url, actual_url);
    }
}
