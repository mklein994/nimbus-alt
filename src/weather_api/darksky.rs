use super::Config;
use super::WeatherApi;
use crate::config::DarkSkyUnit;
use url::Url;

#[derive(Debug, PartialEq)]
pub struct DarkSkyApi {
    pub url: Url,
}

impl WeatherApi for DarkSkyApi {
    const BASE_URL: &'static str = "https://api.darksky.net/forecast";

    fn new(config: &Config) -> Self {
        config.darksky.as_ref().map_or_else(
            || panic!("Tried to create DarkSkyApi without api key."),
            |darksky| {
                let mut url = Url::parse(Self::BASE_URL).unwrap();
                let (lat, lon) = config
                    .coordinates
                    .map(|(lat, lon)| (lat.to_string(), lon.to_string()))
                    .unwrap();

                url.path_segments_mut()
                    .unwrap()
                    .push(&darksky.key)
                    .push(&format!("{},{}", lat, lon));

                if let Some(unit) = darksky.unit.or_else(|| config.unit.map(DarkSkyUnit::from)) {
                    url.query_pairs_mut()
                        .append_pair("units", &unit.to_string());
                }
                Self { url }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, DarkSkyConfig};

    #[test]
    fn it_creates_new_darksky_api_with_coordinates() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            darksky: Some(DarkSkyConfig {
                key: String::from("my_key"),
                ..Default::default()
            }),
            ..Default::default()
        };

        let api = DarkSkyApi::new(&config);

        let expected_url =
            Url::parse("https://api.darksky.net/forecast/my_key/12.345,-54.321").unwrap();
        assert_eq!(DarkSkyApi { url: expected_url }, api);
    }

    #[test]
    fn it_creates_new_darksky_api_with_darksky_unit() {
        let config = Config {
            coordinates: Some((12.345, -54.321)),
            darksky: Some(DarkSkyConfig {
                key: String::from("my_key"),
                unit: Some(DarkSkyUnit::Uk2),
            }),
            ..Default::default()
        };

        let api = DarkSkyApi::new(&config);

        let expected_url = Url::parse(
            "https://api.darksky.net/forecast/my_key/12.345,-54.321?\
             units=uk2",
        )
        .unwrap();
        assert_eq!(DarkSkyApi { url: expected_url }, api);
    }
}
