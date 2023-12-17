mod cli;
mod format;

#[cfg(not(any(target_os = "windows", target_os = "mac", target_os = "linux")))]
compile_error!("This crate only supports Windows, Mac OS or Linux");

use std::error::Error;

use cli::*;
use format::*;

use clap::Parser;

type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result {
    Cli::parse().run()
}
