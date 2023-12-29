#![forbid(unsafe_code)]

mod api_links;
mod file_def;
mod links;
mod mod_info;
mod mod_links;
mod platform;
mod tag;
mod version;

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

#[cfg(feature = "changelog")]
pub use mod_links_changelog::*;

#[cfg(feature = "changelog-template")]
pub use changelog_template::*;

/// Test if a string is a valid mod name.
///
/// The regex equivalent of this function is `[A-Za-z][^\\/:*?<>"|]+`.
pub fn is_valid_mod_name(name: impl AsRef<str>) -> bool {
    let mut chars = name.as_ref().chars();

    if !matches!(chars.next(), Some('A'..='Z' | 'a'..='z')) {
        return false;
    }

    let rest = chars.as_str();
    !rest.is_empty() && !rest.contains(['\\', '/', ':', '*', '?', '<', '>', '"', '|'])
}

/// Test if a url is a valid url for [`FileDef`].
///
/// This namely tests if the url's scheme is `http` or `https`.
pub fn is_valid_file_url(url: &url::Url) -> bool {
    matches!(url.scheme(), "http" | "https")
}

/// Find a counterpart for mod name that behaves well when used as filesystem
/// path, URL path and part of XML. Does not promise uniqueness, but should be
/// sufficiently unique.
///
/// Steps in detail:
/// 1. Split the string into parts by characters: ` ` (space), `[` and `]`,
/// empty parts are discarded throughout the process;
/// 2. Remove all characters that are neither ascii-alphanumeric nor one of
/// `.`, `-`, `_`;
/// 4. Convert first character in each part to ascii uppercase;
/// 5. Join all the parts together.
#[must_use]
pub fn get_safe_mod_name(name: &str) -> String {
    name.split([
        ' ', // Not good for URI
        '[', ']', // Not good for XML CDATA
    ])
    .filter(|s| !s.is_empty())
    .map(|d| {
        d.replace(
            |c: char| !matches!(c, '.' | '-' | '_') && !c.is_ascii_alphanumeric(),
            "",
        )
    })
    .filter(|s| !s.is_empty())
    .map(|mut s| {
        s[0..1].make_ascii_uppercase();
        s
    })
    .collect::<String>()
}
