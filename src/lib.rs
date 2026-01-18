use serde::Serialize;
use std::fmt;

pub mod formatters;

pub use formatters::{
    JsonFormatter,
    YamlFormatter,
    TomlFormatter,
    XmlFormatter,
    ToonFormatter,
    Formatter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    JsonPretty,
    Yaml,
    Toml,
    Xml,
    Toon,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "json-pretty" | "jsonpretty" => Some(Self::JsonPretty),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            "xml" => Some(Self::Xml),
            "toon" => Some(Self::Toon),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum FormatError {
    SerializationError(String),
    UnsupportedFormat(String),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
        }
    }
}

impl std::error::Error for FormatError {}

pub fn format_data<T: Serialize>(
    data: &T,
    format: OutputFormat,
) -> Result<String, FormatError> {
    use formatters::Formatter;
    
    match format {
        OutputFormat::Json => JsonFormatter::new().format(data),
        OutputFormat::JsonPretty => JsonFormatter::pretty().format(data),
        OutputFormat::Yaml => YamlFormatter::new().format(data),
        OutputFormat::Toml => TomlFormatter::new().format(data),
        OutputFormat::Xml => XmlFormatter::new().format(data),
        OutputFormat::Toon => ToonFormatter::new().format(data),
    }
}

pub fn to_value<T: Serialize>(data: &T) -> Result<serde_json::Value, FormatError> {
    serde_json::to_value(data)
        .map_err(|e| FormatError::SerializationError(e.to_string()))
}

