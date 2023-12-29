mod api_links;
mod file_def;
mod links;
mod list_wrappers;
mod mod_info;
mod mod_links;

use const_format::concatcp;

use serde::Serialize;

use file_def::*;
use links::*;
use list_wrappers::*;
use mod_info::*;

pub use api_links::ApiLinks;
pub use mod_links::ModLinks;

const XSD: &str = "http://www.w3.org/2001/XMLSchema";
const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";

const NAMESPACE: &str = "https://github.com/HollowKnight-Modding\
	/HollowKnight.ModLinks/HollowKnight.ModManager";
const SCHEMA_URL_BASE: &str = "https://raw.githubusercontent.com/\
	HollowKnight-Modding/HollowKnight.ModLinks/main/Schemas/";
const APILINKS_SCHEMA_URL: &str = concatcp!(SCHEMA_URL_BASE, "ApiLinks.xml");
const MODLINKS_SCHEMA_URL: &str = concatcp!(SCHEMA_URL_BASE, "ModLinks.xml");

macro_rules! impl_xml_convert {
    ($type:ident) => {
        impl<'a> $type<'a> {
            #[inline]
            pub fn to_xml(&self) -> Result<String, quick_xml::DeError> {
                let mut string = String::new();
                self.to_xml_writer(&mut string)?;
                Ok(string)
            }

            #[inline]
            pub fn to_xml_writer<W: std::fmt::Write>(
                &self,
                writer: &mut W,
            ) -> Result<(), quick_xml::DeError> {
                let mut serializer = quick_xml::se::Serializer::new(writer);
                serializer.indent('\t', 1);
                self.serialize(serializer)
            }

            #[inline]
            pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
                quick_xml::de::from_str(s)
            }

            #[inline]
            pub fn from_xml_reader<R: std::io::BufRead>(
                reader: R,
            ) -> Result<Self, quick_xml::DeError> {
                quick_xml::de::from_reader(reader)
            }
        }
    };
}

impl_xml_convert!(ApiLinks);
impl_xml_convert!(ModLinks);
