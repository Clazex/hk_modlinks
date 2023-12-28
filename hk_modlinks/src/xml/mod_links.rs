use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::ModInfo;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModLinks {
    #[serde(rename = "Manifest")]
    mods: Option<BTreeSet<ModInfo>>,
}

impl From<crate::ModLinks> for ModLinks {
    fn from(value: crate::ModLinks) -> Self {
        let mods: BTreeSet<_> = value.into_inner().into_iter().map(Into::into).collect();

        Self {
            mods: match mods.len() {
                0 => None,
                _ => Some(mods),
            },
        }
    }
}

impl From<ModLinks> for crate::ModLinks {
    fn from(value: ModLinks) -> Self {
        Self::new_from_map(
            value
                .mods
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }
}

impl ModLinks {
    #[inline]
    #[must_use]
    pub fn into_general(self) -> crate::ModLinks {
        self.into()
    }

    pub fn to_xml(&self) -> Result<String, quick_xml::DeError> {
        let mut string = String::new();
        self.to_xml_writer(&mut string)?;
        Ok(string)
    }

    pub fn to_xml_writer<W: std::fmt::Write>(
        &self,
        mut writer: W,
    ) -> Result<(), quick_xml::DeError> {
        let mut serializer = quick_xml::se::Serializer::new(&mut writer);
        serializer.indent('\t', 1);
        self.serialize(serializer)
    }

    #[inline]
    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }

    #[inline]
    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_reader(reader)
    }
}
