use dotenv;
use nimbus_alt::{self, app, Config};

fn main() {
    dotenv::dotenv().ok();

    let _m = app::build_cli().get_matches();

    if let Err(e) = Config::from_file().and_then(|config| nimbus_alt::run(&config)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
