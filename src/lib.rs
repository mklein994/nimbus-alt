extern crate failure;
extern crate reqwest;

mod config;

pub use self::config::Config;
use failure::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run(config: &Config) -> Result<()> {
    println!("{:?}", config);
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?appid={}&id={}&units=metric",
        config.api_key, config.location,
    );
    let res = reqwest::get(&url)?.error_for_status();

    match res {
        Ok(_) => println!("All good: {}", res?.text()?),
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}
