mod cli;
mod format;

use cli::*;
pub use format::*;

use std::error::Error;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    Cli::parse().run()
}
