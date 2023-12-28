use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

use thiserror::Error;

type N = u16;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr, SerializeDisplay,
)]
pub struct Version(N, N, N, N);

impl Display for Version {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("{}.{}.{}.{}", self.0, self.1, self.2, self.3))
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<_> = s.split('.').collect();

        let [s1, s2, s3, s4] = segments.as_slice() else {
            return Err(Self::Err::BadSegmentLength(segments.len()));
        };

        Ok(Self(s1.parse()?, s2.parse()?, s3.parse()?, s4.parse()?))
    }
}

impl Version {
    #[inline]
    #[must_use]
    pub fn new(n1: N, n2: N, n3: N, n4: N) -> Self {
        Self(n1, n2, n3, n4)
    }

    #[inline]
    pub fn new_from_str(str: impl AsRef<str>) -> Result<Self, ParseVersionError> {
        Self::from_str(str.as_ref())
    }
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ParseVersionError {
    #[error("Expect version to have 4 segments, got {0} segments")]
    BadSegmentLength(usize),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}
