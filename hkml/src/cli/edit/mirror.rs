use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::Args;

use reqwest::blocking::Client;
use reqwest::StatusCode;

use sha2::{Digest, Sha256};

use url::Url;

use hk_modlinks::{get_safe_mod_name, FileDef, Links, ModInfo};

use super::{InArgs, Run};
use crate::cli::download_and_zip;
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Mirror {
    #[arg(short, long)]
    base_url: Url,
    #[arg(short, long)]
    dir: PathBuf,
    /// Perform incremental mirroring according to previous mirror located in this directory
    #[arg(short, long)]
    prev: Option<PathBuf>,
    #[command(flatten)]
    in_args: InArgs,
}

impl Run for Mirror {
    fn run(self) -> Result {
        let mut mod_links = self.in_args.read()?;

        let base_url = self.base_url;
        assert!(
            base_url.has_authority() && matches!(base_url.scheme(), "http" | "https"),
            "Invalid base url!"
        );
        let mods_url = base_url.join("mods/")?;

        fs_extra::dir::create_all(&self.dir, true)?;
        let base_dir = fs::canonicalize(self.dir)?;

        println!("Writing base url");
        fs::write(base_dir.join("base-url.txt"), base_url.to_string())?;

        println!("Writing orig ModLinks.xml");
        fs::write(base_dir.join("ModLinks.orig.xml"), mod_links.to_xml()?)?;

        let mods_dir = base_dir.join("mods");
        fs_extra::dir::create_all(&mods_dir, true)?;
        let mods_dir = fs::canonicalize(mods_dir)?;

        let prev_base_dir = self.prev.map(fs::canonicalize).transpose()?;
        let prev_mods_dir = prev_base_dir
            .as_ref()
            .map(|x| x.join("mods"))
            .map(fs::canonicalize)
            .transpose()?;
        let prev_mod_links = prev_base_dir
            .as_ref()
            .map(|x| x.join("ModLinks.xml"))
            .map(InArgs::read_from_file)
            .transpose()?;
        let prev_orig_mod_links = prev_base_dir
            .as_ref()
            .map(|x| x.join("ModLinks.orig.xml"))
            .map(InArgs::read_from_file)
            .transpose()?;
        let prev_base_url = prev_base_dir
            .map(|x| x.join("base-url.txt"))
            .map(fs::read_to_string)
            .transpose()?
            .map(|x| Url::parse(&x))
            .transpose()?;
        let prev_mods_url = prev_base_url.map(|x| x.join("mods/")).transpose()?;

        let client = &(*crate::CLIENT);

        mod_links
            .iter_mut()
            .try_for_each(|(name, info)| -> Result {
                let base_name = format!("{}-v{}", get_safe_mod_name(name), info.version);
                println!("Downloading {name} as {base_name}");

                let prev_orig_info = prev_orig_mod_links.as_ref().and_then(|x| x.get(name));

                match &mut info.links {
                    Links::Universal(file) => {
                        if let Some(ModInfo {
                            links: Links::Universal(prev_orig_file),
                            ..
                        }) = prev_orig_info
                        {
                            if prev_orig_file.sha256 == file.sha256 {
                                let prev_file = match prev_mod_links.as_ref().unwrap().get(name) {
                                    Some(ModInfo {
                                        links: Links::Universal(prev_file),
                                        ..
                                    }) => prev_file,
                                    _ => panic!("Invalid previous mirror"),
                                };

                                if migrate(
                                    prev_file,
                                    file,
                                    prev_mods_dir.as_ref().unwrap(),
                                    &mods_dir,
                                    prev_mods_url.as_ref().unwrap(),
                                    &mods_url,
                                )? {
                                    return Ok(());
                                }
                            }
                        }

                        download_and_update(
                            client,
                            file,
                            &mods_dir,
                            format!("{base_name}.zip"),
                            &mods_url,
                            &base_name,
                        )?;
                    }
                    Links::PlatformSpecific {
                        windows,
                        mac,
                        linux,
                    } => {
                        let (mut windows_ok, mut mac_ok, mut linux_ok) = (false, false, false);

                        if let Some(ModInfo {
                            links:
                                Links::PlatformSpecific {
                                    windows: prev_orig_windows,
                                    mac: prev_orig_mac,
                                    linux: prev_orig_linux,
                                },
                            ..
                        }) = prev_orig_info
                        {
                            let prev_mods_dir = prev_mods_dir.as_ref().unwrap();
                            let prev_mods_url = prev_mods_url.as_ref().unwrap();
                            let Some(ModInfo {
                                links:
                                    Links::PlatformSpecific {
                                        windows: prev_windows,
                                        mac: prev_mac,
                                        linux: prev_linux,
                                    },
                                ..
                            }) = prev_mod_links.as_ref().unwrap().get(name)
                            else {
                                panic!("Invalid previous mirror")
                            };

                            if prev_orig_windows.sha256 == windows.sha256
                                && migrate(
                                    prev_windows,
                                    windows,
                                    prev_mods_dir,
                                    &mods_dir,
                                    prev_mods_url,
                                    &mods_url,
                                )?
                            {
                                windows_ok = true;
                            }

                            if prev_orig_mac.sha256 == mac.sha256
                                && migrate(
                                    prev_mac,
                                    mac,
                                    prev_mods_dir,
                                    &mods_dir,
                                    prev_mods_url,
                                    &mods_url,
                                )?
                            {
                                mac_ok = true;
                            }

                            if prev_orig_linux.sha256 == linux.sha256
                                && migrate(
                                    prev_linux,
                                    linux,
                                    prev_mods_dir,
                                    &mods_dir,
                                    prev_mods_url,
                                    &mods_url,
                                )?
                            {
                                linux_ok = true;
                            }
                        }

                        if !windows_ok {
                            download_and_update(
                                client,
                                windows,
                                &mods_dir,
                                format!("{base_name}-Win.zip"),
                                &mods_url,
                                &base_name,
                            )?;
                        }
                        if !mac_ok {
                            download_and_update(
                                client,
                                mac,
                                &mods_dir,
                                format!("{base_name}-Mac.zip"),
                                &mods_url,
                                &base_name,
                            )?;
                        }
                        if !linux_ok {
                            download_and_update(
                                client,
                                linux,
                                &mods_dir,
                                format!("{base_name}-Linux.zip"),
                                &mods_url,
                                &base_name,
                            )?;
                        }
                    }
                };

                Ok(())
            })?;

        println!("Writing new ModLinks.xml");
        fs::write(base_dir.join("ModLinks.xml"), mod_links.to_xml()?)?;

        Ok(())
    }
}

