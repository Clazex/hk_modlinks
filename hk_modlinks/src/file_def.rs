use std::fmt::{self, Debug, Formatter};

use serde::{Deserialize, Serialize};

use url::Url;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FileDef {
    #[serde(
        deserialize_with = "hex::deserialize",
        serialize_with = "hex::serialize_upper"
    )]
    pub sha256: [u8; 32],
    pub url: Url,
}

impl Debug for FileDef {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("FileDef")
            .field("sha256", &hex::encode_upper(self.sha256))
            .field("url", &self.url)
            .finish()
    }
}

impl FileDef {
    pub fn new(sha256: [u8; 32], url: Url) -> Self {
        Self { sha256, url }
    }

    pub fn new_from_hex<T: AsRef<[u8]>>(sha256: T, url: Url) -> Result<Self, hex::FromHexError> {
        let mut fd = Self::new([0; 32], url);
        hex::decode_to_slice(sha256, &mut fd.sha256)?;
        Ok(fd)
    }
}
