use crate::FormatError;
use super::Formatter;
use serde::Serialize;

pub struct TomlFormatter;

impl TomlFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TomlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for TomlFormatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError> {
        toml::to_string_pretty(data)
            .map_err(|e| FormatError::SerializationError(e.to_string()))
    }
}
