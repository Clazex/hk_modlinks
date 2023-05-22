mod api_links;
mod file_def;
mod links;
mod mod_info;
mod mod_links;
mod tag;

#[cfg(feature = "xml")]
mod xml;

#[cfg(feature = "convert")]
mod convert;

#[cfg(feature = "changelog-template")]
mod changelog_template;

#[cfg(feature = "changelog")]
mod mod_links_changelog;

pub use api_links::*;
pub use file_def::*;
pub use links::*;
pub use mod_info::*;
pub use mod_links::*;
pub use tag::*;

#[cfg(feature = "xml")]
pub use xml::{ApiLinks as ApiLinksXmlCompat, ModLinks as ModLinksXmlCompat};

#[cfg(feature = "convert")]
pub use convert::*;

#[cfg(feature = "changelog-template")]
pub use changelog_template::*;

#[cfg(feature = "changelog")]
pub use mod_links_changelog::*;
