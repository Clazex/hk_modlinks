use std::error::Error;

use clap::Args;

use hk_modlinks::ModLinks;

use super::{InArgs, OutArgs, Run};

#[derive(Args, Debug, Clone)]
pub struct Merge {
    #[arg(required(true))]
    files: Vec<String>,

    #[command(flatten)]
    out_args: OutArgs,
}

impl Run for Merge {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut mod_links = ModLinks::new();

        for file in self.files {
            for (name, mod_info) in InArgs::read_from(None, Some(file))? {
                mod_links.insert(name, mod_info);
            }
        }

        self.out_args.write(mod_links)
    }
}
