use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileList {
    #[serde(default, rename = "File")]
    inner: BTreeSet<String>,
}

impl FileList {
    pub fn new(inner: BTreeSet<String>) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> BTreeSet<String> {
        self.inner
    }
}
