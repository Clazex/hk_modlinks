use std::collections::HashMap;
use std::fs;

use clap::Args;

use itertools::Itertools;

use super::{InArgs, ModTranslation, OutArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Apply {
    #[command(flatten)]
    in_args: InArgs,
    #[command(flatten)]
    out_args: OutArgs,
    #[arg(short, long)]
    lang_file: String,
}

impl Run for Apply {
    fn run(self) -> Result {
        let mut mod_links = self.in_args.read()?;

        let mut lang: HashMap<String, ModTranslation> =
            toml::from_str(fs::read_to_string(self.lang_file)?.as_str())?;

        for (name, info) in mod_links.iter_mut() {
            let Some(mod_lang) = lang.remove(name) else {
                eprintln!("Missing translation for mod: {name}");
                continue;
            };

            info.description = if *name == mod_lang.name {
                mod_lang.desc
            } else {
                format!("({})\n{}", mod_lang.name, mod_lang.desc)
            };
        }

        if !lang.is_empty() {
            eprintln!("Redundant translation for mods: {}", lang.keys().join(", "));
        }

        self.out_args.write(mod_links)
    }
}
