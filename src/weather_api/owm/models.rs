use failure::Fail;
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Forecast {
    coord: Option<Coord>,
    weather: Option<Vec<Weather>>,
    // internal
    base: Option<String>,
    main: Option<Main>,
    wind: Option<Wind>,
    clouds: Option<Clouds>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    dt: Option<i64>,
    sys: Option<Sys>,
    id: Option<i64>,
    name: Option<String>,
    // internal
    cod: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    id: Option<i32>,
    description: Option<String>,
    icon: Option<Icon>,
    main: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Icon(String);

#[derive(Fail, Debug)]
#[fail(display = "Invalid OWM Icon")]
pub struct InvalidOwmIcon;

impl FromStr for Icon {
    type Err = InvalidOwmIcon;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Icon(String::from(s)))
    }
}

#[derive(Debug, Deserialize)]
pub struct Main {
    temp: Option<f64>,
    pressure: Option<i32>,
    humidity: Option<i32>,
    temp_min: Option<i32>,
    temp_max: Option<i32>,
    sea_level: Option<i32>,
    grnd_level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    speed: Option<f64>,
    deg: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    all: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Rain {
    #[serde(rename = "3h")]
    three_h: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Snow {
    #[serde(rename = "3h")]
    three_h: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    // internal
    sys_type: Option<i32>,
    // internal
    id: Option<i32>,
    // internal
    message: Option<f64>,
    country: Option<String>,
    sunrise: Option<i64>,
    sunset: Option<i64>,
}
