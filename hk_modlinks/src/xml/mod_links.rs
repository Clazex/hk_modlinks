use std::collections::HashSet;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use serde_with::rust::sets_duplicate_value_is_error;

use const_format::concatcp;

use super::{ModInfo, MODLINKS_SCHEMA_URL, NAMESPACE, XSD, XSI};

const ATTRS: &[(&str, &str)] = &[
    ("@xmlns", NAMESPACE),
    ("@xmlns:xsd", XSD),
    ("@xmlns:xsi", XSI),
    (
        "@xsi:schemaLocation",
        concatcp!(NAMESPACE, ' ', MODLINKS_SCHEMA_URL),
    ),
];

#[derive(Debug, Clone, Deserialize)]
pub struct ModLinks<'a> {
    #[serde(
        rename = "Manifest",
        deserialize_with = "sets_duplicate_value_is_error::deserialize"
    )]
    mods: HashSet<ModInfo<'a>>,
}

impl<'a> From<ModLinks<'a>> for crate::ModLinks {
    #[inline]
    fn from(value: ModLinks<'a>) -> Self {
        value.mods.into_iter().map(Into::into).collect()
    }
}

impl<'a> From<&'a crate::ModLinks> for ModLinks<'a> {
    #[inline]
    fn from(value: &'a crate::ModLinks) -> Self {
        Self {
            mods: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> Serialize for ModLinks<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let empty = self.mods.is_empty();

        let mut ser =
            serializer.serialize_struct("ModLinks", ATTRS.len() + if empty { 0 } else { 1 })?;

        if empty {
            ser.skip_field("Manifest")?;
        } else {
            ser.serialize_field("Manifest", &self.mods)?;
        }

        for (key, value) in ATTRS {
            ser.serialize_field(key, value)?;
        }

        ser.end()
    }
}
