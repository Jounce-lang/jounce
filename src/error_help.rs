// Error Help Text Database
// Provides helpful suggestions and explanations for common compile errors

use std::collections::HashMap;

pub struct ErrorHelp {
    suggestions: HashMap<String, ErrorHelpEntry>,
}

#[derive(Clone)]
pub struct ErrorHelpEntry {
    pub code: &'static str,
    pub title: &'static str,
    pub suggestion: &'static str,
    pub example: Option<&'static str>,
}

impl ErrorHelp {
    pub fn new() -> Self {
        let mut suggestions = HashMap::new();

        // Parser Errors
        suggestions.insert(
            "expected_semicolon".to_string(),
            ErrorHelpEntry {
                code: "E001",
                title: "Missing semicolon",
                suggestion: "Add a semicolon (;) at the end of the statement",
                example: Some("let count = 0;  // ← semicolon required"),
            },
        );

        suggestions.insert(
            "unexpected_token".to_string(),
            ErrorHelpEntry {
                code: "E002",
                title: "Unexpected token",
                suggestion: "Check for syntax errors, missing braces, or incorrect token placement",
                example: None,
            },
        );

        suggestions.insert(
            "expected_identifier".to_string(),
            ErrorHelpEntry {
                code: "E003",
                title: "Expected identifier",
                suggestion: "An identifier (variable or function name) was expected here",
                example: Some("let name = \"value\";  // 'name' is the identifier"),
            },
        );

        suggestions.insert(
            "expected_expression".to_string(),
            ErrorHelpEntry {
                code: "E004",
                title: "Expected expression",
                suggestion: "An expression (value, variable, or computation) was expected",
                example: Some("let x = count + 1;  // 'count + 1' is an expression"),
            },
        );

        // Component Errors
        suggestions.insert(
            "component_syntax".to_string(),
            ErrorHelpEntry {
                code: "E010",
                title: "Invalid component syntax",
                suggestion: "Components must follow the format: component Name(params) { ... }",
                example: Some("component Card(title: String) {\n    return <div>{title}</div>;\n}"),
            },
        );

        suggestions.insert(
            "component_name".to_string(),
            ErrorHelpEntry {
                code: "E011",
                title: "Invalid component name",
                suggestion: "Component names must start with an uppercase letter",
                example: Some("component Card() { ... }  // ✓ Correct\ncomponent card() { ... }  // ✗ Wrong"),
            },
        );

        // JSX Errors
        suggestions.insert(
            "jsx_unclosed_tag".to_string(),
            ErrorHelpEntry {
                code: "E020",
                title: "Unclosed JSX tag",
                suggestion: "Every JSX opening tag must have a matching closing tag",
                example: Some("<div>content</div>  // ✓ Correct\n<div>content        // ✗ Missing </div>"),
            },
        );

        suggestions.insert(
            "jsx_invalid_attribute".to_string(),
            ErrorHelpEntry {
                code: "E021",
                title: "Invalid JSX attribute",
                suggestion: "Check attribute syntax: name=\"value\" or name={expression}",
                example: Some("<button class=\"btn\">Click</button>\n<button onclick={handler}>Click</button>"),
            },
        );

        suggestions.insert(
            "jsx_invalid_expression".to_string(),
            ErrorHelpEntry {
                code: "E022",
                title: "Invalid JSX expression",
                suggestion: "JSX expressions must be wrapped in curly braces { }",
                example: Some("<div>{count}</div>  // ✓ Correct\n<div>count</div>    // ✗ Renders literal text"),
            },
        );

        // Reactivity Errors
        suggestions.insert(
            "signal_without_value".to_string(),
            ErrorHelpEntry {
                code: "E030",
                title: "Accessing signal without .value",
                suggestion: "Signals must be accessed with .value property",
                example: Some("let count = signal(0);\nconsole.log(count.value);  // ✓ Correct\nconsole.log(count);        // ✗ Wrong"),
            },
        );

        suggestions.insert(
            "effect_syntax".to_string(),
            ErrorHelpEntry {
                code: "E031",
                title: "Invalid effect syntax",
                suggestion: "Effects must be created with effect(() => { ... }) syntax",
                example: Some("effect(() => {\n    console.log(count.value);\n});"),
            },
        );

        // Type Errors
        suggestions.insert(
            "type_mismatch".to_string(),
            ErrorHelpEntry {
                code: "E040",
                title: "Type mismatch",
                suggestion: "The value type doesn't match the expected type",
                example: Some("let count: Int = 0;   // ✓ Correct\nlet count: Int = \"0\"; // ✗ String not Int"),
            },
        );

        suggestions.insert(
            "undefined_variable".to_string(),
            ErrorHelpEntry {
                code: "E041",
                title: "Undefined variable",
                suggestion: "The variable hasn't been declared. Did you forget to define it?",
                example: Some("let count = 0;       // ✓ Declare first\nconsole.log(count);  // Then use"),
            },
        );

        // Brace/Paren Errors
        suggestions.insert(
            "missing_closing_brace".to_string(),
            ErrorHelpEntry {
                code: "E050",
                title: "Missing closing brace",
                suggestion: "Every opening brace { must have a matching closing brace }",
                example: Some("if (condition) {\n    doSomething();\n}  // ← closing brace required"),
            },
        );

        suggestions.insert(
            "missing_closing_paren".to_string(),
            ErrorHelpEntry {
                code: "E051",
                title: "Missing closing parenthesis",
                suggestion: "Every opening paren ( must have a matching closing paren )",
                example: Some("function add(a, b) {  // ← closing paren required\n    return a + b;\n}"),
            },
        );

        suggestions.insert(
            "unexpected_closing_brace".to_string(),
            ErrorHelpEntry {
                code: "E052",
                title: "Unexpected closing brace",
                suggestion: "This closing brace } doesn't match an opening brace. Check for extra braces.",
                example: None,
            },
        );

        // Import/Export Errors
        suggestions.insert(
            "import_not_found".to_string(),
            ErrorHelpEntry {
                code: "E060",
                title: "Import not found",
                suggestion: "The imported module or package doesn't exist. Check the path.",
                example: Some("import Button from \"./components/Button.jnc\";"),
            },
        );

        suggestions.insert(
            "export_syntax".to_string(),
            ErrorHelpEntry {
                code: "E061",
                title: "Invalid export syntax",
                suggestion: "Use 'export' keyword before component, function, or let declarations",
                example: Some("export component Button() { ... }\nexport let API_URL = \"...\";"),
            },
        );

        // Style Errors
        suggestions.insert(
            "style_syntax".to_string(),
            ErrorHelpEntry {
                code: "E070",
                title: "Invalid style syntax",
                suggestion: "Styles can be inline objects or style blocks",
                example: Some("<div style={{ color: \"red\" }}>Text</div>\n\nstyle Button {\n    background: blue;\n}"),
            },
        );

        ErrorHelp { suggestions }
    }

