extern crate failure;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
