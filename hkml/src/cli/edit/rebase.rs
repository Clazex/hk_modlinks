use std::error::Error;

use clap::{Args, Subcommand};

use hk_modlinks::{get_safe_mod_name, FileDef, Links, ModInfo};

use super::{InArgs, OutArgs, Run};

const DEFAULT_MODLINKS_BASE: &str = "https://github.com/hk-modding/modlinks/";

#[derive(Args, Debug, Clone)]
pub struct Rebase {
    #[command(flatten)]
    in_args: InArgs,
    #[command(flatten)]
    out_args: OutArgs,
    #[command(subcommand)]
    command: Operation,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Operation {
    /// Replace a base url with another
    Replace {
        #[arg(default_value = DEFAULT_MODLINKS_BASE)]
        from: String,
        to: String,
    },
    /// Substitute all links under pre-defined rules with a specified root
    Root { root: String },
}

type RebaseFn = Box<dyn FnMut(&str, &mut ModInfo)>;

impl Run for Rebase {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut mod_links = self.in_args.read()?;

        let mut rebase_fn = match self.command {
            Operation::Replace { from, to } => replace_fn(from, to),
            Operation::Root { root } => root_fn(root),
        };

        for (name, info) in mod_links.iter_mut() {
            rebase_fn(name.as_str(), info);
        }

        self.out_args.write(mod_links)
    }
}

fn replace_fn(from: String, to: String) -> RebaseFn {
    let edit_fn = move |file: &mut FileDef| {
        if let Some(remainder) = file.url.strip_prefix(from.as_str()) {
            file.url = format!("{to}{remainder}");
        }
    };

    Box::new(move |_, info: &mut ModInfo| match &mut info.links {
        Links::Universal(universal) => edit_fn(universal),
        Links::PlatformDependent {
            windows,
            mac,
            linux,
        } => {
            edit_fn(windows);
            edit_fn(mac);
            edit_fn(linux);
        }
    })
}

fn root_fn(root: String) -> RebaseFn {
    let root_1 = root.clone();
    let uni_edit_fn = move |file: &mut FileDef, name: &str, version: &str| {
        file.url = format!("{root_1}/mods/{name}-v{version}.zip");
    };

    let plat_edit_fn = move |file: &mut FileDef, name: &str, version: &str, platform: &str| {
        file.url = format!("{root}/mods/{name}-v{version}-{platform}.zip");
    };

    Box::new(move |name: &str, info: &mut ModInfo| {
        let ModInfo { version, .. } = info;
        let name = get_safe_mod_name(name);

        match &mut info.links {
            Links::Universal(universal) => uni_edit_fn(universal, name.as_str(), version),
            Links::PlatformDependent {
                windows,
                mac,
                linux,
            } => {
                plat_edit_fn(windows, name.as_str(), version, "Win");
                plat_edit_fn(mac, name.as_str(), version, "Mac");
                plat_edit_fn(linux, name.as_str(), version, "Linux");
            }
        }
    })
}