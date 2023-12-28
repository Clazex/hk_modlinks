use std::fs::{self, File};
use std::io::{self, prelude::*, Cursor, Error as IoError};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use actix_web::http::header::{
    ContentDisposition, HeaderValue, CONTENT_DISPOSITION, CONTENT_LENGTH,
};

use clap::Args;

use itertools::Itertools;

use lazy_static::lazy_static;

use sha2::{Digest, Sha256};

use url::Url;

use zip::{write::FileOptions as ZipFileOptions, ZipArchive, ZipWriter};

use hk_modlinks::{FileDef, Links, Platform};

use super::resolve::read_mods_from_vec_or_file;
use super::{InArgs, Run};
use crate::Result;

lazy_static! {
    pub static ref BEST_COMPRESSION: ZipFileOptions =
        ZipFileOptions::default().compression_level(Some(9));
}

#[derive(Args, Debug, Clone)]
#[group(id = "operation", multiple = false)]
#[group(id = "mod", required = true, multiple = false)]
pub struct Download {
    #[command(flatten)]
    in_args: InArgs,
    /// Mods to be downloaded
    #[arg(required = true, value_name = "MOD", group = "mod")]
    mods: Option<Vec<String>>,
    /// Read mods to be downloaded from file, in which a mod name is stated each line, empty lines are ignored
    #[arg(short = 'f', long = "file", value_name = "MODS FILE", group = "mod")]
    mods_file: Option<PathBuf>,
    /// Output directory or file
    #[arg(short, long, value_name = "FILE|DIR")]
    out: PathBuf,
    /// Do not resolve dependencies
    #[arg(long)]
    no_deps: bool,
    /// Platform to download for, defaults to local platform
    #[arg(long)]
    platform: Option<Platform>,
    /// Unpack mod zips into subdirectories, output path should be a directory.
    #[arg(long, group = "operation")]
    unpack: bool,
    /// Repack unpacked mod zips into a single zip file, output path should be a file.
    #[arg(long, group = "operation")]
    repack: bool,
}

impl Run for Download {
    fn run(self) -> Result {
        let mod_links = self.in_args.read()?;
        let platform = self.platform.map(Into::into).unwrap_or(Platform::LOCAL);

        let out = self.out;
        if self.unpack || !self.repack {
            fs_extra::dir::create_all(&out, true)?;
        } else {
            fs_extra::dir::create_all(out.parent().unwrap(), false)?;
        }

        let mods = read_mods_from_vec_or_file(self.mods, self.mods_file)?;

        let mods = if self.no_deps {
            mods
        } else {
            mod_links
                .resolve_deps(mods.iter().map(String::as_str))
                .map_err(|u| format!("Unknown mods: {}", u.join(", ")))?
                .into_iter()
                .map(ToString::to_string)
                .collect_vec()
        };

        let agent = ureq::AgentBuilder::new().build();
        let mut zip = if self.repack {
            Some(ZipWriter::new(File::create(&out)?))
        } else {
            None
        };

        let mut process_fn: Box<dyn FnMut(_, _) -> Result> = if self.unpack {
            Box::new(|name: String, file: &FileDef| {
                download_to_dir(&agent, file, out.join(&name), name)?;
                Ok(())
            })
        } else if self.repack {
            Box::new(|name: String, file: &FileDef| {
                let zip = zip.as_mut().unwrap();
                zip.add_directory(&name, *BEST_COMPRESSION)?;

                let (buf, file_name) = download_and_verify(&agent, file)?;

                if infer::archive::is_zip(&buf) {
                    let mut mod_zip = ZipArchive::new(Cursor::new(buf))?;
                    for i in 0..mod_zip.len() {
                        let mut file = mod_zip.by_index(i)?;

                        if file.is_dir() {
                            zip.add_directory(
                                format!("{name}/{}", file.name()),
                                *BEST_COMPRESSION,
                            )?;
                        } else {
                            zip.start_file(format!("{name}/{}", file.name()), *BEST_COMPRESSION)?;
                            io::copy(&mut file, zip)?;
                        }
                    }
                } else {
                    zip.start_file(
                        format!(
                            "{name}/{}",
                            file_name.unwrap_or_else(|| format!("{name}.dll"))
                        ),
                        *BEST_COMPRESSION,
                    )?;
                    zip.write_all(&buf)?;
                }

                Ok(())
            })
        } else {
            Box::new(|name: String, file: &FileDef| {
                download_to_zip(&agent, file, out.join(format!("{name}.zip")), name)?;
                Ok(())
            })
        };

        for name in mods {
            println!("Downloading {name}");

            let file = match &mod_links.get(&name).unwrap().links {
                Links::Universal(file) => file,
                Links::PlatformSpecific {
                    windows,
                    mac,
                    linux,
                } => match platform {
                    Platform::Windows => windows,
                    Platform::Mac => mac,
                    Platform::Linux => linux,
                },
            };

            process_fn(name, file)?;
        }

        drop(process_fn);

        if let Some(mut zip) = zip {
            zip.finish()?;
        }

        Ok(())
    }
}

