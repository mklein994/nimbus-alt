extern crate nimbus_alt;

fn main() {
    if let Err(e) = nimbus_alt::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
