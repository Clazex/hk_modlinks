use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use super::{Authors, Dependencies, Integrations, Links, Tags};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Manifest", rename_all = "PascalCase")]
pub struct ModInfo {
    name: String,
    description: String,
    version: String,
    #[serde(flatten)]
    links: Links,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Dependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issues: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integrations: Option<Integrations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authors: Option<Authors>,
}

impl Hash for ModInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for ModInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ModInfo {}

impl PartialOrd for ModInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ModInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<(String, crate::ModInfo)> for ModInfo {
    fn from(value: (String, crate::ModInfo)) -> Self {
        let info = value.1;

        Self {
            name: value.0,
            description: info.description,
            version: info.version,
            links: info.links.into(),
            dependencies: Dependencies::wrap(info.dependencies),
            repository: info.repository,
            issues: info.issues,
            integrations: Integrations::wrap(info.integrations),
            tags: Tags::wrap(info.tags),
            authors: Authors::wrap(info.authors),
        }
    }
}

impl From<ModInfo> for (String, crate::ModInfo) {
    fn from(value: ModInfo) -> Self {
        (
            value.name,
            crate::ModInfo {
                description: value.description,
                version: value.version,
                links: value.links.into(),
                dependencies: Dependencies::unwrap(value.dependencies),
                repository: value.repository,
                issues: value.issues,
                integrations: Integrations::unwrap(value.integrations),
                tags: Tags::unwrap(value.tags),
                authors: Authors::unwrap(value.authors),
            },
        )
    }
}
