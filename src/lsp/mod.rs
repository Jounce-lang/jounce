// Language Server Protocol implementation for RavensOne
// Provides IDE features: autocomplete, hover, diagnostics, etc.

use crate::diagnostics::Diagnostic;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::type_checker::TypeChecker;
use std::collections::HashMap;

/// LSP Server for RavensOne
pub struct LanguageServer {
    documents: HashMap<String, Document>,
    stdlib_docs: StdlibDocs,
}

/// Represents an open document
pub struct Document {
    pub uri: String,
    pub content: String,
    pub version: i32,
    pub diagnostics: Vec<Diagnostic>,
}

/// Position in a document (line, character)
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

/// Range in a document
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Completion item for autocomplete
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Function,
    Variable,
    Keyword,
    Class,
    Module,
    Property,
    Snippet,
}

/// Hover information
#[derive(Debug, Clone)]
pub struct Hover {
    pub contents: String,
    pub range: Option<Range>,
}

impl LanguageServer {
    pub fn new() -> Self {
        LanguageServer {
            documents: HashMap::new(),
            stdlib_docs: StdlibDocs::new(),
        }
    }

    /// Open a document
    pub fn open_document(&mut self, uri: String, content: String, version: i32) {
        let diagnostics = self.analyze_document(&content);
        let document = Document {
            uri: uri.clone(),
            content,
            version,
            diagnostics,
        };
        self.documents.insert(uri, document);
    }

    /// Change document content
    pub fn change_document(&mut self, uri: &str, content: String, version: i32) {
        // Analyze first to avoid borrow checker issues
        let diagnostics = self.analyze_document(&content);

        if let Some(doc) = self.documents.get_mut(uri) {
            doc.content = content;
            doc.version = version;
            doc.diagnostics = diagnostics;
        }
    }

    /// Close a document
    pub fn close_document(&mut self, uri: &str) {
        self.documents.remove(uri);
    }

