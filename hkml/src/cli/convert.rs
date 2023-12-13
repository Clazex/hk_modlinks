use clap::Args;

use super::{InArgs, OutArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Convert {
    #[command(flatten)]
    in_args: InArgs,
    #[command(flatten)]
    out_args: OutArgs,
}

impl Run for Convert {
    fn run(self) -> Result {
        let mod_links = self.in_args.read()?;
        self.out_args.write(mod_links)
    }
}
