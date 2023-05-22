use crate::{ApiLinks, ModInfo, ModLinks};

#[cfg(feature = "ron")]
#[inline]
fn get_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::new().new_line("\n".to_string())
}

macro_rules! impl_convert {
    ($type:ty) => {
        #[cfg(feature = "toml")]
        impl $type {
            pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
                toml::to_string_pretty(self)
            }

            pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
                toml::from_str(s)
            }
        }

        #[cfg(feature = "json")]
        impl $type {
            pub fn to_json(&self) -> serde_json::Result<String> {
                serde_json::to_string_pretty(&self)
            }

            pub fn to_json_writer<W: std::io::Write>(&self, writer: W) -> serde_json::Result<()> {
                serde_json::to_writer_pretty(writer, &self)
            }

            pub fn from_json(s: &str) -> serde_json::Result<Self> {
                serde_json::from_str(s)
            }

            pub fn from_json_reader<R: std::io::Read>(rdr: R) -> serde_json::Result<Self> {
                serde_json::from_reader(rdr)
            }
        }

        #[cfg(feature = "yaml")]
        impl $type {
            pub fn to_yaml(&self) -> serde_yaml::Result<String> {
                serde_yaml::to_string(self)
            }

            pub fn to_yaml_writer<W: std::io::Write>(&self, writer: W) -> serde_yaml::Result<()> {
                serde_yaml::to_writer(writer, self)
            }

            pub fn from_yaml(s: &str) -> serde_yaml::Result<Self> {
                serde_yaml::from_str(s)
            }

            pub fn from_yaml_reader<R: std::io::Read>(rdr: R) -> serde_yaml::Result<Self> {
                serde_yaml::from_reader(rdr)
            }
        }

        #[cfg(feature = "ron")]
        impl $type {
            pub fn to_ron(&self) -> ron::Result<String> {
                ron::ser::to_string_pretty(self, get_config())
            }

            pub fn to_ron_writer<W: std::io::Write>(&self, writer: W) -> ron::Result<()> {
                ron::ser::to_writer_pretty(writer, self, get_config())
            }

            pub fn from_ron(s: &str) -> ron::error::SpannedResult<Self> {
                ron::from_str(s)
            }

            pub fn from_ron_reader<R: std::io::Read>(rdr: R) -> ron::error::SpannedResult<Self> {
                ron::de::from_reader(rdr)
            }
        }
    };
}

impl_convert!(ApiLinks);

impl_convert!(ModInfo);

impl_convert!(ModLinks);
