use std::ffi::OsStr;
use std::{fs, io};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Format {
    Xml,
    Toml,
    Json,
    Yaml,
    Ron,
}

impl Format {
    pub fn from_file_extension(s: &OsStr) -> io::Result<Self> {
        if matches!(s.len(), 3 | 4) {
            // OsStr has PartialEq<&str>, but cannot be matched directly
            if s == "xml" {
                return Ok(Self::Xml);
            } else if s == "toml" {
                return Ok(Self::Toml);
            } else if s == "json" {
                return Ok(Self::Json);
            } else if s == "yaml" || s == "yml" {
                return Ok(Self::Yaml);
            } else if s == "ron" {
                return Ok(Self::Ron);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown extension: {s:?}"),
        ))
    }

    pub fn from_file_name(s: &str) -> io::Result<Self> {
        Self::from_file_extension(fs::canonicalize(s)?.extension().unwrap_or_default())
    }

    pub fn file_extension(&self) -> &str {
        match self {
            Self::Xml => "xml",
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Yaml => "yml",
            Self::Ron => "ron",
        }
    }
}
