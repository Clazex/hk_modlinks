use std::collections::BTreeSet;

use derive_builder::Builder;

use serde::{Deserialize, Serialize};
use serde_with::{rust::unwrap_or_skip, skip_serializing_none};

use url::Url;

use crate::{Links, Tag, Version};

#[skip_serializing_none]
#[serde_with::apply(
	BTreeSet => #[serde(default, skip_serializing_if = "BTreeSet::is_empty")],
	Option => #[serde(default, with = "unwrap_or_skip")]
)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Builder)]
#[serde(rename_all = "kebab-case")]
#[builder(derive(Debug), setter(into, strip_option))]
pub struct ModInfo {
    #[builder(default)]
    pub display_name: Option<String>,
    #[builder(default)]
    pub description: String,
    pub version: Version,
    pub links: Links,

    #[serde_with(skip_apply)]
    #[builder(default, setter(each(name = "dependency", into)))]
    pub dependencies: BTreeSet<String>,

    #[builder(try_setter)]
    pub repository: Url,

    #[builder(default)]
    pub issues: Option<Url>,

    #[builder(default, setter(each(name = "integration", into)))]
    pub integrations: BTreeSet<String>,

    #[builder(default, setter(each(name = "tag")))]
    pub tags: BTreeSet<Tag>,

    #[builder(default, setter(each(name = "author", into)))]
    pub authors: BTreeSet<String>,
}

impl ModInfo {
    #[inline]
    #[must_use]
    pub fn builder() -> ModInfoBuilder {
        ModInfoBuilder::create_empty()
    }

    #[must_use]
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
            .dependencies(dependencies)
            .repository(repository);

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
    #[inline]
    fn from(value: ModInfo) -> Self {
        value.into_builder()
    }
}
