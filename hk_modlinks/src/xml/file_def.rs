use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct FileDef {
    #[serde(
        rename = "@SHA256",
        deserialize_with = "hex::deserialize",
        serialize_with = "hex::serialize_upper"
    )]
    sha256: [u8; 32],
    #[serde(rename = "$text")]
    url: String,
}

impl From<crate::FileDef> for FileDef {
    fn from(value: crate::FileDef) -> Self {
        Self {
            sha256: value.sha256,
            url: value.url,
        }
    }
}

impl From<FileDef> for crate::FileDef {
    fn from(value: FileDef) -> Self {
        Self {
            sha256: value.sha256,
            url: value.url,
        }
    }
}

impl Debug for FileDef {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("FileDef")
            .field("sha256", &hex::encode_upper(self.sha256))
            .field("url", &self.url)
            .finish()
    }
}
