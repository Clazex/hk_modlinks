use std::fmt::{self, Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "clap-enum", derive(clap::ValueEnum))]
pub enum Platform {
    Windows,
    Mac,
    Linux,
}

impl Display for Platform {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, fmt)
    }
}

#[cfg(any(target_os = "windows", target_os = "mac", target_os = "linux"))]
impl Default for Platform {
    #[inline]
    fn default() -> Self {
        Self::LOCAL
    }
}

impl Platform {
    #[cfg(target_os = "windows")]
    pub const LOCAL: Self = Self::Windows;

    #[cfg(target_os = "macos")]
    pub const LOCAL: Self = Self::Mac;

    #[cfg(target_os = "linux")]
    pub const LOCAL: Self = Self::Linux;
}
