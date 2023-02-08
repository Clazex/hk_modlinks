use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependencies {
    #[serde(default, rename = "Dependency")]
    inner: BTreeSet<String>,
}

impl Dependencies {
    pub fn new(dependencies: BTreeSet<String>) -> Self {
        Self {
            inner: dependencies,
        }
    }

    pub fn inner(self) -> BTreeSet<String> {
        self.inner
    }
}
