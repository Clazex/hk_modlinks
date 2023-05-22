use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Integrations {
    #[serde(default, rename = "Integration")]
    inner: BTreeSet<String>,
}

impl Integrations {
    pub fn new(integrations: BTreeSet<String>) -> Self {
        Self {
            inner: integrations,
        }
    }

    pub fn inner(self) -> BTreeSet<String> {
        self.inner
    }
}
