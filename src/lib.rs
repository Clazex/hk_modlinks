use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Links {
    Universal(String),
    PlatformDependent {
        windows: String,
        mac: String,
        linux: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModInfo {
    #[serde(flatten)]
    links: Links,
}

impl ModInfo {
	pub fn to_xml(&self) -> Result<String, quick_xml::DeError> {
        quick_xml::se::to_string(&self)
    }
}
