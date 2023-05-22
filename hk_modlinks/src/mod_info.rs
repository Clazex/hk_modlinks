use std::collections::BTreeSet;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{Links, Tag};

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[builder(derive(Debug), setter(into, strip_option))]
pub struct ModInfo {
    #[builder(default)]
    pub description: String,
    pub version: String,
    pub links: Links,

    #[serde(default)]
    #[builder(default, setter(each(name = "dependency", into)))]
    pub dependencies: BTreeSet<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repository: Option<String>,

    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    #[builder(default, setter(each(name = "integration", into)))]
    pub integrations: BTreeSet<String>,

    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    #[builder(default, setter(each(name = "tag", into)))]
    pub tags: BTreeSet<Tag>,

    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    #[builder(default, setter(each(name = "author", into)))]
    pub authors: BTreeSet<String>,
}

impl ModInfo {
    pub fn builder() -> ModInfoBuilder {
        ModInfoBuilder::create_empty()
    }

    pub fn into_builder(self) -> ModInfoBuilder {
        let mut builder = Self::builder();

        builder
            .description(self.description)
            .version(self.version)
            .links(self.links)
            .dependencies(self.dependencies);

        if let Some(repo) = self.repository {
            builder.repository(repo);
        }

        builder
            .integrations(self.integrations)
            .tags(self.tags)
            .authors(self.authors);

        builder
    }
}

impl From<ModInfo> for ModInfoBuilder {
    fn from(value: ModInfo) -> Self {
        value.into_builder()
    }
}
