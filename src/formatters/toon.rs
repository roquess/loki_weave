use crate::FormatError;
use super::Formatter;
use serde::Serialize;

pub struct ToonFormatter {
    pub prefix: String,
}

impl ToonFormatter {
    pub fn new() -> Self {
        Self {
            prefix: "[toon]".to_string(),
        }
    }

    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl Default for ToonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for ToonFormatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError> {
        let json_value = serde_json::to_value(data)
            .map_err(|e| FormatError::SerializationError(e.to_string()))?;
        
        Ok(format_toon_value(&json_value, &self.prefix, 0))
    }
}

fn format_toon_value(value: &serde_json::Value, prefix: &str, indent: usize) -> String {
    use serde_json::Value;
    
    let indent_str = "  ".repeat(indent);
    
    match value {
        Value::Object(map) => {
            let mut lines = Vec::new();
            let mut main_fields = Vec::new();
            
            for (k, v) in map.iter() {
                match v {
                    Value::String(_) | Value::Number(_) | Value::Bool(_) => {
                        main_fields.push(format!("{}={}", k, format_simple_value(v)));
                    }
                    _ => {
                        lines.push(format!("{}  {}={}", indent_str, k, format_simple_value(v)));
                    }
                }
            }
            
            if !main_fields.is_empty() {
                lines.insert(0, format!("{}{} {}", indent_str, prefix, main_fields.join(" ")));
            }
            
            lines.join("\n")
        }
        Value::Array(arr) => {
            arr.iter()
                .map(|v| format_toon_value(v, prefix, indent))
                .collect::<Vec<_>>()
                .join("\n")
        }
        _ => format!("{}{} {}", indent_str, prefix, format_simple_value(value)),
    }
}

fn format_simple_value(value: &serde_json::Value) -> String {
    use serde_json::Value;
    
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            format!("[{}]", arr.iter()
                .map(format_simple_value)
                .collect::<Vec<_>>()
                .join(", "))
        }
        Value::Object(map) => {
            format!("{{{}}}", map.iter()
                .map(|(k, v)| format!("{}={}", k, format_simple_value(v)))
                .collect::<Vec<_>>()
                .join(", "))
        }
    }
}

