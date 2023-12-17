use std::fs;
use std::path::PathBuf;

use clap::Args;

use itertools::Itertools;

use super::{InArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
#[group(id = "mod", required = true, multiple = false)]
pub struct Resolve {
    #[command(flatten)]
    in_args: InArgs,

    #[arg(required = true, value_name = "MOD", group = "mod")]
    mods: Option<Vec<String>>,
    /// Read mods to be downloaded from file, in which a mod name is stated each line, empty lines are ignored
    #[arg(short = 'f', long = "file", value_name = "MODS FILE", group = "mod")]
    mods_file: Option<PathBuf>,
}

pub fn read_mods_from_vec_or_file(
    mods: Option<Vec<String>>,
    mods_file: Option<PathBuf>,
) -> Result<Vec<String>> {
    Ok(match mods {
        Some(mods) => mods,
        None => fs::read_to_string(mods_file.unwrap())?
            .split(['\n', '\r'])
            .filter(|x| !x.is_empty())
            .map(ToString::to_string)
            .collect_vec(),
    })
}

impl Run for Resolve {
    fn run(self) -> Result {
        let mod_links = self.in_args.read()?;
        let mods = read_mods_from_vec_or_file(self.mods, self.mods_file)?;

		println!("{}", mod_links.resolve_deps(mods.iter().map(String::as_str))?.into_iter().join("\n"));

        Ok(())
    }
}