pub fn download_and_verify(
    agent: &ureq::Agent,
    file: &FileDef,
) -> Result<(Vec<u8>, Option<String>)> {
    let resp = agent.get(file.url.as_str()).call()?;

    let disposition = resp
        .header(CONTENT_DISPOSITION.as_str())
        .and_then(|header| {
            ContentDisposition::from_raw(&HeaderValue::from_str(header).unwrap())
                .unwrap()
                .get_filename()
                .map(ToOwned::to_owned)
        })
        .or_else(|| {
            Url::parse(resp.get_url())
                .unwrap()
                .path_segments()
                .and_then(|segments| segments.last().map(ToOwned::to_owned))
        });

    let mut buf = Vec::with_capacity(
        resp.header(CONTENT_LENGTH.as_str())
            .and_then(|s| usize::from_str(s).ok())
            .unwrap_or_default(),
    );
    resp.into_reader().read_to_end(&mut buf)?;

    let hash: [u8; 32] = <Sha256 as Digest>::digest(&buf).into();
    if hash != file.sha256 {
        Err(IoError::other(format!(
            "Hash mismatch!\nExpected: {}\nActual: {}",
            hex::encode_upper(file.sha256),
            hex::encode_upper(hash)
        )))?;
    };

    Ok((buf, disposition))
}

pub fn download_and_zip(
    agent: &ureq::Agent,
    file: &FileDef,
    fallback_name: impl AsRef<str>,
) -> Result<Vec<u8>> {
    let (buf, name) = download_and_verify(agent, file)?;
    let file_name = name.unwrap_or_else(|| format!("{}.dll", fallback_name.as_ref()));

    if infer::archive::is_zip(&buf) {
        return Ok(buf);
    }

    let mut zip_writer = ZipWriter::new(Cursor::new(vec![]));
    zip_writer.set_comment(hex::encode_upper(file.sha256));

    zip_writer.start_file(
        file_name,
        ZipFileOptions::default().compression_level(Some(9)),
    )?;
    zip_writer.write_all(buf.as_slice())?;
    drop(buf);

    Ok(zip_writer.finish()?.into_inner())
}

pub fn download_to_dir(
    agent: &ureq::Agent,
    file: &FileDef,
    dest: impl AsRef<Path>,
    fallback_name: impl AsRef<str>,
) -> Result {
    let (buf, name) = download_and_verify(agent, file)?;
    let dest = dest.as_ref();
    let file_name = name.unwrap_or_else(|| format!("{}.dll", fallback_name.as_ref()));

    if !infer::archive::is_zip(&buf) {
        fs::write(dest.with_file_name(file_name), buf)?;
        return Ok(());
    }

    ZipArchive::new(Cursor::new(buf))?.extract(dest)?;

    Ok(())
}

pub fn download_to_zip(
    agent: &ureq::Agent,
    file: &FileDef,
    dest: impl AsRef<Path>,
    fallback_name: impl AsRef<str>,
) -> Result {
    let buf = download_and_zip(agent, file, fallback_name)?;
    fs::write(dest, buf)?;
    Ok(())
}
