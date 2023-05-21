use std::error::Error;

use clap::Args;

use super::{InArgs, Run};

#[derive(Args, Debug, Clone)]
pub struct Resolve {
    #[command(flatten)]
    in_args: InArgs,

    #[arg(value_name = "MOD")]
    mods: Vec<String>,
}

impl Run for Resolve {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let mod_links = self.in_args.read()?;

        for i in mod_links.resolve_dependencies_multi(self.mods.iter().map(String::as_str))? {
            println!("{i}");
        }

        Ok(())
    }
}