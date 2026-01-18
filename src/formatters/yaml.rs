use crate::FormatError;
use super::Formatter;
use serde::Serialize;

pub struct YamlFormatter;

impl YamlFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YamlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for YamlFormatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError> {
        serde_yaml::to_string(data)
            .map_err(|e| FormatError::SerializationError(e.to_string()))
    }
}