    /// Get diagnostics for a document
    pub fn get_diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        self.documents
            .get(uri)
            .map(|doc| doc.diagnostics.clone())
            .unwrap_or_default()
    }

    /// Analyze document and return diagnostics
    fn analyze_document(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Lexical analysis and parsing
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(e) => {
                diagnostics.push(Diagnostic::error(format!("{:?}", e)));
                return diagnostics;
            }
        };

        // Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        if let Err(e) = analyzer.analyze_program(&ast) {
            diagnostics.push(Diagnostic::error(format!("{:?}", e)));
            return diagnostics;
        }

        // Type checking
        let mut type_checker = TypeChecker::new();
        if let Err(e) = type_checker.check_program(&ast.statements) {
            diagnostics.push(Diagnostic::error(format!("{:?}", e)));
        }

        diagnostics
    }

    /// Get completions at a position
    #[allow(unused_variables)] // uri and position used in future context-aware completions
    pub fn get_completions(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Add keywords
        completions.extend(self.get_keyword_completions());

        // Add stdlib functions
        completions.extend(self.stdlib_docs.get_completions());

        // Add reactive primitives
        completions.extend(self.get_reactive_completions());

        // Add JSX completions
        completions.extend(self.get_jsx_completions());

        // Add local variables and functions from current scope
        if let Some(doc) = self.documents.get(uri) {
            completions.extend(self.get_scope_completions(&doc.content));
        }

        completions
    }

    /// Get hover information at a position
    pub fn get_hover(&self, uri: &str, position: Position) -> Option<Hover> {
        let doc = self.documents.get(uri)?;
        let word = self.get_word_at_position(&doc.content, position)?;

        // Check stdlib
        if let Some(docs) = self.stdlib_docs.get_documentation(&word) {
            return Some(Hover {
                contents: docs,
                range: None,
            });
        }

        // Check reactive primitives
        if let Some(docs) = self.get_reactive_docs(&word) {
            return Some(Hover {
                contents: docs,
                range: None,
            });
        }

        None
    }

    /// Get word at position
    fn get_word_at_position(&self, content: &str, position: Position) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        if position.line >= lines.len() {
            return None;
        }

        let line = lines[position.line];
        if position.character >= line.len() {
            return None;
        }

        // Find word boundaries
        let chars: Vec<char> = line.chars().collect();
        let mut start = position.character;
        let mut end = position.character;

        // Go backwards to find start
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }

        // Go forwards to find end
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }

        if start < end {
            Some(chars[start..end].iter().collect())
        } else {
            None
        }
    }

    /// Get keyword completions
    fn get_keyword_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "component".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define a component".to_string()),
                documentation: Some("Create a new component".to_string()),
                insert_text: Some("component ${1:Name}() {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "fn".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define a function".to_string()),
                documentation: None,
                insert_text: Some("fn ${1:name}($2) -> $3 {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "let".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Declare a variable".to_string()),
                documentation: None,
                insert_text: Some("let ${1:name} = $0;".to_string()),
            },
            CompletionItem {
                label: "@server".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Server-side annotation".to_string()),
                documentation: Some("Mark function as server-side only (has network access, database access)".to_string()),
                insert_text: None,
            },
            CompletionItem {
                label: "@client".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Client-side annotation".to_string()),
                documentation: Some("Mark function as client-side only (has DOM access, browser APIs)".to_string()),
                insert_text: None,
            },
            CompletionItem {
                label: "if".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Conditional".to_string()),
                documentation: None,
                insert_text: Some("if $1 {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "for".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Loop".to_string()),
                documentation: None,
                insert_text: Some("for ${1:item} in ${2:items} {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "while".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("While loop".to_string()),
                documentation: None,
                insert_text: Some("while $1 {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "match".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Pattern matching".to_string()),
                documentation: None,
                insert_text: Some("match ${1:value} {\n    ${2:pattern} => $0,\n}".to_string()),
            },
            CompletionItem {
                label: "struct".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define a struct".to_string()),
                documentation: None,
                insert_text: Some("struct ${1:Name} {\n    $0\n}".to_string()),
            },
            CompletionItem {
                label: "enum".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Define an enum".to_string()),
                documentation: None,
                insert_text: Some("enum ${1:Name} {\n    $0,\n}".to_string()),
            },
            CompletionItem {
                label: "return".to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("Return a value".to_string()),
                documentation: None,
                insert_text: Some("return $0;".to_string()),
            },
        ]
    }

    /// Get JSX-specific completions
    fn get_jsx_completions(&self) -> Vec<CompletionItem> {
        vec![
            // HTML elements
            CompletionItem {
                label: "<div>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Div element".to_string()),
                documentation: Some("Container div element".to_string()),
                insert_text: Some("<div class=\"${1:className}\">\n    $0\n</div>".to_string()),
            },
            CompletionItem {
                label: "<button>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Button element".to_string()),
                documentation: Some("Clickable button".to_string()),
                insert_text: Some("<button onclick={$1}>\n    \"${2:Click me}\"\n</button>".to_string()),
            },
            CompletionItem {
                label: "<input>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Input element".to_string()),
                documentation: Some("Text input field".to_string()),
                insert_text: Some("<input type=\"${1:text}\" value={${2:signal}.get()} oninput={(e) => ${2:signal}.set(e.target.value)} />".to_string()),
            },
            CompletionItem {
                label: "<h1>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Heading 1".to_string()),
                documentation: Some("Top-level heading".to_string()),
                insert_text: Some("<h1>\"${1:Title}\"</h1>".to_string()),
            },
            CompletionItem {
                label: "<p>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Paragraph".to_string()),
                documentation: Some("Paragraph element".to_string()),
                insert_text: Some("<p>\"${1:Text}\"</p>".to_string()),
            },
            CompletionItem {
                label: "<ul>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Unordered list".to_string()),
                documentation: Some("Bullet point list".to_string()),
                insert_text: Some("<ul>\n    {${1:items}.iter().map(|${2:item}| {\n        <li>{${2:item}}</li>\n    })}\n</ul>".to_string()),
            },
            CompletionItem {
                label: "<form>".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Form element".to_string()),
                documentation: Some("Form container".to_string()),
                insert_text: Some("<form onsubmit={$1}>\n    $0\n</form>".to_string()),
            },
            // Common patterns
            CompletionItem {
                label: "counter".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Counter component".to_string()),
                documentation: Some("Basic counter with increment/decrement".to_string()),
                insert_text: Some("component ${1:Counter}() {\n    let count = Signal::new(0);\n\n    <div>\n        <h1>\"Counter: \" {count.get()}</h1>\n        <button onclick={() => count.set(count.get() + 1)}>\"Increment\"</button>\n        <button onclick={() => count.set(count.get() - 1)}>\"Decrement\"</button>\n    </div>\n}".to_string()),
            },
            CompletionItem {
                label: "list-map".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Map array to JSX".to_string()),
                documentation: Some("Render list items".to_string()),
                insert_text: Some("{${1:items}.iter().map(|${2:item}| {\n    <div key={${2:item}.id}>\n        {${2:item}.name}\n    </div>\n})}".to_string()),
            },
            CompletionItem {
                label: "conditional".to_string(),
                kind: CompletionItemKind::Snippet,
                detail: Some("Conditional JSX".to_string()),
                documentation: Some("Render conditionally".to_string()),
                insert_text: Some("{if ${1:condition} {\n    <div>$0</div>\n}}".to_string()),
            },
        ]
    }

    /// Get reactive primitive completions
    fn get_reactive_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "Signal::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create a new Signal".to_string()),
                documentation: Some("let count = Signal::new(0);".to_string()),
                insert_text: Some("Signal::new($0)".to_string()),
            },
            CompletionItem {
                label: "Computed::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create a computed value".to_string()),
                documentation: Some("let doubled = Computed::new(|| count.get() * 2);".to_string()),
                insert_text: Some("Computed::new(|| $0)".to_string()),
            },
            CompletionItem {
                label: "Effect::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create an effect".to_string()),
                documentation: Some("Effect::new(|| { console.log(count.get()); });".to_string()),
                insert_text: Some("Effect::new(|| $0)".to_string()),
            },
            CompletionItem {
                label: "Resource::new".to_string(),
                kind: CompletionItemKind::Function,
                detail: Some("Create an async resource".to_string()),
                documentation: Some("let data = Resource::new(async { fetch_data().await });".to_string()),
                insert_text: Some("Resource::new(async { $0 })".to_string()),
            },
        ]
    }

    /// Get reactive primitive documentation
    fn get_reactive_docs(&self, word: &str) -> Option<String> {
        match word {
            "Signal" => Some("**Signal** - A reactive value that can be read and written.\n\n```raven\nlet count = Signal::new(0);\ncount.set(5);\nlet value = count.get();\n```".to_string()),
            "Computed" => Some("**Computed** - A derived value that automatically updates.\n\n```raven\nlet doubled = Computed::new(|| count.get() * 2);\n```".to_string()),
            "Effect" => Some("**Effect** - Run side effects when dependencies change.\n\n```raven\nEffect::new(|| {\n    console.log(count.get());\n});\n```".to_string()),
            "Resource" => Some("**Resource** - Async data loading with automatic refetching.\n\n```raven\nlet data = Resource::new(async {\n    fetch_data().await\n});\n```".to_string()),
            _ => None,
        }
    }

    /// Get completions from current document scope
    fn get_scope_completions(&self, content: &str) -> Vec<CompletionItem> {
        use crate::ast::Statement;

        let mut completions = Vec::new();

        // Parse the document to extract local variables and functions
        let mut lexer = Lexer::new(content.to_string());
        let mut parser = Parser::new(&mut lexer);
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(_) => return completions,
        };

        // Extract function names
        for statement in &ast.statements {
            match statement {
                Statement::Function(func_def) => {
                    completions.push(CompletionItem {
                        label: func_def.name.value.clone(),
                        kind: CompletionItemKind::Function,
                        detail: Some(format!("fn {}(...)", func_def.name.value)),
                        documentation: Some("User-defined function".to_string()),
                        insert_text: None,
                    });
                }
                Statement::Let(let_stmt) => {
                    // Add completions for all identifiers bound in the pattern
                    for ident in let_stmt.pattern.bound_identifiers() {
                        completions.push(CompletionItem {
                            label: ident.value.clone(),
                            kind: CompletionItemKind::Variable,
                            detail: Some("Local variable".to_string()),
                            documentation: None,
                            insert_text: None,
                        });
                    }
                }
                Statement::Component(comp_def) => {
                    completions.push(CompletionItem {
                        label: comp_def.name.value.clone(),
                        kind: CompletionItemKind::Class,
                        detail: Some(format!("component {}", comp_def.name.value)),
                        documentation: Some("User-defined component".to_string()),
                        insert_text: None,
                    });
                }
                _ => {}
            }
        }

        completions
    }
}

