# waeve

**waeve** is a Rust library for parsing, normalizing, and formatting structured data across multiple formats.

It provides a unified abstraction to render data into common formats such as JSON, YAML, TOML, XML, and others, while keeping a consistent and extensible API.

## Features

- Support for multiple output formats
  - JSON
  - YAML
  - TOML
  - XML
  - Toon
- Unified `Formatter` trait
- Built on top of `serde::Serialize`
- Designed to be extensible and format-agnostic
- Clean and predictable formatting output

## Architecture

Each format is implemented as a dedicated formatter exposing a common interface:

```rust
pub trait Formatter {
    fn format<T: Serialize>(&self, data: &T) -> Result<String, FormatError>;
}
```

This allows seamless switching between formats while keeping the same data model.

## License
MIT

