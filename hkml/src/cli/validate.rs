use std::io;

use clap::Args;

use sha2::{Digest, Sha256};

use hk_modlinks::{FileDef, Links};

use super::{InArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Validate {
    #[command(flatten)]
    in_args: InArgs,
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

        let agent = ureq::AgentBuilder::new().build();
        for (name, info) in mod_links {
            match info.links {
                Links::Universal(file) => verify(&agent, &name, file, None),
                Links::PlatformSpecific {
                    windows,
                    mac,
                    linux,
                } => {
                    verify(&agent, &name, *windows, Some("Windows"));
                    verify(&agent, &name, *mac, Some("Mac"));
                    verify(&agent, &name, *linux, Some("Linux"));
                }
            };
        }

        Ok(())
    }
}

fn verify(agent: &ureq::Agent, name: &String, file: FileDef, variant: Option<&'static str>) {
    let mut hasher = <Sha256 as Digest>::new();
    io::copy(
        &mut agent.get(file.url.as_str()).call().unwrap().into_reader(),
        &mut hasher,
    )
    .unwrap();
    let hash: [u8; 32] = <Sha256 as Digest>::finalize(hasher).into();

    if hash != file.sha256 {
        eprint!("Hash mismatch for {name}");
        if variant.is_some() {
            eprint!(" ({})", variant.unwrap())
        }
        eprintln!();
        eprintln!("  Expected: {}", hex::encode_upper(file.sha256));
        eprintln!("  Actual:   {}", hex::encode_upper(hash));
    }
}
