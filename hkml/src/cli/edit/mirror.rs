use std::fs;
use std::io::{self, prelude::*, Cursor};
use std::path::Path;
use std::str::FromStr;

use actix_web::http::header::{
    ContentDisposition, HeaderValue, CONTENT_DISPOSITION, CONTENT_LENGTH,
};

use clap::Args;

use sha2::{Digest, Sha256};

use url::Url;

use zip::{write::FileOptions as ZipFileOptions, ZipWriter};

use hk_modlinks::{get_safe_mod_name, FileDef, Links};

use super::{InArgs, Run};
use crate::Result;

#[derive(Args, Debug, Clone)]
pub struct Mirror {
    #[arg(short, long)]
    base_url: Url,
    #[arg(short, long)]
    dir: String,
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

        fs_extra::dir::create_all(self.dir.as_str(), true)?;
        let base_dir = fs::canonicalize(self.dir)?;

        let mods_dir = base_dir.join("mods");
        fs_extra::dir::create_all(&mods_dir, true)?;
        let mods_dir = fs::canonicalize(mods_dir)?;

        let agent = ureq::AgentBuilder::new().build();

        mod_links
            .iter_mut()
            .try_for_each(|(name, info)| -> Result {
                let base_name = format!("{}-v{}", get_safe_mod_name(name), info.version);
                println!("Downloading {name} as {base_name}");

                match &mut info.links {
                    Links::Universal(file) => {
                        download_and_update(
                            file,
                            &agent,
                            &mods_url,
                            format!("{base_name}.zip"),
                            &mods_dir,
                            &base_name,
                        )?;
                    }
                    Links::PlatformDependent {
                        windows,
                        mac,
                        linux,
                    } => {
                        download_and_update(
                            windows,
                            &agent,
                            &mods_url,
                            format!("{base_name}-Win.zip"),
                            &mods_dir,
                            &base_name,
                        )?;
                        download_and_update(
                            mac,
                            &agent,
                            &mods_url,
                            format!("{base_name}-Mac.zip"),
                            &mods_dir,
                            &base_name,
                        )?;
                        download_and_update(
                            linux,
                            &agent,
                            &mods_url,
                            format!("{base_name}-Linux.zip"),
                            &mods_dir,
                            &base_name,
                        )?;
                    }
                };

                Ok(())
            })?;

        println!("Writing new ModLinks.xml");
        fs::write(base_dir.join("ModLinks.xml"), mod_links.into_xml()?)?;

        Ok(())
    }
}

fn download_and_update(
    file: &mut FileDef,
    agent: &ureq::Agent,
    mods_url: &Url,
    file_name: impl AsRef<str>,
    mods_dir: impl AsRef<Path>,
    fallback_name: impl AsRef<str>,
) -> Result {
    let file_name = file_name.as_ref();

    file.sha256 = download_to_zip(
        agent,
        &file.url,
        fallback_name,
        mods_dir.as_ref().join(file_name),
        file.sha256,
    )?;
    file.url = mods_url.join(file_name)?.into();

    Ok(())
}

fn download_to_zip(
    agent: &ureq::Agent,
    url: impl AsRef<str>,
    fallback_name: impl AsRef<str>,
    dest: impl AsRef<Path>,
    sha256: [u8; 32],
) -> Result<[u8; 32]> {
    let resp = agent.get(url.as_ref()).call()?;
    let file_name = get_file_name_for_zip(&resp, fallback_name);

    let buf = {
        let mut buf = Vec::with_capacity(
            resp.header(CONTENT_LENGTH.as_str())
                .and_then(|s| usize::from_str(s).ok())
                .unwrap_or_default(),
        );
        resp.into_reader().read_to_end(&mut buf)?;
        buf
    };

    if <Sha256 as Digest>::digest(&buf) != sha256.into() {
        Err(io::Error::new(io::ErrorKind::Other, "Hash mismatch!"))?;
    };

    if infer::archive::is_zip(&buf) {
        fs::write(dest, buf)?;
        return Ok(sha256);
    }

    let mut zip_writer = ZipWriter::new(Cursor::new(vec![]));
    zip_writer.set_comment(hex::encode_upper(sha256));

    zip_writer.start_file(
        file_name,
        ZipFileOptions::default().compression_level(Some(9)),
    )?;
    zip_writer.write_all(buf.as_slice())?;
    drop(buf);

    let zip = zip_writer.finish()?.into_inner();
    fs::write(dest, &zip)?;

    Ok(<Sha256 as Digest>::digest(zip).into())
}

fn get_file_name_for_zip(resp: &ureq::Response, fallback: impl AsRef<str>) -> String {
    resp.header(CONTENT_DISPOSITION.as_str())
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
        })
        .unwrap_or_else(|| fallback.as_ref().to_string())
}