impl Default for LanguageServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard library documentation
pub struct StdlibDocs {
    functions: HashMap<String, FunctionDoc>,
}

#[derive(Debug, Clone)]
pub struct FunctionDoc {
    pub name: String,
    pub signature: String,
    pub description: String,
    pub examples: Vec<String>,
}

impl StdlibDocs {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        // ==================== MATH FUNCTIONS ====================

        // Basic operations
        functions.insert("Math::abs".to_string(), FunctionDoc {
            name: "Math::abs".to_string(),
            signature: "fn abs(x: f64) -> f64".to_string(),
            description: "Returns the absolute value of a number".to_string(),
            examples: vec!["let positive = Math::abs(-42.5); // 42.5".to_string()],
        });

        functions.insert("Math::min".to_string(), FunctionDoc {
            name: "Math::min".to_string(),
            signature: "fn min(a: f64, b: f64) -> f64".to_string(),
            description: "Returns the smaller of two numbers".to_string(),
            examples: vec!["let smallest = Math::min(10.0, 20.0); // 10.0".to_string()],
        });

        functions.insert("Math::max".to_string(), FunctionDoc {
            name: "Math::max".to_string(),
            signature: "fn max(a: f64, b: f64) -> f64".to_string(),
            description: "Returns the larger of two numbers".to_string(),
            examples: vec!["let largest = Math::max(10.0, 20.0); // 20.0".to_string()],
        });

