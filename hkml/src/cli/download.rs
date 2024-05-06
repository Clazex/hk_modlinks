use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use actix_web::http::header::{ContentDisposition, CONTENT_DISPOSITION};

use clap::Args;

use itertools::Itertools;

use lazy_static::lazy_static;

use reqwest::blocking::Client;

use sha2::{Digest, Sha256};

use zip::{write::SimpleFileOptions as ZipFileOptions, ZipArchive, ZipWriter};

use hk_modlinks::{FileDef, Links, Platform};

use super::resolve::read_mods_from_vec_or_file;
use super::{InArgs, Run};
use crate::{copy_pb_buf_read, copy_pb_slice, Result};

lazy_static! {
    // It is assumed that no mods exceeds the 4 GiB limit
    pub static ref BEST_COMPRESSION: ZipFileOptions =
        ZipFileOptions::default().compression_level(Some(264));
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
        if self.unpack {
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

        let client = &(*crate::CLIENT);
        let mut zip = if self.repack {
            Some(ZipWriter::new(File::create(&out)?))
        } else {
            None
        };

        let mut process_fn: Box<dyn FnMut(_, _) -> Result> = if self.unpack {
            Box::new(|name: String, file: &FileDef| {
                download_to_dir(client, file, out.join(&name), name)?;
                Ok(())
            })
        } else if self.repack {
            Box::new(|name: String, file: &FileDef| {
                let zip = zip.as_mut().unwrap();
                zip.add_directory(&name, *BEST_COMPRESSION)?;

                let (buf, file_name) = download_and_verify(client, file)?;

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
                            let size = file.size() as usize;
                            copy_pb_buf_read(&mut file, zip, Some(size), "Re-compressing")?;
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
                    copy_pb_slice(&buf, zip, "Compressing")?;
                }

                Ok(())
            })
        } else {
            Box::new(|name: String, file: &FileDef| {
                download_to_zip(client, file, &out, name)?;
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

pub fn download_and_verify(client: &Client, file: &FileDef) -> Result<(Vec<u8>, Option<String>)> {
    let resp = client.get(file.url.clone()).send()?.error_for_status()?;

    let disposition = resp
        .headers()
        .get(CONTENT_DISPOSITION.as_str())
        .and_then(|header| {
            ContentDisposition::from_raw(
                &actix_web::http::header::HeaderValue::from_bytes(header.as_bytes())
                    .expect("failed to perform identity transformation on header value"),
            )
            .unwrap()
            .get_filename()
            .map(ToOwned::to_owned)
        })
        .or_else(|| {
            resp.url()
                .path_segments()
                .and_then(|segments| segments.last().map(ToOwned::to_owned))
        });

    let buf = {
        let size = resp.content_length().map(|i| i as usize);
        let mut buf = Vec::with_capacity(size.unwrap_or(crate::DEFAULT_BUF_SIZE));

        let mut resp = resp;
        copy_pb_buf_read(&mut resp, &mut buf, size, "Downloading")?;

        buf
    };

    let hash: [u8; 32] = {
        let mut hasher = <Sha256 as Digest>::new();
        copy_pb_slice(&buf, &mut hasher, "Hashing")?;
        Digest::finalize(hasher).into()
    };
    if hash != file.sha256 {
        Err(io::Error::other(format!(
            "Hash mismatch!\n  Expected: {}\n  Actual: {}",
            hex::encode_upper(file.sha256),
            hex::encode_upper(hash)
        )))?;
    };

    Ok((buf, disposition))
}

pub fn download_and_zip(
    client: &Client,
    file: &FileDef,
    fallback_name: impl AsRef<str>,
) -> Result<Vec<u8>> {
    let (buf, name) = download_and_verify(client, file)?;
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
    copy_pb_slice(&buf, &mut zip_writer, "Compressing")?;
    drop(buf);

    Ok(zip_writer.finish()?.into_inner())
}

pub fn download_to_dir(
    client: &Client,
    file: &FileDef,
    dest: impl AsRef<Path>,
    fallback_name: impl AsRef<str>,
) -> Result {
    let (buf, name) = download_and_verify(client, file)?;
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
    client: &Client,
    file: &FileDef,
    dest: impl AsRef<Path>,
    fallback_name: impl AsRef<str>,
) -> Result {
    let buf = download_and_zip(client, file, fallback_name)?;

    let mut file = File::create(dest)?;
    copy_pb_slice(&buf, &mut file, "Writing")?;
    file.sync_all()?;

    Ok(())
}
