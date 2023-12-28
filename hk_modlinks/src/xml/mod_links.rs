use serde::{Deserialize, Serialize, Serializer};

use const_format::concatcp;

use super::ModInfo;

const XSD: &str = "http://www.w3.org/2001/XMLSchema";
const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";
const NS: &str = "https://github.com/HollowKnight-Modding\
	/HollowKnight.ModLinks/HollowKnight.ModManager";
const SCHEMA_URL: &str = "https://raw.githubusercontent.com/\
	HollowKnight-Modding/HollowKnight.ModLinks/main/Schemas/ModLinks.xml";
const SCHEMA_LOCATION: &str = concatcp!(NS, ' ', SCHEMA_URL);

#[derive(Debug, Clone, Deserialize)]
pub struct ModLinks {
    #[serde(rename = "Manifest")]
    mods: Vec<ModInfo>,
}

#[must_use]
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename = "ModLinks")]
struct ModLinksWithExtraAttrs<'a> {
    #[serde(rename = "Manifest", skip_serializing_if = "Vec::is_empty")]
    mods: &'a Vec<ModInfo>,
    #[serde(rename = "@xmlns")]
    xml_ns: &'static str,
    #[serde(rename = "@xmlns:xsd")]
    xml_ns_xsd: &'static str,
    #[serde(rename = "@xmlns:xsi")]
    xml_ns_xsi: &'static str,
    #[serde(rename = "@xsi:schemaLocation")]
    xsi_schema_location: &'static str,
}

impl From<crate::ModLinks> for ModLinks {
    fn from(value: crate::ModLinks) -> Self {
        Self {
            mods: value.into_inner().into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ModLinks> for crate::ModLinks {
    fn from(value: ModLinks) -> Self {
        Self::new_from_map(value.mods.into_iter().map(Into::into).collect())
    }
}

impl Serialize for ModLinks {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ModLinksWithExtraAttrs::from(self).serialize(serializer)
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

impl<'a> From<&'a ModLinks> for ModLinksWithExtraAttrs<'a> {
    #[inline]
    fn from(value: &'a ModLinks) -> Self {
        Self {
            mods: &value.mods,
            xml_ns: NS,
            xml_ns_xsd: XSD,
            xml_ns_xsi: XSI,
            xsi_schema_location: SCHEMA_LOCATION,
        }
    }
}
