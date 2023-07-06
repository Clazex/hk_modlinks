use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tags {
    #[serde(default, rename = "Tag")]
    inner: BTreeSet<String>,
}

impl Tags {
    pub fn new(tags: BTreeSet<String>) -> Self {
        Self { inner: tags }
    }

    pub fn into_inner(self) -> BTreeSet<String> {
        self.inner
    }
}
