use crate::FormatError;
use super::Formatter;
use serde::Serialize;

pub struct JsonFormatter {
    pub pretty: bool,
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self { pretty: false }
    }

    pub fn pretty() -> Self {
        Self { pretty: true }
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for JsonFormatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError> {
        if self.pretty {
            serde_json::to_string_pretty(data)
        } else {
            serde_json::to_string(data)
        }
        .map_err(|e| FormatError::SerializationError(e.to_string()))
    }
}
