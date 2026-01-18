use crate::FormatError;
use serde::Serialize;

pub mod json;
pub mod yaml;
pub mod toml;
pub mod xml;
pub mod toon;

pub use json::JsonFormatter;
pub use yaml::YamlFormatter;
pub use toml::TomlFormatter;
pub use xml::XmlFormatter;
pub use toon::ToonFormatter;

pub trait Formatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError>;
}

