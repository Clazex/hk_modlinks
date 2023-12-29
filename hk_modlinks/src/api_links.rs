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
    fn xml(&self) -> crate::xml::ApiLinks<'_> {
        self.into()
    }

    pub fn to_xml(self) -> Result<String, quick_xml::DeError> {
        self.xml().to_xml()
    }

    pub fn to_xml_writer<W: std::fmt::Write>(
        self,
        writer: &mut W,
    ) -> Result<(), quick_xml::DeError> {
        self.xml().to_xml_writer(writer)
    }

    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        crate::xml::ApiLinks::from_xml(s).map(Into::into)
    }

    pub fn from_xml_reader<R: std::io::BufRead>(reader: R) -> Result<Self, quick_xml::DeError> {
        crate::xml::ApiLinks::from_xml_reader(reader).map(Into::into)
    }
}
