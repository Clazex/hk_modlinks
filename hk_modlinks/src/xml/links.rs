use serde::{Deserialize, Serialize};

use super::FileDef;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Links {
    #[serde(rename = "Link")]
    Universal(FileDef),
    #[serde(rename = "Links")]
    PlatformSpecific {
        #[serde(rename = "Windows")]
        windows: Box<FileDef>,
        #[serde(rename = "Mac")]
        mac: Box<FileDef>,
        #[serde(rename = "Linux")]
        linux: Box<FileDef>,
    },
}

impl From<crate::Links> for Links {
    fn from(value: crate::Links) -> Self {
        match value {
            crate::Links::Universal(file_def) => Self::Universal(file_def.into()),
            crate::Links::PlatformSpecific {
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

impl From<Links> for crate::Links {
    fn from(value: Links) -> Self {
        match value {
            Links::Universal(file_def) => Self::Universal(file_def.into()),
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
