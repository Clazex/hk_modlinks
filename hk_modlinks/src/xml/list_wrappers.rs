use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::Tag;

macro_rules! list_wrapper {
    ($type:ident, $item:ty, $inner:literal) => {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $type {
            #[serde(default, rename = $inner)]
            inner: BTreeSet<$item>,
        }

        impl $type {
            #[inline]
            #[must_use]
            pub fn new(inner: BTreeSet<$item>) -> Self {
                Self { inner }
            }

            #[inline]
            #[must_use]
            pub fn into_inner(self) -> BTreeSet<$item> {
                self.inner
            }
        }
    };
    ($type:ident, $item:ty, $inner:literal, optional) => {
        list_wrapper!($type, $item, $inner);

        impl $type {
            #[must_use]
            pub fn wrap(list: BTreeSet<$item>) -> Option<Self> {
                match list.len() {
                    0 => None,
                    _ => Some(Self::new(list)),
                }
            }

            #[must_use]
            pub fn unwrap(list: Option<Self>) -> BTreeSet<$item> {
                list.map_or_else(Default::default, |x| x.into_inner())
            }
        }
    };
}

list_wrapper!(Authors, String, "Author", optional);
list_wrapper!(Dependencies, String, "Dependency");
list_wrapper!(Integrations, String, "Integration", optional);
list_wrapper!(Tags, Tag, "Tag", optional);
list_wrapper!(FileList, String, "File");
