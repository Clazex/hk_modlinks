use std::collections::HashMap;
use std::fs;

use clap::Args;

use super::{InArgs, ModTranslation, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Generate {
    #[command(flatten)]
    in_args: InArgs,
    #[arg(short, long)]
    lang_file: String,
}

impl Run for Generate {
    fn run(self) -> Result {
        let mod_links = self.in_args.read()?;
        let mut lang: HashMap<String, ModTranslation> = Default::default();

        for (name, info) in mod_links {
            lang.insert(
                name.clone(),
                ModTranslation {
                    name,
                    desc: info.description,
                },
            );
        }

        fs::write(self.lang_file, toml::to_string_pretty(&lang)?)?;

        Ok(())
    }
}