fn migrate(
    prev_file: &FileDef,
    file: &mut FileDef,
    prev_mods_dir: impl AsRef<Path>,
    mods_dir: impl AsRef<Path>,
    prev_mods_url: &Url,
    mods_url: &Url,
) -> Result<bool> {
    let path = prev_mods_url.make_relative(&prev_file.url).unwrap();
    println!("Migrating {path} from previous mirror");

    file.sha256 = prev_file.sha256;
    file.url = mods_url.join(&path)?;

    let prev_path = match fs::canonicalize(prev_mods_dir.as_ref().join(&path)) {
        Ok(p) => p,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(e) => Err(e)?,
    };

    fs::copy(prev_path, mods_dir.as_ref().join(path))?;

    Ok(true)
}

fn download_and_update(
    client: &Client,
    file: &mut FileDef,
    mods_dir: impl AsRef<Path>,
    file_name: impl AsRef<str>,
    mods_url: &Url,
    fallback_name: impl AsRef<str>,
) -> Result {
    let file_name = file_name.as_ref();

    let zip = match download_and_zip(client, file, fallback_name) {
        Ok(zip) => zip,
        Err(e) => match e.downcast_ref::<reqwest::Error>() {
            Some(req_err) if req_err.status() == Some(StatusCode::NOT_FOUND) => {
                println!("File Not Found! Skipping");
                return Ok(());
            }
            _ => Err(e)?,
        },
    };
    fs::write(mods_dir.as_ref().join(file_name), &zip)?;

    file.sha256 = <Sha256 as Digest>::digest(zip).into();
    file.url = mods_url.join(file_name)?;

    Ok(())
}
