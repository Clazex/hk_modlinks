use std::collections::BTreeSet;

use derive_builder::Builder;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Links, Version};

#[skip_serializing_none]
#[serde_with::apply(BTreeSet => #[serde(default, skip_serializing_if = "BTreeSet::is_empty")])]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Builder)]
#[serde(rename_all = "kebab-case")]
#[builder(derive(Debug), setter(into, strip_option))]
pub struct ModInfo {
    pub display_name: Option<String>,
    #[builder(default)]
    pub description: String,
    pub version: Version,
    pub links: Links,

    #[serde_with(skip_apply)]
    #[serde(default)]
    #[builder(default, setter(each(name = "dependency", into)))]
    pub dependencies: BTreeSet<String>,

    #[builder(default)]
    pub repository: Option<String>,

    #[builder(default)]
    pub issues: Option<String>,

    #[builder(default, setter(each(name = "integration", into)))]
    pub integrations: BTreeSet<String>,

    #[builder(default, setter(each(name = "tag", into)))]
    pub tags: BTreeSet<String>,

    #[builder(default, setter(each(name = "author", into)))]
    pub authors: BTreeSet<String>,
}

impl ModInfo {
    pub fn builder() -> ModInfoBuilder {
        ModInfoBuilder::create_empty()
    }

    pub fn into_builder(self) -> ModInfoBuilder {
        // Destruct first to ensure new fields get updated
        let Self {
            display_name,
            description,
            version,
            links,
            dependencies,
            repository,
            issues,
            integrations,
            tags,
            authors,
        } = self;

        let mut builder = Self::builder();

        if let Some(display_name) = display_name {
            builder.display_name(display_name);
        }

        builder
            .description(description)
            .version(version)
            .links(links)
            .dependencies(dependencies);

        if let Some(repo) = repository {
            builder.repository(repo);
        }

        if let Some(issues) = issues {
            builder.issues(issues);
        }

        builder
            .integrations(integrations)
            .tags(tags)
            .authors(authors);

        builder
    }
}

impl From<ModInfo> for ModInfoBuilder {
    fn from(value: ModInfo) -> Self {
        value.into_builder()
    }
}
