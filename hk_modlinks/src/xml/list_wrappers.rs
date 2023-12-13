use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

macro_rules! list_wrapper {
    ($type:ident, $inner:literal) => {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $type {
            #[serde(default, rename = $inner)]
            inner: BTreeSet<String>,
        }

        impl $type {
            pub fn new(inner: BTreeSet<String>) -> Self {
                Self { inner }
            }

            pub fn into_inner(self) -> BTreeSet<String> {
                self.inner
            }
        }
    };
    ($type:ident, $inner:literal, optional) => {
        list_wrapper!($type, $inner);

        impl $type {
            pub fn wrap(list: BTreeSet<String>) -> Option<Self> {
                match list.len() {
                    0 => None,
                    _ => Some(Self::new(list)),
                }
            }

            pub fn unwrap(list: Option<Self>) -> BTreeSet<String> {
                list.map_or_else(Default::default, |x| x.into_inner())
            }
        }
    };
}

list_wrapper!(Authors, "Author", optional);
list_wrapper!(Dependencies, "Dependency", optional);
list_wrapper!(Integrations, "Integration", optional);
list_wrapper!(Tags, "Tag", optional);
list_wrapper!(FileList, "File");
