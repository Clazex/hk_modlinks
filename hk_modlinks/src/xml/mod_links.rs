use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::ModInfo;

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
    pub fn into_general(self) -> crate::ModLinks {
        self.into()
    }

    pub fn to_xml(&self) -> Result<String, quick_xml::DeError> {
        quick_xml::se::to_string(&self)
    }

    pub fn to_xml_writer<W: std::fmt::Write>(&self, writer: W) -> Result<(), quick_xml::DeError> {
        quick_xml::se::to_writer(writer, &self)
    }

    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }

    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_reader(reader)
    }
}
