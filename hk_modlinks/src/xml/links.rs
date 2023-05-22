use serde::{Deserialize, Serialize};

use super::FileDef;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Links {
    #[serde(rename = "Link")]
    Universal(FileDef),
    #[serde(rename = "Links")]
    PlatformDependent {
        #[serde(rename = "Windows")]
        windows: FileDef,
        #[serde(rename = "Mac")]
        mac: FileDef,
        #[serde(rename = "Linux")]
        linux: FileDef,
    },
}

impl From<crate::Links> for Links {
    fn from(value: crate::Links) -> Self {
        match value {
            crate::Links::Universal(file_def) => Self::Universal(file_def.into()),
            crate::Links::PlatformDependent {
                windows,
                mac,
                linux,
            } => Self::PlatformDependent {
                windows: windows.into(),
                mac: mac.into(),
                linux: linux.into(),
            },
        }
    }
}

impl From<Links> for crate::Links {
    fn from(value: Links) -> Self {
        match value {
            Links::Universal(file_def) => Self::Universal(file_def.into()),
            Links::PlatformDependent {
                windows,
                mac,
                linux,
            } => Self::PlatformDependent {
                windows: windows.into(),
                mac: mac.into(),
                linux: linux.into(),
            },
        }
    }
}
