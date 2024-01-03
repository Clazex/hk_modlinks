use clap::Args;

use reqwest::blocking::Client;

use hk_modlinks::{FileDef, Links};

use super::{InArgs, Run};
use crate::cli::download_and_verify;
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Validate {
    #[command(flatten)]
    in_args: InArgs,
    /// Skip validating hash
    #[arg(long)]
    no_hash: bool,
}

impl Run for Validate {
    fn run(self) -> Result {
        let mod_links = self.in_args.read()?;

        mod_links
            .validate_names()
            .map_err(|m| format!("The following mod(s) has invalid name: {}", m.join(", ")))?;

        mod_links.validate_relations().map_err(|m| {
            format!(
                "The following mod(s) contains non-existant mod in their relations: {}",
                m.join(", ")
            )
        })?;

        if self.no_hash {
            return Ok(());
        }

        let client = &(*crate::CLIENT);
        for (name, info) in mod_links {
            match info.links {
                Links::Universal(file) => verify(client, &name, file, None)?,
                Links::PlatformSpecific {
                    windows,
                    mac,
                    linux,
                } => {
                    verify(client, &name, *windows, Some("Windows"))?;
                    verify(client, &name, *mac, Some("Mac"))?;
                    verify(client, &name, *linux, Some("Linux"))?;
                }
            };
        }

        Ok(())
    }
}

fn verify(client: &Client, name: &String, file: FileDef, variant: Option<&'static str>) -> Result {
    print!("Validating {name}");
    match variant {
        Some(variant) => println!(" ({variant})"),
        None => println!(),
    };

    download_and_verify(client, &file)?;

    Ok(())
}
