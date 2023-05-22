use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tags {
    #[serde(default, rename = "Tag")]
    inner: BTreeSet<(Tag,)>,
}

impl Tags {
    pub fn new(tags: BTreeSet<Tag>) -> Self {
        Self {
            inner: tags.into_iter().map(|i| (i,)).collect(),
        }
    }

    pub fn inner(self) -> BTreeSet<Tag> {
        self.inner.into_iter().map(|i| i.0).collect()
    }
}
