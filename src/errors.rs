use std::fmt;
use crate::diagnostics::{Diagnostic, DiagnosticBuilder, SourceLocation};

#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    LexerError(String),
    ParserError { message: String, line: usize, column: usize },
    BorrowError(String),
    Generic(String),
    /// Error with source location for better diagnostics
    WithLocation {
        message: String,
        location: SourceLocation,
        suggestion: Option<String>,
    },
    /// Style-specific parsing errors
    StyleError {
        message: String,
        line: usize,
        column: usize,
        code: String,
    },
}

impl CompileError {
    /// Convert this CompileError to a Diagnostic for beautiful display
    pub fn to_diagnostic(&self, file: &str) -> Diagnostic {
        match self {
            CompileError::LexerError(msg) => {
                Diagnostic::error(msg.clone())
                    .with_code("E0001")
            }
            CompileError::ParserError { message, line, column } => {
                DiagnosticBuilder::syntax_error(
                    "",
                    message,
                    SourceLocation {
                        file: file.to_string(),
                        line: *line,
                        column: *column,
                        length: 1,
                    },
                )
            }
            CompileError::BorrowError(msg) => {
                DiagnosticBuilder::borrow_error(
                    msg,
                    SourceLocation {
                        file: file.to_string(),
                        line: 0,
                        column: 0,
                        length: 0,
                    },
                )
            }
            CompileError::Generic(msg) => {
                Diagnostic::error(msg.clone())
            }
            CompileError::WithLocation { message, location, suggestion } => {
                let mut diag = Diagnostic::error(message.clone())
                    .at(location.clone());
                if let Some(sugg) = suggestion {
                    diag = diag.with_suggestion(sugg.clone());
                }
                diag
            }
            CompileError::StyleError { message, line, column, code } => {
                Diagnostic::error(message.clone())
                    .with_code(code)
                    .at(SourceLocation {
                        file: file.to_string(),
                        line: *line,
                        column: *column,
                        length: 1,
                    })
                    .with_suggestion("Jounce currently supports one level of selector nesting inside `style <Component> { ... }`".to_string())
                    .with_note("See docs/guides/SYNTAX_LIMITATIONS.md for current CSS syntax".to_string())
            }
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::LexerError(msg) => write!(f, "Lexer Error: {}", msg),
            CompileError::ParserError { message, line, column } => {
                write!(f, "Parser Error [{}:{}]: {}", line, column, message)
            }
            CompileError::BorrowError(msg) => write!(f, "Borrow Error: {}", msg),
            CompileError::Generic(msg) => write!(f, "Error: {}", msg),
            CompileError::WithLocation { message, .. } => write!(f, "Error: {}", message),
            CompileError::StyleError { message, line, column, code } => {
                write!(f, "Style Error [{}:{}] [{}]: {}", line, column, code, message)
            }
        }
    }
}
impl std::error::Error for CompileError {}