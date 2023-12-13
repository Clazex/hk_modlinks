use serde::{Deserialize, Serialize};

use crate::FileDef;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Links {
    Universal(FileDef),
    PlatformDependent {
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
        Self::new_platform_dependent(value.0, value.1, value.2)
    }
}

impl Links {
    pub fn new_universal(file_def: FileDef) -> Self {
        Self::Universal(file_def)
    }

    pub fn new_platform_dependent(windows: FileDef, mac: FileDef, linux: FileDef) -> Self {
        Self::PlatformDependent {
            windows: Box::new(windows),
            mac: Box::new(mac),
            linux: Box::new(linux),
        }
    }
}
