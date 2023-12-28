use serde::{Deserialize, Serialize};

use crate::{FileDef, Platform};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Links {
    Universal(FileDef),
    PlatformSpecific {
        windows: Box<FileDef>,
        mac: Box<FileDef>,
        linux: Box<FileDef>,
    },
}

impl From<FileDef> for Links {
    fn from(value: FileDef) -> Self {
        Self::new_universal(value)
    }
}

impl From<(FileDef, FileDef, FileDef)> for Links {
    fn from(value: (FileDef, FileDef, FileDef)) -> Self {
        Self::new_platform_specific(value.0, value.1, value.2)
    }
}

impl Links {
    #[inline]
    #[must_use]
    pub fn new_universal(file_def: FileDef) -> Self {
        Self::Universal(file_def)
    }

    #[must_use]
    pub fn new_platform_specific(windows: FileDef, mac: FileDef, linux: FileDef) -> Self {
        Self::PlatformSpecific {
            windows: Box::new(windows),
            mac: Box::new(mac),
            linux: Box::new(linux),
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "mac", target_os = "linux"))]
impl Links {
    #[must_use]
    pub fn file(&self, platform: Option<Platform>) -> &FileDef {
        match self {
            Self::Universal(file) => file,
            Self::PlatformSpecific {
                windows,
                mac,
                linux,
            } => match platform.unwrap_or(Platform::LOCAL) {
                Platform::Windows => windows,
                Platform::Mac => mac,
                Platform::Linux => linux,
            },
        }
    }

    #[must_use]
    pub fn into_file(self, platform: Option<Platform>) -> FileDef {
        match self {
            Self::Universal(file) => file,
            Self::PlatformSpecific {
                windows,
                mac,
                linux,
            } => *match platform.unwrap_or(Platform::LOCAL) {
                Platform::Windows => windows,
                Platform::Mac => mac,
                Platform::Linux => linux,
            },
        }
    }
}
