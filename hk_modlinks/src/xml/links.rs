use serde::{Deserialize, Serialize};

use super::FileDef;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Links<'a> {
    #[serde(rename = "Link")]
    Universal(FileDef<'a>),
    #[serde(rename = "Links", rename_all = "PascalCase")]
    PlatformSpecific {
        windows: Box<FileDef<'a>>,
        mac: Box<FileDef<'a>>,
        linux: Box<FileDef<'a>>,
    },
}

impl<'a> From<Links<'a>> for crate::Links {
    fn from(value: Links<'a>) -> Self {
        match value {
            Links::Universal(file) => Self::Universal(file.into()),
            Links::PlatformSpecific {
                windows,
                mac,
                linux,
            } => Self::PlatformSpecific {
                windows: Box::new((*windows).into()),
                mac: Box::new((*mac).into()),
                linux: Box::new((*linux).into()),
            },
        }
    }
}

impl<'a> From<&'a crate::Links> for Links<'a> {
    fn from(value: &'a crate::Links) -> Self {
        match value {
            crate::Links::Universal(file) => Self::Universal(file.into()),
            crate::Links::PlatformSpecific {
                windows,
                mac,
                linux,
            } => Self::PlatformSpecific {
                windows: Box::new(windows.as_ref().into()),
                mac: Box::new(mac.as_ref().into()),
                linux: Box::new(linux.as_ref().into()),
            },
        }
    }
}