        functions.insert("Math::clamp".to_string(), FunctionDoc {
            name: "Math::clamp".to_string(),
            signature: "fn clamp(value: f64, min: f64, max: f64) -> f64".to_string(),
            description: "Constrains a value to a range [min, max]".to_string(),
            examples: vec!["let clamped = Math::clamp(150.0, 0.0, 100.0); // 100.0".to_string()],
        });

        // Powers and roots
        functions.insert("Math::pow".to_string(), FunctionDoc {
            name: "Math::pow".to_string(),
            signature: "fn pow(base: f64, exponent: f64) -> f64".to_string(),
            description: "Raises a number to a power".to_string(),
            examples: vec!["let result = Math::pow(2.0, 8.0); // 256.0".to_string()],
        });

        functions.insert("Math::sqrt".to_string(), FunctionDoc {
            name: "Math::sqrt".to_string(),
            signature: "fn sqrt(x: f64) -> f64".to_string(),
            description: "Returns the square root of a number".to_string(),
            examples: vec!["let root = Math::sqrt(16.0); // 4.0".to_string()],
        });

        functions.insert("Math::square".to_string(), FunctionDoc {
            name: "Math::square".to_string(),
            signature: "fn square(x: f64) -> f64".to_string(),
            description: "Returns the square of a number (x²)".to_string(),
            examples: vec!["let sq = Math::square(5.0); // 25.0".to_string()],
        });

        // Rounding
        functions.insert("Math::round".to_string(), FunctionDoc {
            name: "Math::round".to_string(),
            signature: "fn round(x: f64) -> f64".to_string(),
            description: "Rounds to the nearest integer".to_string(),
            examples: vec!["let r = Math::round(3.7); // 4.0".to_string()],
        });

        functions.insert("Math::floor".to_string(), FunctionDoc {
            name: "Math::floor".to_string(),
            signature: "fn floor(x: f64) -> f64".to_string(),
            description: "Rounds down to the nearest integer".to_string(),
            examples: vec!["let f = Math::floor(3.7); // 3.0".to_string()],
        });

        functions.insert("Math::ceil".to_string(), FunctionDoc {
            name: "Math::ceil".to_string(),
            signature: "fn ceil(x: f64) -> f64".to_string(),
            description: "Rounds up to the nearest integer".to_string(),
            examples: vec!["let c = Math::ceil(3.2); // 4.0".to_string()],
        });

        // Trigonometry
        functions.insert("Math::sin".to_string(), FunctionDoc {
            name: "Math::sin".to_string(),
            signature: "fn sin(x: f64) -> f64".to_string(),
            description: "Returns the sine of an angle (in radians)".to_string(),
            examples: vec!["let s = Math::sin(Math::PI); // ~0.0".to_string()],
        });

