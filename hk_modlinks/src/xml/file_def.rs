use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};
use serde_with::{formats::Uppercase, hex::Hex, serde_as};

use url::Url;

#[serde_as]
#[derive(Clone, Deserialize, Serialize)]
pub struct FileDef {
    #[serde(rename = "@SHA256")]
    #[serde_as(as = "Hex<Uppercase>")]
    sha256: [u8; 32],
    #[serde(rename = "$text")]
    url: Url,
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
