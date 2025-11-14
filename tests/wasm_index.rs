// WASM Array Index Type Tests
// Ensures array indexes must be integer-typed (E430)

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::semantic_analyzer::SemanticAnalyzer;

#[test]
fn ok_integer_index() {
    // Integer index should work
    let source = r#"
fn test() {
    let items = vec![1, 2, 3];
    let i: i32 = 0;
    let x = items[i];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(
        result.is_ok(),
        "Integer index should be valid, got: {:?}",
        result.err()
    );
}

#[test]
fn ok_literal_integer_index() {
    // Literal integer index should work
    let source = r#"
fn test() {
    let items = vec![10, 20, 30];
    let x = items[0];
    let y = items[1];
    let z = items[2];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(
        result.is_ok(),
        "Literal integer index should be valid, got: {:?}",
        result.err()
    );
}

#[test]
fn ok_range_loop_index() {
    // Range loop variables are integers
    let source = r#"
fn test() {
    let items = vec![1, 2, 3];
    for i in 0..items.len() {
        let x = items[i];
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(
        result.is_ok(),
        "Range loop index should be valid, got: {:?}",
        result.err()
    );
}

#[test]
fn fail_float_index() {
    // Float index should fail with E430
    let source = r#"
fn test() {
    let items = vec![1, 2, 3];
    let x: f32 = 1.2;
    let value = items[x];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(result.is_err(), "Float index should fail");

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("E430") && (error.contains("f32") || error.contains("f64")),
        "Error should mention E430 and float type, got: {}",
        error
    );
}

#[test]
fn fail_float_literal_index() {
    // Float literal index should fail
    let source = r#"
fn test() {
    let items = vec![1, 2, 3];
    let value = items[1.5];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(result.is_err(), "Float literal index should fail");

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("E430"),
        "Error should mention E430, got: {}",
        error
    );
}

#[test]
fn fail_string_index() {
    // String index should fail
    let source = r#"
fn test() {
    let items = vec![1, 2, 3];
    let key = "0";
    let value = items[key];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(result.is_err(), "String index should fail");

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("E430") && error.contains("string"),
        "Error should mention E430 and string type, got: {}",
        error
    );
}

#[test]
fn fail_bool_index() {
    // Boolean index should fail
    let source = r#"
fn test() {
    let items = vec![1, 2];
    let flag = true;
    let value = items[flag];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(result.is_err(), "Boolean index should fail");

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("E430") && error.contains("bool"),
        "Error should mention E430 and bool type, got: {}",
        error
    );
}

#[test]
fn fail_complex_expression_non_integer() {
    // Complex expression resulting in non-integer should fail
    let source = r#"
fn test() {
    let items = vec![10, 20, 30];
    let price: f64 = 10.5;
    let count: f64 = 2.0;
    let idx = price / count;
    let value = items[idx];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(
        result.is_err(),
        "Non-integer expression index should fail"
    );

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("E430"),
        "Error should mention E430, got: {}",
        error
    );
}

// Note: Testing explicit cast would require .to_i32() method to be implemented
// This test documents the expected behavior when cast methods are available
#[test]
#[ignore] // TODO: Implement to_i32() cast method
fn ok_cast_index() {
    // Cast to i32 should work
    let source = r#"
fn test() {
    let items = vec![10, 20, 30];
    let price: f64 = 10.5;
    let count: f64 = 2.0;
    let idx = (price / count).to_i32();
    let value = items[idx];
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Should parse successfully");

    let program = ast.unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze_program(&program);

    assert!(
        result.is_ok(),
        "Cast to i32 should be valid, got: {:?}",
        result.err()
    );
}
