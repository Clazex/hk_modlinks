mod api_links;
mod file_def;
mod links;
mod mod_info;
mod mod_links;
mod tag;

#[cfg(feature = "xml")]
mod xml;

pub use api_links::*;
pub use file_def::*;
pub use links::*;
pub use mod_info::*;
pub use mod_links::*;
pub use tag::*;

#[cfg(feature = "xml")]
pub use xml::ApiLinks as ApiLinksXmlCompat;

#[cfg(feature = "xml")]
pub use xml::ModLinks as ModLinksXmlCompat;
