use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Authors {
    #[serde(default, rename = "Author")]
    inner: BTreeSet<String>,
}

impl Authors {
    pub fn new(authors: BTreeSet<String>) -> Self {
        Self { inner: authors }
    }

    pub fn inner(self) -> BTreeSet<String> {
        self.inner
    }
}
