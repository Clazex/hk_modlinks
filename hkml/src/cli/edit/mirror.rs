use std::fs;
use std::path::{Path, PathBuf};

use clap::Args;

use reqwest::blocking::Client;

use sha2::{Digest, Sha256};

use url::Url;

use hk_modlinks::{get_safe_mod_name, FileDef, Links};

use super::{InArgs, Run};
use crate::cli::download_and_zip;
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Mirror {
    #[arg(short, long)]
    base_url: Url,
    #[arg(short, long)]
    dir: PathBuf,
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

        let mods_dir = base_dir.join("mods");
        fs_extra::dir::create_all(&mods_dir, true)?;
        let mods_dir = fs::canonicalize(mods_dir)?;

        let client = &(*crate::CLIENT);

        mod_links
            .iter_mut()
            .try_for_each(|(name, info)| -> Result {
                let base_name = format!("{}-v{}", get_safe_mod_name(name), info.version);
                println!("Downloading {name} as {base_name}");

                match &mut info.links {
                    Links::Universal(file) => {
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
                        download_and_update(
                            client,
                            windows,
                            &mods_dir,
                            format!("{base_name}-Win.zip"),
                            &mods_url,
                            &base_name,
                        )?;
                        download_and_update(
                            client,
                            mac,
                            &mods_dir,
                            format!("{base_name}-Mac.zip"),
                            &mods_url,
                            &base_name,
                        )?;
                        download_and_update(
                            client,
                            linux,
                            &mods_dir,
                            format!("{base_name}-Linux.zip"),
                            &mods_url,
                            &base_name,
                        )?;
                    }
                };

                Ok(())
            })?;

        println!("Writing new ModLinks.xml");
        fs::write(base_dir.join("ModLinks.xml"), mod_links.to_xml()?)?;

        Ok(())
    }
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

    let zip = download_and_zip(client, file, fallback_name)?;
    fs::write(mods_dir.as_ref().join(file_name), &zip)?;

    file.sha256 = <Sha256 as Digest>::digest(zip).into();
    file.url = mods_url.join(file_name)?;

    Ok(())
}
