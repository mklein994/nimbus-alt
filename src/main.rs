use dotenv;
use nimbus_alt::{self, Config};

fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = Config::from_file().and_then(|config| nimbus_alt::run(&config)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
