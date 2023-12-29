use std::borrow::Cow;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::Tag;

macro_rules! list_wrapper {
    ($name:ident, $type:ty, $item:literal) => {
        #[derive(Debug, Clone, Default, Deserialize, Serialize)]
        pub struct $name<'a> {
            #[serde(default, rename = $item)]
            value: Cow<'a, BTreeSet<$type>>,
        }

        impl<'a> From<$name<'a>> for BTreeSet<$type> {
            #[inline]
            fn from(value: $name<'a>) -> Self {
                value.value.into_owned()
            }
        }

        impl<'a> From<&'a BTreeSet<$type>> for $name<'a> {
            #[inline]
            fn from(value: &'a BTreeSet<$type>) -> Self {
                Self {
                    value: Cow::Borrowed(value),
                }
            }
        }

        impl<'a> $name<'a> {
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.value.is_empty()
            }
        }
    };
}

list_wrapper!(Authors, String, "Author");
list_wrapper!(Dependencies, String, "Dependency");
list_wrapper!(Integrations, String, "Integration");
list_wrapper!(Tags, Tag, "Tag");
list_wrapper!(FileList, String, "File");
