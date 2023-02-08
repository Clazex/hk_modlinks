use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::Links;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiLinks {
    pub version: String,
    pub links: Links,
    pub files: BTreeSet<String>,
}

#[cfg(feature = "json")]
impl ApiLinks {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self).map_err(Into::into)
    }

    pub fn to_json_writer<W: std::io::Write>(&self, writer: W) -> serde_json::Result<()> {
        serde_json::to_writer_pretty(writer, &self)
    }

    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    pub fn from_json_reader<R: std::io::Read>(reader: R) -> serde_json::Result<Self> {
        serde_json::from_reader(reader)
    }
}

#[cfg(feature = "xml")]
impl ApiLinks {
    pub fn into_xml_compat(self) -> crate::ApiLinksXmlCompat {
        self.into()
    }

    pub fn to_xml(self) -> Result<String, quick_xml::DeError> {
        self.into_xml_compat().to_xml()
    }

    pub fn to_xml_writer<W: std::io::Write>(self, writer: W) -> Result<(), quick_xml::DeError> {
        self.into_xml_compat().to_xml_writer(writer)
    }

    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        crate::ApiLinksXmlCompat::from_xml(s).map(Into::into)
    }

    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        crate::ApiLinksXmlCompat::from_xml_reader(reader).map(Into::into)
    }
}
