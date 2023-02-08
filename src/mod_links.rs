use std::collections::{btree_map, BTreeMap};

use serde::{Deserialize, Serialize};

use crate::ModInfo;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ModLinks(BTreeMap<String, ModInfo>);

impl From<BTreeMap<String, ModInfo>> for ModLinks {
    fn from(value: BTreeMap<String, ModInfo>) -> Self {
        Self::new_from_map(value)
    }
}

impl From<ModLinks> for BTreeMap<String, ModInfo> {
    fn from(value: ModLinks) -> Self {
        value.into_inner()
    }
}

impl FromIterator<(String, ModInfo)> for ModLinks {
    fn from_iter<T: IntoIterator<Item = (String, ModInfo)>>(iter: T) -> Self {
        Self::new_from_map(iter.into_iter().collect())
    }
}

impl IntoIterator for ModLinks {
    type IntoIter = btree_map::IntoIter<String, ModInfo>;
    type Item = (String, ModInfo);

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<(String, ModInfo)> for ModLinks {
    fn extend<T: IntoIterator<Item = (String, ModInfo)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl ModLinks {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_from_map(mods: BTreeMap<String, ModInfo>) -> Self {
        Self(mods)
    }

    pub fn into_inner(self) -> BTreeMap<String, ModInfo> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_mod(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn insert(&mut self, name: String, mod_info: ModInfo) -> Option<ModInfo> {
        self.0.insert(name, mod_info)
    }

    pub fn remove(&mut self, name: &str) -> Option<ModInfo> {
        self.0.remove(name)
    }

    pub fn entry(&mut self, name: String) -> btree_map::Entry<'_, String, ModInfo> {
        self.0.entry(name)
    }

    pub fn mod_names(&self) -> btree_map::Keys<'_, String, ModInfo> {
        self.0.keys()
    }

    pub fn iter(&self) -> btree_map::Iter<'_, String, ModInfo> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> btree_map::IterMut<'_, String, ModInfo> {
        self.0.iter_mut()
    }
}

#[cfg(feature = "json")]
impl ModLinks {
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

#[cfg(feature = "xml")]
impl ModLinks {
    pub fn into_xml_compat(self) -> crate::ModLinksXmlCompat {
        self.into()
    }

    pub fn to_xml(self) -> Result<String, quick_xml::DeError> {
        self.into_xml_compat().to_xml()
    }

    pub fn to_xml_writer<W: std::io::Write>(self, writer: W) -> Result<(), quick_xml::DeError> {
        self.into_xml_compat().to_xml_writer(writer)
    }

    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        crate::ModLinksXmlCompat::from_xml(s).map(Into::into)
    }

    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        crate::ModLinksXmlCompat::from_xml_reader(reader).map(Into::into)
    }
}
