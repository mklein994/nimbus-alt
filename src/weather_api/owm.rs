use super::{Api, Location, UnitLike, WeatherApi};
use url::Url;

#[derive(Debug)]
pub struct OwmApi(Api<OwmUnit>);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
