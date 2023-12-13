use std::error::Error;

use clap::Args;

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

        if let Err(mut mods) = mod_links.validate_relations() {
            let mut msg =
                String::from("The following mods contains non-existant mod in their relations: ");

            msg.push_str(mods.pop().unwrap());

            for mod_name in mods {
                msg.push_str(", ");
                msg.push_str(mod_name);
            }

            Err(msg)?;
        }

        Ok(())
    }
}
