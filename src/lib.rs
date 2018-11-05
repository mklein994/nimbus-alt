extern crate failure;
extern crate reqwest;
extern crate url;

mod config;

pub use self::config::Config;
use failure::Error;
use url::Url;

pub fn run(config: &Config) -> Result<(), Error> {
    println!("{:?}", config);

    let url = owm_weather_url(&config)?;
    let res = reqwest::get(url.as_str())?.error_for_status();

    match res {
        Ok(_) => println!("All good: {}", res?.text()?),
        Err(e) => println!("error: {}", e),
    }

    let url = darksky_weather_url(&config)?;
    let res = reqwest::get(url.as_str())?.error_for_status();

    match res {
        Ok(_) => println!("All good: {}", res?.text()?),
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}

fn owm_weather_url(config: &Config) -> Result<Url, url::ParseError> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?appid={}&id={}&units=metric",
        config.owm_api_key, config.location,
    );
    Url::parse(&url)
}

fn darksky_weather_url(config: &Config) -> Result<Url, url::ParseError> {
    let url = format!(
        "https://api.darksky.net/forecast/{}/{},{}",
        config.darksky_api_key, config.latitude, config.longitude
    );
    Url::parse(&url)
}