    /// Get help for a specific error pattern
    pub fn get_help(&self, pattern: &str) -> Option<&ErrorHelpEntry> {
        self.suggestions.get(pattern)
    }

    /// Get help suggestion for a general error message
    pub fn suggest_from_message(&self, message: &str) -> Option<&ErrorHelpEntry> {
        let lower = message.to_lowercase();

        // Pattern matching based on error message content
        if lower.contains("semicolon") || lower.contains("expected ';'") {
            return self.get_help("expected_semicolon");
        }

        if lower.contains("expected identifier") {
            return self.get_help("expected_identifier");
        }

        if lower.contains("expected expression") {
            return self.get_help("expected_expression");
        }

        if lower.contains("unclosed") && lower.contains("tag") {
            return self.get_help("jsx_unclosed_tag");
        }

        if lower.contains("component") && (lower.contains("syntax") || lower.contains("invalid")) {
            return self.get_help("component_syntax");
        }

        if lower.contains("signal") && lower.contains("value") {
            return self.get_help("signal_without_value");
        }

        if lower.contains("type mismatch") {
            return self.get_help("type_mismatch");
        }

        if lower.contains("undefined") && lower.contains("variable") {
            return self.get_help("undefined_variable");
        }

        if lower.contains("missing") && lower.contains("}") || lower.contains("closing brace") {
            return self.get_help("missing_closing_brace");
        }

        if lower.contains("missing") && lower.contains(")") || lower.contains("closing paren") {
            return self.get_help("missing_closing_paren");
        }

        if lower.contains("unexpected") && lower.contains("}") {
            return self.get_help("unexpected_closing_brace");
        }

        if lower.contains("import") && (lower.contains("not found") || lower.contains("doesn't exist")) {
            return self.get_help("import_not_found");
        }

        None
    }

    /// Format a complete help message with code and example
    pub fn format_help(&self, entry: &ErrorHelpEntry) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n💡 {} [{}]\n", entry.title, entry.code));
        output.push_str(&format!("   {}\n", entry.suggestion));

        if let Some(example) = entry.example {
            output.push_str("\n📝 Example:\n");
            for line in example.lines() {
                output.push_str(&format!("   {}\n", line));
            }
        }

        output
    }
}

impl Default for ErrorHelp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_help() {
        let help = ErrorHelp::new();

        let entry = help.get_help("expected_semicolon");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().code, "E001");
    }

    #[test]
    fn test_suggest_from_message() {
        let help = ErrorHelp::new();

        let entry = help.suggest_from_message("Expected ';' after statement");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().code, "E001");

        let entry = help.suggest_from_message("Unclosed JSX tag detected");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().code, "E020");
    }

    #[test]
    fn test_format_help() {
        let help = ErrorHelp::new();
        let entry = help.get_help("expected_semicolon").unwrap();
        let formatted = help.format_help(entry);

        assert!(formatted.contains("E001"));
        assert!(formatted.contains("semicolon"));
        assert!(formatted.contains("Example"));
    }
}