        functions.insert("Math::cos".to_string(), FunctionDoc {
            name: "Math::cos".to_string(),
            signature: "fn cos(x: f64) -> f64".to_string(),
            description: "Returns the cosine of an angle (in radians)".to_string(),
            examples: vec!["let c = Math::cos(0.0); // 1.0".to_string()],
        });

        functions.insert("Math::tan".to_string(), FunctionDoc {
            name: "Math::tan".to_string(),
            signature: "fn tan(x: f64) -> f64".to_string(),
            description: "Returns the tangent of an angle (in radians)".to_string(),
            examples: vec!["let t = Math::tan(0.0); // 0.0".to_string()],
        });

        // Random
        functions.insert("Math::random".to_string(), FunctionDoc {
            name: "Math::random".to_string(),
            signature: "fn random() -> f64".to_string(),
            description: "Returns a random number between 0 (inclusive) and 1 (exclusive)".to_string(),
            examples: vec!["let r = Math::random(); // e.g., 0.742".to_string()],
        });

        functions.insert("Math::random_int".to_string(), FunctionDoc {
            name: "Math::random_int".to_string(),
            signature: "fn random_int(min: i32, max: i32) -> i32".to_string(),
            description: "Returns a random integer in the range [min, max] inclusive".to_string(),
            examples: vec!["let dice = Math::random_int(1, 6); // 1-6".to_string()],
        });

        // ==================== REACTIVE FUNCTIONS ====================

        functions.insert("Signal::new".to_string(), FunctionDoc {
            name: "Signal::new".to_string(),
            signature: "fn new<T>(initial: T) -> Signal<T>".to_string(),
            description: "Creates a new reactive signal with an initial value".to_string(),
            examples: vec!["let count = Signal::new(0);".to_string()],
        });

        functions.insert("Signal::get".to_string(), FunctionDoc {
            name: "signal.get".to_string(),
            signature: "fn get(&self) -> T".to_string(),
            description: "Gets the current value and tracks the dependency".to_string(),
            examples: vec!["let value = count.get();".to_string()],
        });

        functions.insert("Signal::set".to_string(), FunctionDoc {
            name: "signal.set".to_string(),
            signature: "fn set(&self, value: T)".to_string(),
            description: "Sets a new value and notifies all dependents".to_string(),
            examples: vec!["count.set(5);".to_string()],
        });

        functions.insert("Computed::new".to_string(), FunctionDoc {
            name: "Computed::new".to_string(),
            signature: "fn new<T>(fn() -> T) -> Computed<T>".to_string(),
            description: "Creates a computed value that automatically updates when dependencies change".to_string(),
            examples: vec!["let doubled = Computed::new(|| count.get() * 2);".to_string()],
        });

        functions.insert("create_effect".to_string(), FunctionDoc {
            name: "create_effect".to_string(),
            signature: "fn create_effect(fn())".to_string(),
            description: "Creates an effect that runs when dependencies change".to_string(),
            examples: vec!["create_effect(|| println!(\"Count: {}\", count.get()));".to_string()],
        });

        // ==================== HTTP FUNCTIONS ====================

        functions.insert("HttpRequest::get".to_string(), FunctionDoc {
            name: "HttpRequest::get".to_string(),
            signature: "fn get(url: &str) -> HttpRequest".to_string(),
            description: "Creates a GET request".to_string(),
            examples: vec!["let resp = HttpRequest::get(\"https://api.example.com/users\").send().await;".to_string()],
        });

        functions.insert("HttpRequest::post".to_string(), FunctionDoc {
            name: "HttpRequest::post".to_string(),
            signature: "fn post(url: &str) -> HttpRequest".to_string(),
            description: "Creates a POST request".to_string(),
            examples: vec!["let resp = HttpRequest::post(url).json(data).send().await;".to_string()],
        });

        functions.insert("get".to_string(), FunctionDoc {
            name: "get".to_string(),
            signature: "@server async fn get(url: &str) -> Result<HttpResponse, String>".to_string(),
            description: "Convenience function for quick GET requests".to_string(),
            examples: vec!["let resp = get(\"https://api.example.com/data\").await;".to_string()],
        });

        // ==================== COLLECTIONS FUNCTIONS ====================

