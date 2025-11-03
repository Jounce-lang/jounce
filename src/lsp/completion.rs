// LSP Completions
// Session 28

use lsp_types::*;

pub fn get_completions(source: &str, position: Position) -> Vec<CompletionItem> {
    let mut completions = vec![];

    // Get the line where cursor is
    let lines: Vec<&str> = source.lines().collect();
    if position.line as usize >= lines.len() {
        return completions;
    }

    let line = lines[position.line as usize];
    let char_pos = position.character as usize;

    // Context-aware completions
    if char_pos > 0 {
        let before_cursor = &line[..char_pos.min(line.len())];

        // Component/function keywords
        if before_cursor.ends_with("comp") || before_cursor.trim().is_empty() {
            completions.push(CompletionItem {
                label: "component".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Define a component".to_string()),
                insert_text: Some("component $1($2) {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        // Signal completions
        if before_cursor.ends_with("sig") {
            completions.push(CompletionItem {
                label: "signal".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Create a reactive signal".to_string()),
                insert_text: Some("signal($1)".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        // Common Jounce keywords
        for keyword in &["let", "const", "fn", "if", "else", "return", "component", "signal", "computed"] {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }

        // JSX completion after <
        if before_cursor.trim_end().ends_with('<') {
            for tag in &["div", "span", "p", "h1", "h2", "h3", "button", "input", "form"] {
                completions.push(CompletionItem {
                    label: tag.to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some("JSX element".to_string()),
                    insert_text: Some(format!("{}>$1</{}>", tag, tag)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                });
            }
        }
    }

    completions
}
