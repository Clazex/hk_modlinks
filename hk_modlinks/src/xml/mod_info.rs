use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use url::Url;

use super::{Authors, Dependencies, Integrations, Links, Tags};
use crate::Version;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Manifest", rename_all = "PascalCase")]
pub struct ModInfo<'a> {
    name: Cow<'a, String>,
    display_name: Option<Cow<'a, String>>,
    description: Cow<'a, String>,
    version: Cow<'a, Version>,
    #[serde(flatten)]
    links: Links<'a>,
    dependencies: Dependencies<'a>,
    repository: Cow<'a, Url>,
    issues: Option<Cow<'a, Url>>,
    #[serde(default, skip_serializing_if = "Integrations::is_empty")]
    integrations: Integrations<'a>,
    #[serde(default, skip_serializing_if = "Tags::is_empty")]
    tags: Tags<'a>,
    #[serde(default, skip_serializing_if = "Authors::is_empty")]
    authors: Authors<'a>,
}

impl<'a> Hash for ModInfo<'a> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> PartialEq for ModInfo<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for ModInfo<'a> {}

impl<'a> From<ModInfo<'a>> for (String, crate::ModInfo) {
    fn from(value: ModInfo<'a>) -> Self {
        (
            value.name.into_owned(),
            crate::ModInfo {
                display_name: value.display_name.map(Cow::into_owned),
                description: value.description.into_owned(),
                version: value.version.into_owned(),
                links: value.links.into(),
                dependencies: value.dependencies.into(),
                repository: value.repository.into_owned(),
                issues: value.issues.map(Cow::into_owned),
                integrations: value.integrations.into(),
                tags: value.tags.into(),
                authors: value.authors.into(),
            },
        )
    }
}

impl<'a> From<(&'a String, &'a crate::ModInfo)> for ModInfo<'a> {
    fn from((name, value): (&'a String, &'a crate::ModInfo)) -> Self {
        Self {
            name: Cow::Borrowed(name),
            display_name: value.display_name.as_ref().map(Cow::Borrowed),
            description: Cow::Borrowed(&value.description),
            version: Cow::Borrowed(&value.version),
            links: (&value.links).into(),
            dependencies: (&value.dependencies).into(),
            repository: Cow::Borrowed(&value.repository),
            issues: value.issues.as_ref().map(Cow::Borrowed),
            integrations: (&value.integrations).into(),
            tags: (&value.tags).into(),
            authors: (&value.authors).into(),
        }
    }
}
