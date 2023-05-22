use std::collections::{btree_map, BTreeMap, BTreeSet};
use std::iter;
use std::ops::Index;

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

impl Index<&str> for ModLinks {
    type Output = ModInfo;

    fn index(&self, index: &str) -> &Self::Output {
        self.0.index(index)
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

    pub fn inner(&self) -> &BTreeMap<String, ModInfo> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut BTreeMap<String, ModInfo> {
        &mut self.0
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

    pub fn get(&self, name: &str) -> Option<&ModInfo> {
        self.0.get(name)
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

    pub fn resolve_dependencies<'a>(
        &'a self,
        mod_name: &'a str,
    ) -> Result<BTreeSet<&'a str>, String> {
        self.resolve_dependencies_multi(iter::once(mod_name))
    }

    pub fn resolve_dependencies_multi<'a>(
        &'a self,
        iter: impl IntoIterator<Item = &'a str>,
    ) -> Result<BTreeSet<&'a str>, String> {
        let mut mods_to_resolve: BTreeSet<&'a str> = iter.into_iter().collect();
        let mut resolved_mods: BTreeSet<&'a str> = Default::default();

        while let Some(name) = mods_to_resolve.pop_first() {
            let mod_info = self
                .get(name)
                .ok_or_else(|| format!("Unknown mod: {name}"))?;

            if !resolved_mods.insert(name) {
                continue;
            }

            for dep in mod_info.dependencies.iter() {
                mods_to_resolve.insert(dep);
            }
        }

        Ok(resolved_mods)
    }

    pub fn validate_relations(&self) -> Result<(), Vec<&str>> {
        let mut invalid_mods: Vec<&str> = vec![];

        for (name, mod_info) in self.iter() {
            let mut flag = true;

            for dep in mod_info.dependencies.iter() {
                if self.get(dep).is_none() {
                    flag = false;
                    break;
                }
            }

            if flag {
                for int in mod_info.dependencies.iter() {
                    if self.get(int).is_none() {
                        flag = false;
                        break;
                    }
                }
            }

            if !flag {
                invalid_mods.push(name.as_str());
            }
        }

        if invalid_mods.is_empty() {
            Ok(())
        } else {
            Err(invalid_mods)
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "xml")]
impl ModLinks {
    pub fn into_xml_compat(self) -> crate::ModLinksXmlCompat {
        self.into()
    }

    pub fn into_xml(self) -> Result<String, quick_xml::DeError> {
        self.into_xml_compat().to_xml()
    }

    pub fn into_xml_writer<W: std::fmt::Write>(self, writer: W) -> Result<(), quick_xml::DeError> {
        self.into_xml_compat().to_xml_writer(writer)
    }

    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        crate::ModLinksXmlCompat::from_xml(s).map(Into::into)
    }

    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        crate::ModLinksXmlCompat::from_xml_reader(reader).map(Into::into)
    }
}

#[cfg(feature = "changelog")]
impl ModLinks {
    pub fn changelog_since<'a>(&'a self, old: &'a Self) -> crate::ModLinksChangelog {
        crate::ModLinksChangelog::new(old, self)
    }

    pub fn changelog_until<'a>(&'a self, new: &'a Self) -> crate::ModLinksChangelog {
        new.changelog_since(self)
    }
}
