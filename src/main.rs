extern crate failure;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
