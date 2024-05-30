use std::error::Error;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = ::grepper::run(grepper::Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    Ok(())
}