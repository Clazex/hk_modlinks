use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileList {
    #[serde(default, rename = "File")]
    inner: BTreeSet<String>,
}

impl FileList {
    pub fn new(files: BTreeSet<String>) -> Self {
        Self { inner: files }
    }

    pub fn inner(self) -> BTreeSet<String> {
        self.inner
    }
}
