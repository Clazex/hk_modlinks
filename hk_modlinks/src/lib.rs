#![forbid(unsafe_code)]

mod api_links;
mod file_def;
mod links;
mod mod_info;
mod mod_links;
mod platform;
mod tag;
mod version;

#[cfg(feature = "safe-name")]
mod safe_name;

#[cfg(feature = "xml")]
mod xml;

#[cfg(feature = "convert")]
mod convert;

#[cfg(feature = "changelog")]
mod mod_links_changelog;

#[cfg(feature = "changelog-template")]
mod changelog_template;

pub use api_links::*;
pub use file_def::*;
pub use links::*;
pub use mod_info::*;
pub use mod_links::*;
pub use platform::*;
pub use tag::*;
pub use version::*;

#[cfg(feature = "safe-name")]
pub use safe_name::*;

#[cfg(feature = "xml")]
pub use xml::{ApiLinks as ApiLinksXmlCompat, ModLinks as ModLinksXmlCompat};

#[cfg(feature = "convert")]
pub use convert::*;

#[cfg(feature = "changelog")]
pub use mod_links_changelog::*;

#[cfg(feature = "changelog-template")]
pub use changelog_template::*;
