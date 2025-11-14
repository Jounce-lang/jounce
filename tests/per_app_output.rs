// Per-app output directory tests
// Regression tests to ensure each compiled app goes to its own directory

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::js_emitter::JSEmitter;
use std::path::PathBuf;

/// Helper to extract app folder name (mirrors main.rs logic)
fn get_app_folder_name(input_path: &std::path::Path) -> String {
    if let Some(parent) = input_path.parent() {
        if let Some(folder_name) = parent.file_name() {
            let name = folder_name.to_string_lossy();
            return sanitize_folder_name(&name);
        }
    }
    "app".to_string()
}

fn sanitize_folder_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c == ' ' {
                '-'
            } else {
                '_'
            }
        })
        .collect()
}

#[test]
fn test_app_folder_extraction_from_nested_path() {
    let path = PathBuf::from("examples/apps/36-animations/main.jnc");
    let folder = get_app_folder_name(&path);
    assert_eq!(folder, "36-animations");
}

#[test]
fn test_app_folder_extraction_from_simple_path() {
    let path = PathBuf::from("my-app/main.jnc");
    let folder = get_app_folder_name(&path);
    assert_eq!(folder, "my-app");
}

#[test]
fn test_app_folder_extraction_root_file() {
    let path = PathBuf::from("main.jnc");
    let folder = get_app_folder_name(&path);
    assert_eq!(folder, "app"); // fallback
}

#[test]
fn test_app_folder_sanitization() {
    assert_eq!(sanitize_folder_name("my app"), "my-app");
    assert_eq!(sanitize_folder_name("app@2.0"), "app_2_0");
    assert_eq!(sanitize_folder_name("test-app"), "test-app");
    assert_eq!(sanitize_folder_name("test_app"), "test_app");
    assert_eq!(sanitize_folder_name("01-counter"), "01-counter");
}

#[test]
fn test_compilation_creates_correct_directory_structure() {
    // Test that different input paths result in different output directories
    let paths_and_expected = vec![
        ("examples/apps/01-click-counter/main.jnc", "01-click-counter"),
        ("examples/apps/todo-app/main.jnc", "todo-app"),
        ("examples/apps/36-animations/main.jnc", "36-animations"),
        ("src/app.jnc", "src"),
    ];

    for (input, expected_folder) in paths_and_expected {
        let path = PathBuf::from(input);
        let folder = get_app_folder_name(&path);
        assert_eq!(
            folder, expected_folder,
            "Path {} should map to folder {}",
            input, expected_folder
        );
    }
}

#[test]
fn test_different_apps_compile_to_different_dirs() {
    // Simulate compiling two different apps
    let app1_path = PathBuf::from("examples/apps/app1/main.jnc");
    let app2_path = PathBuf::from("examples/apps/app2/main.jnc");

    let dir1 = get_app_folder_name(&app1_path);
    let dir2 = get_app_folder_name(&app2_path);

    assert_ne!(dir1, dir2, "Different apps should have different output directories");
    assert_eq!(dir1, "app1");
    assert_eq!(dir2, "app2");
}

#[test]
fn test_parse_basic_apps_for_directory_structure() {
    // Minimal Jounce program for testing
    let source1 = r#"
        component Counter() {
            let count = signal<i32>(0);
            return <div>{count.value.to_string()}</div>;
        }
    "#;

    let source2 = r#"
        component TodoApp() {
            let todos = signal<Vec<string>>(vec![]);
            return <div>{todos.value.len().to_string()}</div>;
        }
    "#;

    // Parse both programs successfully
    let mut lexer1 = Lexer::new(source1.to_string());
    let mut parser1 = Parser::new(&mut lexer1, source1);
    let ast1 = parser1.parse_program();
    assert!(ast1.is_ok(), "First app should parse successfully");

    let mut lexer2 = Lexer::new(source2.to_string());
    let mut parser2 = Parser::new(&mut lexer2, source2);
    let ast2 = parser2.parse_program();
    assert!(ast2.is_ok(), "Second app should parse successfully");

    // Generate JS for both
    let program1 = ast1.unwrap();
    let emitter1 = JSEmitter::new(&program1);
    let client1 = emitter1.generate_client_js();

    let program2 = ast2.unwrap();
    let emitter2 = JSEmitter::new(&program2);
    let client2 = emitter2.generate_client_js();

    // They should be different
    assert_ne!(client1, client2, "Different apps should generate different JS");
}

#[test]
fn test_absolute_vs_relative_paths() {
    // Test that both absolute and relative paths work
    let relative = PathBuf::from("apps/my-app/main.jnc");
    let absolute = PathBuf::from("/Users/dev/projects/apps/my-app/main.jnc");

    let folder_relative = get_app_folder_name(&relative);
    let folder_absolute = get_app_folder_name(&absolute);

    // Both should extract "my-app"
    assert_eq!(folder_relative, "my-app");
    assert_eq!(folder_absolute, "my-app");
}

#[test]
#[cfg(target_os = "windows")]
fn test_windows_style_paths() {
    // Test Windows-style paths
    let path = PathBuf::from("C:\\Users\\dev\\apps\\my-app\\main.jnc");
    let folder = get_app_folder_name(&path);
    assert_eq!(folder, "my-app");
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_unix_style_paths() {
    // Test Unix-style paths
    let path = PathBuf::from("/Users/dev/apps/my-app/main.jnc");
    let folder = get_app_folder_name(&path);
    assert_eq!(folder, "my-app");
}

#[test]
fn test_special_characters_in_folder_names() {
    assert_eq!(sanitize_folder_name("my@app"), "my_app");
    assert_eq!(sanitize_folder_name("app#1"), "app_1");
    assert_eq!(sanitize_folder_name("test.app"), "test_app");
    assert_eq!(sanitize_folder_name("app (beta)"), "app-_beta_");
}

#[test]
fn test_unicode_folder_names() {
    // Test that unicode characters are handled
    let folder = sanitize_folder_name("café");
    // Unicode letters should be preserved if alphanumeric
    assert!(!folder.is_empty(), "Sanitized folder name should not be empty");
}
