use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::Links;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiLinks {
    pub version: String,
    pub links: Links,
    pub files: BTreeSet<String>,
}

#[cfg(feature = "xml")]
impl ApiLinks {
    #[inline]
    #[must_use]
    pub fn into_xml_compat(self) -> crate::ApiLinksXmlCompat {
        self.into()
    }

    pub fn into_xml(self) -> Result<String, quick_xml::DeError> {
        self.into_xml_compat().to_xml()
    }

    pub fn into_xml_writer<W: std::fmt::Write>(self, writer: W) -> Result<(), quick_xml::DeError> {
        self.into_xml_compat().to_xml_writer(writer)
    }

    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        crate::ApiLinksXmlCompat::from_xml(s).map(Into::into)
    }

    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        crate::ApiLinksXmlCompat::from_xml_reader(reader).map(Into::into)
    }
}
