use std::borrow::Cow;

use const_format::concatcp;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use super::{FileList, Links, APILINKS_SCHEMA_URL, NAMESPACE, XSD, XSI};

const ATTRS: &[(&str, &str)] = &[
    ("@xmlns", NAMESPACE),
    ("@xmlns:xsd", XSD),
    ("@xmlns:xsi", XSI),
    (
        "@xsi:schemaLocation",
        concatcp!(NAMESPACE, ' ', APILINKS_SCHEMA_URL),
    ),
];

#[derive(Debug, Clone, Deserialize)]
pub struct ApiLinks<'a> {
    #[serde(rename = "Manifest")]
    manifest: Manifest<'a>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Manifest<'a> {
    version: Cow<'a, String>,
    #[serde(flatten)]
    links: Links<'a>,
    files: FileList<'a>,
}

impl<'a> From<ApiLinks<'a>> for crate::ApiLinks {
    fn from(value: ApiLinks<'a>) -> Self {
        let value = value.manifest;
        Self {
            version: value.version.into_owned(),
            links: value.links.into(),
            files: value.files.into(),
        }
    }
}

impl<'a> From<&'a crate::ApiLinks> for ApiLinks<'a> {
    #[inline]
    fn from(value: &'a crate::ApiLinks) -> Self {
        Self {
            manifest: Manifest {
                version: Cow::Borrowed(&value.version),
                links: (&value.links).into(),
                files: (&value.files).into(),
            },
        }
    }
}

impl<'a> Serialize for ApiLinks<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut ser = serializer.serialize_struct("ApiLinks", ATTRS.len() + 1)?;

        ser.serialize_field("Manifest", &self.manifest)?;

        for (key, value) in ATTRS {
            ser.serialize_field(key, value)?;
        }

        ser.end()
    }
}
