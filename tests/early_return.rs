// Test early return in functions (Fix #2: Early Return in Functions)

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::type_checker::TypeChecker;
use jounce_compiler::js_emitter::JSEmitter;

fn compile_and_emit(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().map_err(|e| format!("{:?}", e))?;

    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&ast.statements).map_err(|e| format!("{:?}", e))?;

    let emitter = JSEmitter::new(&ast);
    Ok(emitter.generate_client_js())
}

#[test]
fn test_simple_early_return() {
    let source = r#"
        fn validate(value: i32) -> bool {
            if value < 0 {
                return false;
            }
            return true;
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Early return should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("return false"), "Should contain early return");
    assert!(js.contains("return true"), "Should contain final return");
}

#[test]
fn test_early_return_in_loop() {
    let source = r#"
        fn find_positive(items: Vec<i32>) -> i32 {
            for item in items {
                if item > 0 {
                    return item;
                }
            }
            return -1;
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Early return in loop should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("return"), "Should contain return statements");
}

#[test]
fn test_multiple_early_returns() {
    let source = r#"
        fn classify(num: i32) -> String {
            if num < 0 {
                return "negative";
            }
            if num == 0 {
                return "zero";
            }
            return "positive";
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Multiple early returns should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.matches("return").count() >= 3, "Should contain at least 3 return statements");
}

#[test]
fn test_early_return_nested_if() {
    let source = r#"
        fn check_bounds(x: i32, y: i32) -> bool {
            if x < 0 {
                return false;
            }
            if x > 100 {
                return false;
            }
            if y < 0 {
                return false;
            }
            if y > 100 {
                return false;
            }
            return true;
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Nested early returns should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.matches("return").count() >= 5, "Should contain at least 5 return statements");
}

#[test]
fn test_early_return_with_expression() {
    let source = r#"
        fn double_if_positive(num: i32) -> i32 {
            if num <= 0 {
                return 0;
            }
            return num * 2;
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Early return with expression should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("return 0"), "Should contain early return with 0");
    assert!(js.contains("num * 2"), "Should contain multiplication expression");
}

#[test]
fn test_early_return_match_expression() {
    let source = r#"
        fn get_grade(score: i32) -> String {
            if score < 0 {
                return "Invalid";
            }
            if score >= 90 {
                return "A";
            }
            if score >= 80 {
                return "B";
            }
            return "C";
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Early return with match should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("return"), "Should contain return statements");
}

#[test]
fn test_no_early_return_implicit_return() {
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Implicit return should still work: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("return"), "Should contain return statement for implicit return");
}

#[test]
fn test_early_return_vs_implicit_return() {
    let source = r#"
        fn max(a: i32, b: i32) -> i32 {
            if a > b {
                return a;
            }
            b
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Early return with implicit return should compile: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("return a"), "Should contain early return");
    assert!(js.contains("return b") || js.contains("return  b"), "Should convert implicit return to explicit");
}
