mod cli;
mod format;

use std::error::Error;

use cli::*;
use format::*;

use clap::Parser;

type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result {
    Cli::parse().run()
}
