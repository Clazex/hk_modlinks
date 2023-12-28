use std::collections::BTreeMap;

use serde::Serialize;

use serde_json::{json, Value as JsonValue};

use crate::ModLinks;

pub struct ModLinksChangelog {
    pub(crate) ctx: JsonValue,
}

impl ModLinksChangelog {
    #[must_use]
    pub(crate) fn new(old: &ModLinks, new: &ModLinks) -> Self {
        Self {
            ctx: json!({
                "new": empty_map_to_null(Self::new_mods(old, new)),
                "removed": Self::removed_mods(old, new),
                "updated": empty_map_to_null(Self::updated_mods(old, new))
            }),
        }
    }

    #[inline]
    #[must_use]
    pub fn json(&self) -> &JsonValue {
        &self.ctx
    }
}

impl From<ModLinksChangelog> for JsonValue {
    #[inline]
    fn from(value: ModLinksChangelog) -> Self {
        value.ctx
    }
}

impl Serialize for ModLinksChangelog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.json().serialize(serializer)
    }
}

fn to_json_value<T: Serialize>(value: T) -> JsonValue {
    serde_json::to_value(value).expect("Failed to serialize to json")
}

fn empty_map_to_null(value: JsonValue) -> JsonValue {
    match value {
        JsonValue::Object(obj) if obj.is_empty() => JsonValue::Null,
        _ => value,
    }
}

impl ModLinksChangelog {
    #[inline]
    fn new_mods(old: &ModLinks, new: &ModLinks) -> JsonValue {
        to_json_value(
            new.iter()
                .filter_map(|(name, mod_info)| {
                    if old.contains(name) {
                        None
                    } else {
                        Some((
                            name,
                            json!({
                                "description": mod_info.description,
                                "dependencies": mod_info.dependencies,
                                "integrations": mod_info.integrations,
                                "tags": mod_info.tags
                            }),
                        ))
                    }
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }

    #[inline]
    fn removed_mods(old: &ModLinks, new: &ModLinks) -> JsonValue {
        to_json_value(
            old.iter()
                .filter_map(
                    |(name, _)| {
                        if new.contains(name) {
                            None
                        } else {
                            Some(name)
                        }
                    },
                )
                .collect::<Vec<_>>(),
        )
    }

    #[inline]
    fn gen_old_new(old: &str, new: &str) -> JsonValue {
        if old == new {
            JsonValue::Null
        } else {
            json!({
                "old": old,
                "new": new
            })
        }
    }

    #[inline]
    fn gen_removed_added<T: Serialize>(removed: Vec<T>, added: Vec<T>) -> JsonValue {
        let removed_empty = removed.is_empty();
        let added_empty = added.is_empty();

        if removed_empty && added_empty {
            return JsonValue::Null;
        }

        if removed_empty {
            json!({ "added": added })
        } else if added_empty {
            json!({ "removed": removed })
        } else {
            json!({
                "removed": removed,
                "added": added
            })
        }
    }

    #[inline]
    fn updated_mods(old: &ModLinks, new: &ModLinks) -> JsonValue {
        to_json_value(
            new
                .iter()
                .filter_map(|(name, new)| {
					let Some(old) = old.get(name) else {
						return None;
					};

					let ver = Self::gen_old_new(old.version.str(), new.version.str());
					if ver.is_null() {
						return None;
					}

                    Some((
						name,
						json!({
							"version": ver,
							"description": Self::gen_old_new(old.description.as_str(), new.description.as_str()),
							"dependencies": Self::gen_removed_added(
								old.dependencies.difference(&new.dependencies).collect::<Vec<_>>(),
								new.dependencies.difference(&old.dependencies).collect::<Vec<_>>()
							),
							"integrations": Self::gen_removed_added(
								old.integrations.difference(&new.integrations).collect::<Vec<_>>(),
								new.integrations.difference(&old.integrations).collect::<Vec<_>>()
							),
							"tags": Self::gen_removed_added(
								new.tags.difference(&old.tags).collect::<Vec<_>>(),
								old.tags.difference(&new.tags).collect::<Vec<_>>()
							)
						}),
					))
                })
                .collect::<BTreeMap<_, _>>(),
        )
    }
}
