use std::fmt::{self, Debug, Formatter};

use serde::{Deserialize, Serialize};
use serde_with::{formats::Uppercase, hex::Hex, serde_as};

use url::Url;

#[serde_as]
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FileDef {
    #[serde_as(as = "Hex<Uppercase>")]
    pub sha256: [u8; 32],
    pub url: Url,
}

impl Debug for FileDef {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("FileDef")
            .field("sha256", &self.sha256())
            .field("url", &self.url)
            .finish()
    }
}

impl FileDef {
    #[must_use]
    pub fn new(sha256: [u8; 32], url: Url) -> Self {
        Self { sha256, url }
    }

    pub fn new_from_hex(sha256: impl AsRef<[u8]>, url: Url) -> Result<Self, hex::FromHexError> {
        let mut fd = Self::new([0; 32], url);
        hex::decode_to_slice(sha256, &mut fd.sha256)?;
        Ok(fd)
    }

    #[inline]
    #[must_use]
    pub fn sha256(&self) -> String {
        hex::encode_upper(self.sha256)
    }
}
