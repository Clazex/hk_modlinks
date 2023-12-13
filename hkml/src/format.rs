use std::ffi::OsStr;
use std::fs;
use std::io::{self, Error as IoError};
use std::path::Path;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Format {
    #[cfg(feature = "xml")]
    Xml,
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
    #[cfg(feature = "ron")]
    Ron,
}

impl Format {
    pub fn from_file_extension(ext: impl AsRef<OsStr>) -> io::Result<Self> {
        use Format::*;

        let ext = ext.as_ref();
        Ok(match ext.to_string_lossy().as_ref() {
            #[cfg(feature = "xml")]
            "xml" => Xml,
            #[cfg(feature = "json")]
            "json" => Json,
            #[cfg(feature = "toml")]
            "toml" => Toml,
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => Yaml,
            #[cfg(feature = "ron")]
            "ron" => Ron,
            _ => Err(IoError::other(format!("Unknown extension: {ext:?}")))?,
        })
    }

    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::from_file_extension(fs::canonicalize(path)?.extension().unwrap_or_default())
    }
}
