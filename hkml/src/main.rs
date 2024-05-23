mod cli;
mod format;
mod progress;

#[cfg(not(any(target_os = "windows", target_os = "mac", target_os = "linux")))]
compile_error!("This crate only supports Windows, Mac OS or Linux");

use std::error::Error;

use actix_web::http::header::{ACCEPT, CACHE_CONTROL, CONNECTION};

use clap::Parser;

use lazy_static::lazy_static;

use ureq::{Agent, MiddlewareNext, Request};

use cli::*;
use format::*;
use progress::*;

type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

const USER_AGENT: &str = concat!("hkml/", env!("CARGO_PKG_VERSION"));
const DEFAULT_BUF_SIZE: usize = 64 * 1024;
const MODLINKS_DEFAULT_CAPACITY: usize = 3 * 128 * 1024;

lazy_static! {
    static ref AGENT: Agent = ureq::builder()
        .user_agent(USER_AGENT)
        .middleware(|request: Request, next: MiddlewareNext<'_>| {
            next.handle(
                request
                    .set(CONNECTION.as_str(), "keep-alive")
                    .set(CACHE_CONTROL.as_str(), "no-cache, no-store")
                    .set(
                        ACCEPT.as_str(),
                        "application/octet-stream, application/zip, application/x-msdownload",
                    ),
            )
        })
        .build();
}

fn main() -> Result {
    Cli::parse().run()
}
