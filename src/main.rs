extern crate dotenv;
extern crate nimbus_alt;

use nimbus_alt::Config;

fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = Config::from_env().and_then(|config| nimbus_alt::run(&config)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
