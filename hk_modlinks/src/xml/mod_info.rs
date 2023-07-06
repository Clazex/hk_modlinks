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
    dependencies: Option<Dependencies>,
    repository: Option<String>,
    issues: Option<String>,
    integrations: Option<Integrations>,
    tags: Option<Tags>,
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
        self.name.partial_cmp(&other.name)
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
            dependencies: match info.dependencies.len() {
                0 => None,
                _ => Some(Dependencies::new(info.dependencies)),
            },
            repository: info.repository,
            issues: info.issues,
            integrations: match info.integrations.len() {
                0 => None,
                _ => Some(Integrations::new(info.integrations)),
            },
            tags: match info.tags.len() {
                0 => None,
                _ => Some(Tags::new(info.tags)),
            },
            authors: match info.authors.len() {
                0 => None,
                _ => Some(Authors::new(info.authors)),
            },
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
                dependencies: value
                    .dependencies
                    .map_or_else(Default::default, |v| v.into_inner()),
                repository: value.repository,
                issues: value.issues,
                integrations: value
                    .integrations
                    .map_or_else(Default::default, |v| v.into_inner()),
                tags: value.tags.map_or_else(Default::default, |v| v.into_inner()),
                authors: value
                    .authors
                    .map_or_else(Default::default, |v| v.into_inner()),
            },
        )
    }
}
