use std::borrow::Cow;
use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use url::Url;

#[derive(Clone, Deserialize, Serialize)]
pub struct FileDef<'a> {
    #[serde(rename = "@SHA256", with = "hex_cow")]
    sha256: Cow<'a, [u8; 32]>,
    #[serde(rename = "$text")]
    url: Cow<'a, Url>,
}

impl<'a> From<FileDef<'a>> for crate::FileDef {
    fn from(value: FileDef) -> Self {
        Self {
            sha256: value.sha256.into_owned(),
            url: value.url.into_owned(),
        }
    }
}

impl<'a> From<&'a crate::FileDef> for FileDef<'a> {
    fn from(value: &'a crate::FileDef) -> Self {
        Self {
            sha256: Cow::Borrowed(&value.sha256),
            url: Cow::Borrowed(&value.url),
        }
    }
}

impl<'a> Debug for FileDef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("FileDef")
            .field("sha256", &hex::encode_upper(self.sha256.as_ref()))
            .field("url", &self.url)
            .finish()
    }
}

mod hex_cow {
    use super::Cow;
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'a, 'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Cow<'a, [u8; 32]>, D::Error> {
        hex::deserialize(deserializer).map(Cow::Owned)
    }

    pub fn serialize<S: Serializer>(value: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error> {
        hex::serialize_upper(value, serializer)
    }
}
