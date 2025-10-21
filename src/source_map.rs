// Source Map Generator
//
// Generates source maps for JavaScript and WASM output to enable debugging
// in browsers and other tools. Maps generated code back to original .raven source.
//
// Format: Source Map v3 specification
// https://sourcemaps.info/spec.html

use serde::{Serialize, Deserialize};

/// Represents a single mapping from generated code to source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub generated_line: usize,
    pub generated_column: usize,
    pub source_file: String,
    pub source_line: usize,
    pub source_column: usize,
    pub name: Option<String>,  // Optional name of the identifier
}

/// Source map builder that collects mappings during code generation
#[derive(Debug, Clone)]
pub struct SourceMapBuilder {
    pub file: String,  // The generated file name (e.g., "client.js")
    pub source_root: Option<String>,  // Optional path prefix for source files
    pub sources: Vec<String>,  // List of source files
    pub sources_content: Vec<Option<String>>,  // Optional embedded source content
    pub names: Vec<String>,  // List of names (identifiers)
    pub mappings: Vec<Mapping>,  // All mappings
}

impl SourceMapBuilder {
    pub fn new(file: String) -> Self {
        Self {
            file,
            source_root: None,
            sources: Vec::new(),
            sources_content: Vec::new(),
            names: Vec::new(),
            mappings: Vec::new(),
        }
    }

    /// Add a mapping from generated position to source position
    pub fn add_mapping(
        &mut self,
        generated_line: usize,
        generated_column: usize,
        source_file: &str,
        source_line: usize,
        source_column: usize,
        name: Option<&str>,
    ) {
        // Ensure source file is in sources list
        if !self.sources.contains(&source_file.to_string()) {
            self.sources.push(source_file.to_string());
            self.sources_content.push(None);  // Could embed source here
        }

        // Ensure name is in names list
        let name_string = name.map(|n| {
            if !self.names.contains(&n.to_string()) {
                self.names.push(n.to_string());
            }
            n.to_string()
        });

        self.mappings.push(Mapping {
            generated_line,
            generated_column,
            source_file: source_file.to_string(),
            source_line,
            source_column,
            name: name_string,
        });
    }

    /// Set the source root (path prefix for all source files)
    pub fn set_source_root(&mut self, root: String) {
        self.source_root = Some(root);
    }

    /// Embed source content for a source file
    pub fn set_source_content(&mut self, source_file: &str, content: String) {
        if let Some(index) = self.sources.iter().position(|s| s == source_file) {
            while self.sources_content.len() <= index {
                self.sources_content.push(None);
            }
            self.sources_content[index] = Some(content);
        }
    }

    /// Generate the source map JSON
    pub fn generate(&self) -> String {
        let source_map = SourceMap {
            version: 3,
            file: self.file.clone(),
            source_root: self.source_root.clone(),
            sources: self.sources.clone(),
            sources_content: if self.sources_content.iter().any(|s| s.is_some()) {
                Some(self.sources_content.clone())
            } else {
                None
            },
            names: self.names.clone(),
            mappings: self.encode_mappings(),
        };

        serde_json::to_string_pretty(&source_map).unwrap_or_else(|_| "{}".to_string())
    }

    /// Encode mappings using VLQ (Variable-Length Quantity) Base64
    /// This is a simplified version - full implementation would use proper VLQ encoding
    fn encode_mappings(&self) -> String {
        // Sort mappings by generated position
        let mut sorted_mappings = self.mappings.clone();
        sorted_mappings.sort_by_key(|m| (m.generated_line, m.generated_column));

        let mut encoded = String::new();
        let mut current_line = 0;

        for mapping in &sorted_mappings {
            // Add semicolons for each new line
            while current_line < mapping.generated_line {
                encoded.push(';');
                current_line += 1;
            }

            // In a full implementation, this would use VLQ Base64 encoding
            // For now, we'll use a simplified format
            if !encoded.is_empty() && !encoded.ends_with(';') {
                encoded.push(',');
            }

            // Format: [gen_col, source_idx, source_line, source_col, name_idx]
            // Simplified for demonstration - real implementation uses VLQ
            let source_idx = self.sources.iter().position(|s| s == &mapping.source_file).unwrap_or(0);
            encoded.push_str(&format!(
                "{}:{}:{}:{}",
                mapping.generated_column,
                source_idx,
                mapping.source_line,
                mapping.source_column
            ));
        }

        encoded
    }

    /// Generate inline source map comment for JavaScript
    pub fn generate_inline_comment(&self) -> String {
        let json = self.generate();
        let base64 = base64::encode(&json);
        format!("//# sourceMappingURL=data:application/json;charset=utf-8;base64,{}", base64)
    }

    /// Generate reference comment for external source map file
    pub fn generate_reference_comment(&self) -> String {
        format!("//# sourceMappingURL={}.map", self.file)
    }
}

/// Source Map v3 JSON structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SourceMap {
    version: u32,
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_root: Option<String>,
    sources: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources_content: Option<Vec<Option<String>>>,
    names: Vec<String>,
    mappings: String,
}

/// Simple base64 encoding (for inline source maps)
mod base64 {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(data: &str) -> String {
        let bytes = data.as_bytes();
        let mut result = String::new();

        for chunk in bytes.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &b) in chunk.iter().enumerate() {
                buf[i] = b;
            }

            result.push(CHARS[(buf[0] >> 2) as usize] as char);
            result.push(CHARS[(((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize] as char);

            if chunk.len() > 1 {
                result.push(CHARS[(((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize] as char);
            } else {
                result.push('=');
            }

            if chunk.len() > 2 {
                result.push(CHARS[(buf[2] & 0x3f) as usize] as char);
            } else {
                result.push('=');
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_map_builder() {
        let mut builder = SourceMapBuilder::new("output.js".to_string());

        // Add some mappings
        builder.add_mapping(1, 0, "input.raven", 1, 0, Some("main"));
        builder.add_mapping(2, 4, "input.raven", 2, 2, Some("x"));
        builder.add_mapping(3, 4, "input.raven", 3, 2, None);

        // Generate source map
        let source_map = builder.generate();

        // Print for debugging
        eprintln!("Generated source map:\n{}", source_map);

        // Check format with spaces (JSON pretty print adds spaces)
        assert!(source_map.contains("\"version\": 3"));
        assert!(source_map.contains("\"file\": \"output.js\""));
        assert!(source_map.contains("input.raven"));
    }

    #[test]
    fn test_inline_comment() {
        let mut builder = SourceMapBuilder::new("test.js".to_string());
        builder.add_mapping(1, 0, "test.raven", 1, 0, None);

        let comment = builder.generate_inline_comment();
        assert!(comment.starts_with("//# sourceMappingURL=data:application/json"));
    }

    #[test]
    fn test_reference_comment() {
        let builder = SourceMapBuilder::new("app.js".to_string());
        let comment = builder.generate_reference_comment();
        assert_eq!(comment, "//# sourceMappingURL=app.js.map");
    }
}
