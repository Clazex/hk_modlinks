mod changelog;
mod convert;
mod download;
mod edit;
mod resolve;
mod validate;

use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;

use clap::{Args, Parser};

use hk_modlinks::ModLinks;

use changelog::*;
use convert::*;
use download::*;
use edit::*;
use resolve::*;
use validate::*;

use crate::{Format, Result};

const MODLINKS_DEFAULT_CAPACITY: usize = 3 * 128 * 1024;

pub trait Run {
    fn run(self) -> Result;
}

#[macro_export]
macro_rules! impl_run_inner {
	($type:ty; $($variant:ident),+) => {
		impl Run for $type {
			fn run(self) -> $crate::Result {
				match self {
					$(
						Self::$variant(inner) => inner.run(),
					)+
				}
			}
		}
	};
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub enum Cli {
    /// Resolve dependency of given mod(s) in the modlinks
    Resolve(Resolve),
    /// Download mod(s) with dependencies, as zip files by defaults
    Download(Download),
    /// Convert modlinks between different formats
    Convert(Convert),
    /// Validate mod relationships in the modlinks
    Validate(Validate),
    /// Generate changelog between two modlinks
    Changelog(Changelog),
    /// Edit the modlink
    #[command(subcommand)]
    Edit(Edit),
}

impl_run_inner! {
    Cli;
    Resolve,
    Download,
    Convert,
    Validate,
    Changelog,
    Edit
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false, id = "input")]
pub struct InArgs {
    /// Specify path to modlinks file
    #[arg(short, value_name = "FILE")]
    r#in: Option<PathBuf>,
    /// Specify modlinks format and read from stdin
    #[arg(long, value_name = "FORMAT")]
    stdin: Option<Format>,
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

    fn read_from_file(path: PathBuf) -> Result<ModLinks> {
        Self {
            r#in: Some(path),
            stdin: None,
        }
        .read()
    }
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false, id = "output")]
pub struct OutArgs {
    /// Path to write modlinks file to
    #[arg(short, value_name = "FILE")]
    out: Option<PathBuf>,
    /// Format of modlinks, writes to stdout
    #[arg(long, value_name = "FORMAT")]
    stdout: Option<Format>,
}

impl OutArgs {
    fn write(self, mod_links: ModLinks) -> Result {
        let (mut writer, out_format): (Box<dyn Write>, _) = match &self.out {
            Some(path) => (Box::new(File::create(path)?), Format::from_path(path)?),
            None => (Box::new(io::stdout().lock()), self.stdout.unwrap()),
        };

        match out_format {
            #[cfg(feature = "xml")]
            Format::Xml => writer.write_all(mod_links.to_xml()?.as_bytes())?,
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
