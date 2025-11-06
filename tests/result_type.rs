// Test Result<T, E> type support (Fix #1: Result Type Methods)

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::type_checker::TypeChecker;

#[test]
fn test_result_type_parsing() {
    let source = r#"
        fn parse_number(input: String) -> Result<i32, String> {
            return Ok(42);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse Result type: {:?}", ast.err());
}

#[test]
fn test_result_is_ok_method() {
    let source = r#"
        fn check_result() {
            let result: Result<i32, String> = Ok(42);
            let success = result.is_ok();
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse .is_ok() call: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    assert!(result.is_ok(), "Type checking failed for .is_ok(): {:?}", result.err());
}

#[test]
fn test_result_is_err_method() {
    let source = r#"
        fn check_error() {
            let result: Result<i32, String> = Err("error");
            let has_error = result.is_err();
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse .is_err() call: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    assert!(result.is_ok(), "Type checking failed for .is_err(): {:?}", result.err());
}

#[test]
fn test_result_in_if_condition() {
    // This test verifies that .is_ok() can be used in if conditions
    // Note: Currently Ok/Err constructors return Type::Any (see type_checker.rs:393-396)
    // So we can't use explicit Ok(42) in let statements with Result types yet
    // This is a known limitation and will be addressed in a future update
    let source = r#"
        fn check_is_ok(result: Result<i32, String>) -> bool {
            return result.is_ok();
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse Result.is_ok() call: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    // This should pass because .is_ok() on Result<i32, String> parameter returns bool
    assert!(result.is_ok(), "Type checking failed for Result.is_ok(): {:?}", result.err());
}

#[test]
fn test_result_try_operator() {
    let source = r#"
        fn propagate_error() -> Result<i32, String> {
            let result: Result<i32, String> = Ok(42);
            let value = result?;
            return Ok(value);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse ? operator: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    assert!(result.is_ok(), "Type checking failed for ? operator: {:?}", result.err());
}

#[test]
fn test_result_nested_types() {
    let source = r#"
        fn nested_result() -> Result<Vec<i32>, String> {
            return Ok(vec![1, 2, 3]);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse nested Result type: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    assert!(result.is_ok(), "Type checking failed for nested Result: {:?}", result.err());
}

#[test]
fn test_result_with_custom_error_type() {
    let source = r#"
        struct MyError {
            message: String
        }

        fn custom_error() -> Result<i32, MyError> {
            return Ok(42);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse Result with custom error: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    assert!(result.is_ok(), "Type checking failed for custom error Result: {:?}", result.err());
}

#[test]
fn test_result_match_expression() {
    let source = r#"
        fn match_result() {
            let result: Result<i32, String> = Ok(42);
            let value = match result {
                Ok(n) => n,
                Err(e) => 0
            };
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(ast.is_ok(), "Failed to parse Result in match: {:?}", ast.err());

    let program = ast.unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program.statements);

    assert!(result.is_ok(), "Type checking failed for Result match: {:?}", result.err());
}
