use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_with::DeserializeFromStr;

use thiserror::Error;

type N = u32;
type VersionTuple = (N, N, N, N);

#[derive(Debug, Clone, DeserializeFromStr)]
pub struct Version(VersionTuple, String);

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ParseVersionError {
    #[error("Expect version to have 4 segments, got {0} segments")]
    BadSegmentLength(usize),
    #[error("Failed when parsing segments: {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl PartialEq for Version {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Self::parse(s)?, s.to_string()))
    }
}

impl TryFrom<String> for Version {
    type Error = ParseVersionError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Self::parse(&value)?, value))
    }
}

impl From<Version> for String {
    #[inline]
    fn from(value: Version) -> Self {
        value.into_string()
    }
}

impl From<VersionTuple> for Version {
    #[inline]
    fn from(value: VersionTuple) -> Self {
        Self::new_from_tuple(value)
    }
}

impl Display for Version {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.str().fmt(f)
    }
}

impl Serialize for Version {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.1.serialize(serializer)
    }
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Self::new_from_tuple((0, 0, 0, 0))
    }
}

impl Version {
    #[must_use]
    pub fn new(n1: N, n2: N, n3: N, n4: N) -> Self {
        Self((n1, n2, n3, n4), format!("{n1}.{n2}.{n3}.{n4}"))
    }

    #[inline]
    #[must_use]
    pub fn new_from_tuple(tuple: VersionTuple) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }

    #[inline]
    pub fn new_from_str(str: impl AsRef<str>) -> Result<Self, ParseVersionError> {
        Self::from_str(str.as_ref())
    }

    #[inline]
    pub fn new_from_string(str: String) -> Result<Self, ParseVersionError> {
        Self::try_from(str)
    }

    #[inline]
    #[must_use]
    pub fn tuple(&self) -> VersionTuple {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn str(&self) -> &String {
        &self.1
    }

    #[inline]
    #[must_use]
    pub fn into_string(self) -> String {
        self.1
    }

    fn parse(str: &str) -> Result<VersionTuple, ParseVersionError> {
        let segments: Vec<_> = str.split('.').collect();

        let [s1, s2, s3, s4] = segments.as_slice() else {
            return Err(ParseVersionError::BadSegmentLength(segments.len()));
        };

        Ok((
            N::from_str(s1)?,
            N::from_str(s2)?,
            N::from_str(s3)?,
            N::from_str(s4)?,
        ))
    }
}
