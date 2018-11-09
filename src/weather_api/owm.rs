use super::{Api, Location, UnitLike, WeatherApi};
use serde_derive::Deserialize;
use url::Url;

#[derive(Debug)]
pub struct OwmApi(Api<OwmUnit>);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OwmUnit {
    Metric,
    Imperial,
    Si,
}

impl UnitLike for OwmUnit {
    fn metric() -> Self {
        OwmUnit::Metric
    }

    fn imperial() -> Self {
        OwmUnit::Imperial
    }

    fn default() -> Self {
        OwmUnit::Si
    }
}

impl WeatherApi for OwmApi {
    const BASE_URL: &'static str = "http://api.openweathermap.org/data/2.5";
    type Unit = OwmUnit;

    fn new(key: &str, location: Location, unit: &Option<Self::Unit>) -> Self {
        OwmApi(Api {
            key: key.to_string(),
            location,
            unit: *unit,
        })
    }

    fn current_url(&self) -> Url {
        let mut url = format!(
            "{base}/weather?appid={key}",
            base = Self::BASE_URL,
            key = &self.0.key
        )
        .parse::<Url>()
        .unwrap();

        let pairs = match &self.0.location {
            Location::Coord(lat, lon) => vec![("lat", lat.to_string()), ("lon", lon.to_string())],
            Location::Id(id) => vec![("id", id.to_string())],
        };

        url.query_pairs_mut().extend_pairs(&pairs);
        match self.0.unit {
            Some(ref unit) if unit != &OwmUnit::default() => {
                let unit = match unit {
                    OwmUnit::Metric => "metric",
                    OwmUnit::Imperial => "imperial",
                    _ => unimplemented!(),
                };
                url.query_pairs_mut().append_pair("units", unit).finish();
            }
            _ => {}
        };

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_current_weather_url() {
        let owm = OwmApi(Api {
            key: String::from("my_key"),
            location: Location::Id(String::from("a1b2c3d4")),
            unit: Some(OwmUnit::Imperial),
        });

        let expected_url = Url::parse("http://api.openweathermap.org/data/2.5/weather?appid=my_key&id=a1b2c3d4&units=imperial").unwrap();
        let url = owm.current_url();
        assert_eq!(expected_url, url);
    }
}
