use std::collections::{btree_map, BTreeMap, BTreeSet, HashSet};
use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};
use serde_with::rust::maps_duplicate_key_is_error;

use crate::{is_valid_mod_name, ModInfo};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ModLinks(#[serde(with = "maps_duplicate_key_is_error")] BTreeMap<String, ModInfo>);

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
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ModLinks {
    type IntoIter = btree_map::Iter<'a, String, ModInfo>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ModLinks {
    type IntoIter = btree_map::IterMut<'a, String, ModInfo>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Index<&str> for ModLinks {
    type Output = ModInfo;

    fn index(&self, index: &str) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<&str> for ModLinks {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.0.get_mut(index).expect("no entry found for key")
    }
}

impl Extend<(String, ModInfo)> for ModLinks {
    fn extend<T: IntoIterator<Item = (String, ModInfo)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl ModLinks {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn new_from_map(mods: BTreeMap<String, ModInfo>) -> Self {
        Self(mods)
    }

    #[must_use]
    pub fn inner(&self) -> &BTreeMap<String, ModInfo> {
        &self.0
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut BTreeMap<String, ModInfo> {
        &mut self.0
    }

    #[must_use]
    pub fn into_inner(self) -> BTreeMap<String, ModInfo> {
        self.0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.0.contains_key(name.as_ref())
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&ModInfo> {
        self.0.get(name.as_ref())
    }

    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut ModInfo> {
        self.0.get_mut(name.as_ref())
    }

    pub fn get_display_name<'a>(&'a self, name: &'a str) -> Option<&str> {
        self.0
            .get(name)
            .map(|info| info.display_name.as_deref().unwrap_or(name))
    }

    pub fn insert(&mut self, name: String, mod_info: ModInfo) -> Option<ModInfo> {
        self.0.insert(name, mod_info)
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<ModInfo> {
        self.0.remove(name.as_ref())
    }

    pub fn entry(&mut self, name: String) -> btree_map::Entry<'_, String, ModInfo> {
        self.0.entry(name)
    }

    pub fn mod_names(&self) -> btree_map::Keys<'_, String, ModInfo> {
        self.0.keys()
    }

    pub fn into_mod_names(self) -> btree_map::IntoKeys<String, ModInfo> {
        self.0.into_keys()
    }

    pub fn iter(&self) -> btree_map::Iter<'_, String, ModInfo> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> btree_map::IterMut<'_, String, ModInfo> {
        self.0.iter_mut()
    }

    pub fn resolve_deps_single<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<HashSet<&'a str>, Vec<&'a str>> {
        self.resolve_deps(std::iter::once(name))
    }

    pub fn resolve_deps<'a, 'b: 'a>(
        &'a self,
        iter: impl IntoIterator<Item = &'b str>,
    ) -> Result<HashSet<&'a str>, Vec<&'a str>> {
        let mut to_resolve: BTreeSet<&'a str> = iter.into_iter().collect();
        let mut resolved: HashSet<&'a str> = Default::default();
        let mut unknown = vec![];

        while let Some(name) = to_resolve.pop_first() {
            let Some(mod_info) = self.get(name) else {
                unknown.push(name);
                continue;
            };

            if !resolved.insert(name) {
                continue;
            }

            for dep in mod_info.dependencies.iter() {
                to_resolve.insert(dep);
            }
        }

        if unknown.is_empty() {
            Ok(resolved)
        } else {
            Err(unknown)
        }
    }

    pub fn validate_names(&self) -> Result<(), Vec<&str>> {
        let invalid: Vec<_> = self
            .mod_names()
            .filter(|i| !is_valid_mod_name(i))
            .map(String::as_str)
            .collect();

        if invalid.is_empty() {
            Ok(())
        } else {
            Err(invalid)
        }
    }

    pub fn validate_relations(&self) -> Result<(), Vec<&str>> {
        let invalid: Vec<_> = self
            .iter()
            .filter_map(|(name, info)| {
                if info
                    .dependencies
                    .iter()
                    .chain(info.integrations.iter())
                    .all(|i| self.contains(i))
                {
                    None
                } else {
                    Some(name.as_str())
                }
            })
            .collect();

        if invalid.is_empty() {
            Ok(())
        } else {
            Err(invalid)
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "xml")]
impl ModLinks {
    #[inline]
    #[must_use]
    fn xml(&self) -> crate::xml::ModLinks<'_> {
        self.into()
    }

    #[inline]
    pub fn to_xml(&self) -> Result<String, quick_xml::DeError> {
        self.xml().to_xml()
    }

    #[inline]
    pub fn to_xml_writer<W: std::fmt::Write>(
        self,
        writer: &mut W,
    ) -> Result<(), quick_xml::DeError> {
        self.xml().to_xml_writer(writer)
    }

    #[inline]
    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        crate::xml::ModLinks::from_xml(s).map(Into::into)
    }

    #[inline]
    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        crate::xml::ModLinks::from_xml_reader(reader).map(Into::into)
    }
}

#[cfg(feature = "changelog")]
impl ModLinks {
    #[inline]
    #[must_use]
    pub fn changelog_since<'a>(&'a self, old: &'a Self) -> crate::ModLinksChangelog {
        crate::ModLinksChangelog::new(old, self)
    }

    #[inline]
    #[must_use]
    pub fn changelog_until<'a>(&'a self, new: &'a Self) -> crate::ModLinksChangelog {
        new.changelog_since(self)
    }
}
