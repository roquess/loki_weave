use crate::FormatError;
use super::Formatter;
use serde::Serialize;

pub struct XmlFormatter {
    pub root_tag: String,
}

impl XmlFormatter {
    pub fn new() -> Self {
        Self {
            root_tag: "data".to_string(),
        }
    }

    pub fn with_root_tag(tag: impl Into<String>) -> Self {
        Self {
            root_tag: tag.into(),
        }
    }
}

impl Default for XmlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for XmlFormatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError> {
        // Convertir d'abord en JSON pour simplifier la structure
        let json_value = serde_json::to_value(data)
            .map_err(|e| FormatError::SerializationError(e.to_string()))?;
        
        // Construire le XML manuellement pour plus de contrôle
        let xml = format_xml_value(&json_value, &self.root_tag);
        
        Ok(xml)
    }
}

fn format_xml_value(value: &serde_json::Value, tag: &str) -> String {
    use serde_json::Value;
    
    match value {
        Value::Object(map) => {
            let mut inner = String::new();
            for (k, v) in map.iter() {
                inner.push_str(&format_xml_value(v, k));
            }
            format!("<{}>{}</{}>", tag, inner, tag)
        }
        Value::Array(arr) => {
            let items: String = arr.iter()
                .map(|v| format_xml_value(v, "item"))
                .collect();
            format!("<{}>{}</{}>", tag, items, tag)
        }
        Value::String(s) => {
            format!("<{}>{}</{}>", tag, xml_escape(s), tag)
        }
        Value::Number(n) => {
            format!("<{}>{}</{}>", tag, n, tag)
        }
        Value::Bool(b) => {
            format!("<{}>{}</{}>", tag, b, tag)
        }
        Value::Null => {
            format!("<{}/>", tag)
        }
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
