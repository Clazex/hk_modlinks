use std::fs::File;
use std::io::Write;

use clap::Args;

use super::{InArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Changelog {
    #[arg(value_name = "OLD FILE")]
    from: String,
    #[arg(value_name = "NEW FILE")]
    to: String,

    #[arg(short, long, value_name = "FILE")]
    out: Option<String>,
}

impl Run for Changelog {
    fn run(self) -> Result {
        let old_mod_links = InArgs::read_from(None, Some(self.from))?;
        let new_mod_links = InArgs::read_from(None, Some(self.to))?;

        let changelog = new_mod_links
            .changelog_since(&old_mod_links)
            .to_markdown()?;

        match self.out {
            Some(path) => File::create(path)?.write_all(changelog.as_bytes())?,
            None => println!("{changelog}"),
        };

        Ok(())
    }
}
