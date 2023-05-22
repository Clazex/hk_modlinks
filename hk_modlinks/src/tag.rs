use std::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
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
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl FromStr for Tag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boss" => Ok(Tag::Boss),
            "Cosmetic" => Ok(Tag::Cosmetic),
            "Expansion" => Ok(Tag::Expansion),
            "Gameplay" => Ok(Tag::Gameplay),
            "Library" => Ok(Tag::Library),
            "Utility" => Ok(Tag::Utility),
            _ => Err(()),
        }
    }
}
