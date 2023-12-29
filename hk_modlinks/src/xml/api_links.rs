use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{FileList, Links};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiLinks<'a> {
    version: Cow<'a, String>,
    #[serde(flatten)]
    links: Links<'a>,
    files: FileList<'a>,
}

impl<'a> From<ApiLinks<'a>> for crate::ApiLinks {
    fn from(value: ApiLinks<'a>) -> Self {
        Self {
            version: value.version.into_owned(),
            links: value.links.into(),
            files: value.files.into(),
        }
    }
}

impl<'a> From<&'a crate::ApiLinks> for ApiLinks<'a> {
    #[inline]
    fn from(value: &'a crate::ApiLinks) -> Self {
        Self {
            version: Cow::Borrowed(&value.version),
            links: (&value.links).into(),
            files: (&value.files).into(),
        }
    }
}
