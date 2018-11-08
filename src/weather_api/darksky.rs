use super::{Api, Location, UnitLike, WeatherApi};
use url::Url;

#[derive(Debug)]
pub struct DarkSkyApi(Api<DarkSkyUnit>);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DarkSkyUnit {
    Auto,
    Ca,
    Si,
    Uk2,
    Us,
}

impl UnitLike for DarkSkyUnit {
    fn metric() -> Self {
        DarkSkyUnit::Si
    }

    fn imperial() -> Self {
        DarkSkyUnit::Us
    }

    fn default() -> Self {
        DarkSkyUnit::Us
    }
}

impl WeatherApi for DarkSkyApi {
    const BASE_URL: &'static str = "https://api.darksky.net/forecast";
    type Unit = DarkSkyUnit;

    fn new(key: &str, location: Location, unit: &Option<Self::Unit>) -> Self {
        match location {
            Location::Coord(_, _) => DarkSkyApi(Api {
                key: key.to_string(),
                location,
                unit: *unit,
            }),
            _ => unimplemented!(),
        }
    }

    fn current_url(&self) -> Url {
        let mut url = Url::parse(&format!(
            "{base}/{key}",
            base = Self::BASE_URL,
            key = self.0.key
        ))
        .unwrap();

        if let Location::Coord(lat, lon) = self.0.location {
            url.path_segments_mut()
                .unwrap()
                .push(&format!("{lat},{lon}", lat = lat, lon = lon));
        }

        match self.0.unit {
            Some(u) if u != DarkSkyUnit::default() => {
                url.query_pairs_mut().append_pair(
                    "units",
                    match u {
                        DarkSkyUnit::Auto => "auto",
                        DarkSkyUnit::Ca => "ca",
                        DarkSkyUnit::Uk2 => "uk2",
                        DarkSkyUnit::Us => "us",
                        DarkSkyUnit::Si => "si",
                    },
                );
            }
            _ => {}
        }

        url
    }
}
