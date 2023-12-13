mod changelog;
mod convert;
mod merge;
mod resolve;
mod validate;

use std::fs::File;
use std::io::{self, prelude::*};

use clap::{Args, Parser};

use changelog::*;
use convert::*;
use hk_modlinks::ModLinks;
use merge::*;
use resolve::*;
use validate::*;

use crate::{Format, Result};

pub const MODLINKS_DEFAULT_CAPACITY: usize = 160 * 1024 * 1024;

pub trait Run {
    fn run(self) -> Result;
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub enum Cli {
    Resolve(Resolve),
    Convert(Convert),
    Merge(Merge),
    Validate(Validate),
    Changelog(Changelog),
}

impl Run for Cli {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Cli::Resolve(resolve) => resolve.run(),
            Cli::Convert(convert) => convert.run(),
            Cli::Merge(merge) => merge.run(),
            Cli::Validate(validate) => validate.run(),
            Cli::Changelog(changelog) => changelog.run(),
        }
    }
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false, id = "input")]
pub struct InArgs {
    #[arg(long, value_name = "FORMAT")]
    stdin: Option<Format>,
    #[arg(short, value_name = "FILE")]
    r#in: Option<String>,
}

impl InArgs {
    fn read(self) -> Result<ModLinks> {
        let mut buf: Vec<u8> = Vec::with_capacity(MODLINKS_DEFAULT_CAPACITY);

        let in_format: Format = match &self.r#in {
            Some(path) => {
                File::open(path)?.read_to_end(&mut buf)?;
                Format::from_path(path)?
            }
            None => {
                io::stdin().read_to_end(&mut buf)?;
                self.stdin.unwrap()
            }
        };

        let slice = buf.as_slice();
        Ok(match in_format {
            #[cfg(feature = "xml")]
            Format::Xml => ModLinks::from_xml_reader(slice)?,
            #[cfg(feature = "json")]
            Format::Json => ModLinks::from_json_reader(slice)?,
            #[cfg(feature = "toml")]
            Format::Toml => ModLinks::from_toml(String::from_utf8(buf)?.as_str())?,
            #[cfg(feature = "yaml")]
            Format::Yaml => ModLinks::from_yaml_reader(slice)?,
            #[cfg(feature = "ron")]
            Format::Ron => ModLinks::from_ron_reader(slice)?,
        })
    }

    fn read_from(stdin: Option<Format>, r#in: Option<String>) -> Result<ModLinks> {
        Self { stdin, r#in }.read()
    }
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false, id = "output")]
pub struct OutArgs {
    #[arg(long, value_name = "FORMAT")]
    stdout: Option<Format>,
    #[arg(short, value_name = "FILE")]
    out: Option<String>,
}

impl OutArgs {
    fn write(self, mod_links: ModLinks) -> Result {
        let (mut writer, out_format): (Box<dyn Write>, _) = match &self.out {
            Some(path) => (Box::new(File::create(path)?), Format::from_path(path)?),
            None => (Box::new(io::stdout().lock()), self.stdout.unwrap()),
        };

        match out_format {
            #[cfg(feature = "xml")]
            Format::Xml => writer.write_all(mod_links.into_xml()?.as_bytes())?,
            #[cfg(feature = "json")]
            Format::Json => mod_links.to_json_writer(writer)?,
            #[cfg(feature = "toml")]
            Format::Toml => writer.write_all(mod_links.to_toml()?.as_bytes())?,
            #[cfg(feature = "yaml")]
            Format::Yaml => mod_links.to_yaml_writer(writer)?,
            #[cfg(feature = "ron")]
            Format::Ron => mod_links.to_ron_writer(writer)?,
        }

        Ok(())
    }
}