        functions.insert("Collections::filter".to_string(), FunctionDoc {
            name: "Collections::filter".to_string(),
            signature: "fn filter<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> Vec<T>".to_string(),
            description: "Filters elements matching a predicate".to_string(),
            examples: vec!["let evens = Collections::filter(&numbers, |n| n % 2 == 0);".to_string()],
        });

        functions.insert("Collections::map".to_string(), FunctionDoc {
            name: "Collections::map".to_string(),
            signature: "fn map<T, U>(vec: &Vec<T>, mapper: fn(&T) -> U) -> Vec<U>".to_string(),
            description: "Transforms elements with a mapper function".to_string(),
            examples: vec!["let doubled = Collections::map(&numbers, |n| n * 2);".to_string()],
        });

        functions.insert("Collections::find".to_string(), FunctionDoc {
            name: "Collections::find".to_string(),
            signature: "fn find<T>(vec: &Vec<T>, predicate: fn(&T) -> bool) -> Option<&T>".to_string(),
            description: "Finds the first element matching a predicate".to_string(),
            examples: vec!["let found = Collections::find(&items, |i| i.id == 42);".to_string()],
        });

        // ==================== STRING FUNCTIONS ====================

        functions.insert("String::split".to_string(), FunctionDoc {
            name: "String::split".to_string(),
            signature: "fn split(s: &str, delimiter: &str) -> Vec<String>".to_string(),
            description: "Splits a string by a delimiter".to_string(),
            examples: vec!["let parts = String::split(\"a,b,c\", \",\"); // [\"a\", \"b\", \"c\"]".to_string()],
        });

        functions.insert("String::trim".to_string(), FunctionDoc {
            name: "String::trim".to_string(),
            signature: "fn trim(s: &str) -> String".to_string(),
            description: "Removes whitespace from both ends of a string".to_string(),
            examples: vec!["let trimmed = String::trim(\"  hello  \"); // \"hello\"".to_string()],
        });

        functions.insert("String::contains".to_string(), FunctionDoc {
            name: "String::contains".to_string(),
            signature: "fn contains(haystack: &str, needle: &str) -> bool".to_string(),
            description: "Checks if a string contains a substring".to_string(),
            examples: vec!["let has_at = String::contains(email, \"@\");".to_string()],
        });

        // ==================== STORAGE FUNCTIONS ====================

        functions.insert("Storage::set".to_string(), FunctionDoc {
            name: "Storage::set".to_string(),
            signature: "@client fn set(key: &str, value: &str)".to_string(),
            description: "Stores a value in localStorage".to_string(),
            examples: vec!["Storage::set(\"theme\", \"dark\");".to_string()],
        });

        functions.insert("Storage::get".to_string(), FunctionDoc {
            name: "Storage::get".to_string(),
            signature: "@client fn get(key: &str) -> Option<String>".to_string(),
            description: "Retrieves a value from localStorage".to_string(),
            examples: vec!["let theme = Storage::get(\"theme\");".to_string()],
        });

        functions.insert("Storage::set_json".to_string(), FunctionDoc {
            name: "Storage::set_json".to_string(),
            signature: "@client fn set_json(key: &str, value: &Value)".to_string(),
            description: "Stores a JSON value in localStorage".to_string(),
            examples: vec!["Storage::set_json(\"cart\", &cart_data);".to_string()],
        });

        // ==================== FORMS FUNCTIONS ====================

        functions.insert("Forms::validate_email".to_string(), FunctionDoc {
            name: "Forms::validate_email".to_string(),
            signature: "fn validate_email(email: &str) -> bool".to_string(),
            description: "Validates email format".to_string(),
            examples: vec!["if Forms::validate_email(&email) { /* valid */ }".to_string()],
        });

        functions.insert("Forms::validate_required".to_string(), FunctionDoc {
            name: "Forms::validate_required".to_string(),
            signature: "fn validate_required(value: &str) -> bool".to_string(),
            description: "Checks if a value is non-empty".to_string(),
            examples: vec!["if !Forms::validate_required(&username) { /* error */ }".to_string()],
        });

        // ==================== TIME FUNCTIONS ====================

        functions.insert("Time::now".to_string(), FunctionDoc {
            name: "Time::now".to_string(),
            signature: "fn now() -> i64".to_string(),
            description: "Gets current Unix timestamp in seconds".to_string(),
            examples: vec!["let timestamp = Time::now();".to_string()],
        });

