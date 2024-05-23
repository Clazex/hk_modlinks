use clap::Args;

use ureq::Agent;

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

        for (name, info) in mod_links {
            match info.links {
                Links::Universal(file) => verify(crate::AGENT.clone(), &name, file, None)?,
                Links::PlatformSpecific {
                    windows,
                    mac,
                    linux,
                } => {
                    verify(crate::AGENT.clone(), &name, *windows, Some("Windows"))?;
                    verify(crate::AGENT.clone(), &name, *mac, Some("Mac"))?;
                    verify(crate::AGENT.clone(), &name, *linux, Some("Linux"))?;
                }
            };
        }

        Ok(())
    }
}

fn verify(agent: Agent, name: &String, file: FileDef, variant: Option<&'static str>) -> Result {
    print!("Validating {name}");
    match variant {
        Some(variant) => println!(" ({variant})"),
        None => println!(),
    };

    download_and_verify(agent, &file)?;

    Ok(())
}
