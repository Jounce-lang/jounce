// LSP Hover Information
// Session 28

use lsp_types::*;

pub fn get_hover_info(source: &str, position: Position) -> Option<Hover> {
    let lines: Vec<&str> = source.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }

    let line = lines[position.line as usize];
    let char_pos = position.character as usize;

    // Find word at cursor
    let word = get_word_at_position(line, char_pos)?;

    // Provide hover info for known keywords/functions
    let content = match word.as_str() {
        "signal" => "Creates a reactive signal that triggers updates when changed.\n\n```jounce\nlet count = signal(0);\ncount.value = 1; // Triggers reactivity\n```",
        "computed" => "Creates a computed value that automatically updates when dependencies change.\n\n```jounce\nlet doubled = computed(() => count.value * 2);\n```",
        "component" => "Defines a reusable UI component.\n\n```jounce\ncomponent Button(label: String) {\n    <button>{label}</button>\n}\n```",
        "let" => "Declares a variable binding.\n\n```jounce\nlet x = 5;\n```",
        "const" => "Declares a constant value.\n\n```jounce\nconst PI = 3.14159;\n```",
        "fn" => "Defines a function.\n\n```jounce\nfn add(a: i32, b: i32) -> i32 {\n    return a + b;\n}\n```",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content.to_string(),
        }),
        range: None,
    })
}

fn get_word_at_position(line: &str, pos: usize) -> Option<String> {
    if pos > line.len() {
        return None;
    }

    let before = &line[..pos];
    let after = &line[pos..];

    let start = before
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);

    let end = after
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(after.len());

    let word = &line[start..pos + end];
    if word.is_empty() {
        None
    } else {
        Some(word.to_string())
    }
}
