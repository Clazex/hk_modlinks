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
