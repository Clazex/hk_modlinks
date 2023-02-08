use std::collections::BTreeSet;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{Links, Tag};

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[builder(derive(Debug), setter(into, strip_option))]
pub struct ModInfo {
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
}

#[cfg(feature = "json")]
impl ModInfo {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self).map_err(Into::into)
    }

    pub fn to_json_writer<W: std::io::Write>(&self, writer: W) -> serde_json::Result<()> {
        serde_json::to_writer_pretty(writer, &self).map_err(Into::into)
    }

    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s).map_err(Into::into)
    }

    pub fn from_json_reader<R: std::io::Read>(reader: R) -> serde_json::Result<Self> {
        serde_json::from_reader(reader).map_err(Into::into)
    }
}
