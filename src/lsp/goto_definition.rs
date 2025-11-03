// LSP Go to Definition
// Session 28

use lsp_types::*;

pub fn find_definition(source: &str, position: Position, uri: &Url) -> Option<Location> {
    let lines: Vec<&str> = source.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }

    let line = lines[position.line as usize];
    let char_pos = position.character as usize;

    // Find word at cursor
    let word = get_word_at_position(line, char_pos)?;

    // Search for definition in document
    for (line_idx, line_content) in lines.iter().enumerate() {
        // Look for component, function, or let declarations
        if line_content.contains(&format!("component {}", word))
            || line_content.contains(&format!("fn {}", word))
            || line_content.contains(&format!("let {} =", word))
            || line_content.contains(&format!("const {} =", word))
        {
            let col = line_content.find(&word)?;
            return Some(Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: line_idx as u32,
                        character: col as u32,
                    },
                    end: Position {
                        line: line_idx as u32,
                        character: (col + word.len()) as u32,
                    },
                },
            });
        }
    }

    None
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
