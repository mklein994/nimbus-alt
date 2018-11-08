use super::Url;

pub mod darksky;
pub mod owm;

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
