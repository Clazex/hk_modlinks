use std::path::PathBuf;

use clap::Args;

use hk_modlinks::ModLinks;

use super::{InArgs, OutArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Merge {
    #[arg(required(true))]
    files: Vec<PathBuf>,

    #[command(flatten)]
    out_args: OutArgs,
}

impl Run for Merge {
    fn run(self) -> Result {
        let mut mod_links = ModLinks::new();

        for file in self.files {
            for (name, mod_info) in InArgs::read_from_file(file)? {
                mod_links.insert(name, mod_info);
            }
        }

        self.out_args.write(mod_links)
    }
}
