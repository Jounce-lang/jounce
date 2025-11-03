// LSP Diagnostics
// Session 28

use lsp_types::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn analyze_document(source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];

    // Create lexer and parser
    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);

    // Parse the program
    match parser.parse_program() {
        Ok(_) => {
            // Success - no diagnostics
        }
        Err(e) => {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: 10 },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: e.to_string(),
                source: Some("jounce".to_string()),
                ..Default::default()
            });
        }
    }

    diagnostics
}
