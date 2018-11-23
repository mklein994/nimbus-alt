use dotenv;
use nimbus_alt::{self, app, Config};

fn main() {
    dotenv::dotenv().ok();

    let matches = app::build_cli().get_matches();

    if let Err(e) = Config::from_file()
        .and_then(|mut config| config.merge_args(&matches))
        .and_then(|config| nimbus_alt::run(&config, &matches))
    {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
