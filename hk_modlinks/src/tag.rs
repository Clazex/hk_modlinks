use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Tag {
    Boss,
    Cosmetic,
    Expansion,
    Gameplay,
    Library,
    Utility,
}

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