        functions.insert("Time::sleep".to_string(), FunctionDoc {
            name: "Time::sleep".to_string(),
            signature: "fn sleep(ms: u64)".to_string(),
            description: "Sleeps for specified milliseconds (blocking)".to_string(),
            examples: vec!["Time::sleep(1000); // Sleep 1 second".to_string()],
        });

        // ==================== JSON FUNCTIONS ====================

        functions.insert("JSON::parse".to_string(), FunctionDoc {
            name: "JSON::parse".to_string(),
            signature: "fn parse(text: &str) -> Result<Value, String>".to_string(),
            description: "Parses a JSON string".to_string(),
            examples: vec!["let data = JSON::parse(json_str)?;".to_string()],
        });

        functions.insert("JSON::stringify".to_string(), FunctionDoc {
            name: "JSON::stringify".to_string(),
            signature: "fn stringify(value: &Value) -> String".to_string(),
            description: "Converts a value to JSON string".to_string(),
            examples: vec!["let json_str = JSON::stringify(&user);".to_string()],
        });

        // ==================== AUTH FUNCTIONS ====================

        functions.insert("Auth::hash_password".to_string(), FunctionDoc {
            name: "Auth::hash_password".to_string(),
            signature: "@server fn hash_password(password: &str) -> String".to_string(),
            description: "Hashes a password using bcrypt".to_string(),
            examples: vec!["let hashed = Auth::hash_password(&password);".to_string()],
        });

        functions.insert("Auth::verify_password".to_string(), FunctionDoc {
            name: "Auth::verify_password".to_string(),
            signature: "@server fn verify_password(password: &str, hash: &str) -> bool".to_string(),
            description: "Verifies a password against a hash".to_string(),
            examples: vec!["if Auth::verify_password(&password, &user.password_hash) { /* valid */ }".to_string()],
        });

        // ==================== CONSOLE FUNCTIONS ====================

        functions.insert("println".to_string(), FunctionDoc {
            name: "println".to_string(),
            signature: "fn println!(format: &str, ...)".to_string(),
            description: "Prints a formatted message with newline".to_string(),
            examples: vec!["println!(\"Count: {}\", count);".to_string()],
        });

        functions.insert("console.log".to_string(), FunctionDoc {
            name: "console.log".to_string(),
            signature: "fn log(message: Any)".to_string(),
            description: "Logs a message to the console".to_string(),
            examples: vec!["console.log(\"Hello, world!\");".to_string()],
        });

        StdlibDocs { functions }
    }

    pub fn get_completions(&self) -> Vec<CompletionItem> {
        self.functions
            .values()
            .map(|doc| CompletionItem {
                label: doc.name.clone(),
                kind: CompletionItemKind::Function,
                detail: Some(doc.signature.clone()),
                documentation: Some(doc.description.clone()),
                insert_text: None,
            })
            .collect()
    }

    pub fn get_documentation(&self, name: &str) -> Option<String> {
        self.functions.get(name).map(|doc| {
            format!(
                "**{}**\n\n{}\n\n```raven\n{}\n```\n\n{}",
                doc.name,
                doc.signature,
                doc.examples.join("\n"),
                doc.description
            )
        })
    }
}

impl Default for StdlibDocs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_server_open_document() {
        let mut server = LanguageServer::new();
        server.open_document(
            "file:///test.raven".to_string(),
            "let x = 10;".to_string(),
            1,
        );

        assert!(server.documents.contains_key("file:///test.raven"));
    }

    #[test]
    fn test_get_completions() {
        let server = LanguageServer::new();
        let completions = server.get_completions("file:///test.raven", Position { line: 0, character: 0 });

        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "component"));
        assert!(completions.iter().any(|c| c.label == "Signal::new"));
    }

    #[test]
    fn test_get_word_at_position() {
        let server = LanguageServer::new();
        let content = "let count = Signal::new(0);";
        let word = server.get_word_at_position(content, Position { line: 0, character: 5 });

        assert_eq!(word, Some("count".to_string()));
    }

    #[test]
    fn test_reactive_docs() {
        let server = LanguageServer::new();
        let docs = server.get_reactive_docs("Signal");

        assert!(docs.is_some());
        assert!(docs.unwrap().contains("reactive value"));
    }
}
