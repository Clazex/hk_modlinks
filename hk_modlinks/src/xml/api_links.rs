use serde::{Deserialize, Serialize};

use super::{FileList, Links};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiLinks {
    version: String,
    #[serde(flatten)]
    links: Links,
    files: FileList,
}

impl From<crate::ApiLinks> for ApiLinks {
    fn from(value: crate::ApiLinks) -> Self {
        Self {
            version: value.version,
            links: value.links.into(),
            files: FileList::new(value.files),
        }
    }
}

impl From<ApiLinks> for crate::ApiLinks {
    fn from(value: ApiLinks) -> Self {
        Self {
            version: value.version,
            links: value.links.into(),
            files: value.files.into_inner(),
        }
    }
}

impl ApiLinks {
    pub fn into_general(self) -> crate::ApiLinks {
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
