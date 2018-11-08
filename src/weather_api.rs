use super::Url;

#[derive(Debug)]
struct Api<T>
where
    T: UnitLike,
{
    key: String,
    location: Location,
    unit: Option<T>,
}

pub trait UnitLike: PartialEq + Eq + std::fmt::Debug + Copy + Clone {
    fn metric() -> Self;
    fn imperial() -> Self;
    fn default() -> Self;
}

#[derive(Debug)]
pub enum Location {
    Coord(f64, f64),
    Id(String),
}

pub trait WeatherApi {
    const BASE_URL: &'static str;
    type Unit: UnitLike;

    fn new(key: &str, location: Location, unit: &Option<Self::Unit>) -> Self;

    fn current_url(&self) -> Url;
}

pub mod owm {
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
                Location::Coord(lat, lon) => {
                    vec![("lat", lat.to_string()), ("lon", lon.to_string())]
                }
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
}

pub mod darksky {
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
            if let Location::Coord(lat, lon) = self.0.location {
                let mut url = Url::parse(&format!(
                    "{base}/{key}/{lat},{lon}",
                    base = Self::BASE_URL,
                    key = self.0.key,
                    lat = lat,
                    lon = lon
                ))
                .unwrap();

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
            } else {
                unimplemented!()
            }
        }
    }
}
