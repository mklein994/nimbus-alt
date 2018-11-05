use crate::Error;

#[derive(Debug)]
pub struct Config {
    pub owm_api_key: String,
    pub darksky_api_key: String,
    pub location: String,
    //pub latitude: f64,
    //pub longitude: f64,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        let owm_key = dotenv::var("NIMBUS_OWM_KEY")
            .expect("OpenWeatherMap API key missing (NIMBUS_OWM_KEY).");
        let darksky_api_key = dotenv::var("NIMBUS_DARKSKY_KEY")
            .expect("Darksky API key missing (NIMBUS_DARKSKY_KEY).");
        //let latitude = dotenv::var("NIMBUS_LATITUDE").expect("Missing latitude.");
        //let longitude = dotenv::var("NIMBUS_LONGITUDE").expect("Missing longitude.");
        let location = dotenv::var("NIMBUS_OWM_LOCATION").expect("Missing location.");

        Ok(Self {
            owm_api_key: owm_key,
            darksky_api_key,
            location,
            //latitude: latitude.parse()?,
            //longitude: longitude.parse()?,
        })
    }
}
