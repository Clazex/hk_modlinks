use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_with::DeserializeFromStr;

use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Tag {
    Boss,
    Cosmetic,
    Expansion,
    Gameplay,
    Library,
    Utility,
}

#[derive(Debug, Clone, Error)]
pub enum ParseTagError {
    #[error("Unknown variant: {0}")]
    UnknownVariant(String),
}

impl FromStr for Tag {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Tag::*;

        Ok(match s {
            "Boss" => Boss,
            "Cosmetic" => Cosmetic,
            "Expansion" => Expansion,
            "Gameplay" => Gameplay,
            "Library" => Library,
            "Utility" => Utility,
            _ => Err(ParseTagError::UnknownVariant(s.to_string()))?,
        })
    }
}

impl Display for Tag {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl From<Tag> for &'static str {
    #[inline]
    fn from(value: Tag) -> Self {
        value.as_str()
    }
}

impl Serialize for Tag {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl Tag {
    pub fn as_str(&self) -> &'static str {
        use Tag::*;

        match self {
            Boss => "Boss",
            Cosmetic => "Cosmetic",
            Expansion => "Expansion",
            Gameplay => "Gameplay",
            Library => "Library",
            Utility => "Utility",
        }
    }
}
