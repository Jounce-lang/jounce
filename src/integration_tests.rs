// Integration tests that compile full .raven programs end-to-end
// These tests validate that the entire compilation pipeline works correctly

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::type_checker::TypeChecker;
use crate::borrow_checker::BorrowChecker;
use crate::js_emitter::JSEmitter;
use crate::errors::CompileError;

/// Helper function to compile a source string end-to-end
fn compile_source(source: &str) -> Result<(String, String), CompileError> {
    // Lexer
    let mut lexer = Lexer::new(source.to_string());

    // Parser
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program()?;

    // Semantic Analyzer
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(&program)?;

    // Type Checker
    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&program.statements)?;

    // Borrow Checker
    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program)?;

    // Code Generation
    let emitter = JSEmitter::new(&program);
    let server_js = emitter.generate_server_js();
    let client_js = emitter.generate_client_js();

    Ok((server_js, client_js))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else_compiles() {
        let source = r#"
            fn main() {
                if true {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if/else statement should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("if"));
        assert!(server_js.contains("else"));
    }

    #[test]
    fn test_if_else_expression() {
        let source = r#"
            fn max(a: i32, b: i32) -> i32 {
                if a > b {
                    a
                } else {
                    b
                }
            }

            fn main() {
                let result = max(5, 3);
                println!("Max: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if/else expression should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("module.exports.max"));
        assert!(server_js.contains("if"));
        assert!(server_js.contains("else"));
    }

    #[test]
    fn test_nested_if_else() {
        let source = r#"
            fn classify(n: i32) -> String {
                if n < 0 {
                    "negative"
                } else {
                    if n == 0 {
                        "zero"
                    } else {
                        "positive"
                    }
                }
            }

            fn main() {
                let result = classify(5);
                println!("Classification: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested if/else should compile successfully");
    }

    #[test]
    fn test_if_else_in_loop() {
        let source = r#"
            fn main() {
                let numbers = [1, 2, 3, 4, 5];
                for num in numbers {
                    if num > 3 {
                        println!("Large: {}", num);
                    } else {
                        println!("Small: {}", num);
                    }
                }
            }
        "#;

        let result = compile_source(source);
        // The test should at minimum compile successfully
        // Note: for-in loop support may be limited in current implementation
        assert!(result.is_ok(), "if/else in loop should compile successfully");
    }

    #[test]
    fn test_recursive_function() {
        let source = r#"
            fn factorial(n: i32) -> i32 {
                if n <= 1 {
                    1
                } else {
                    n * factorial(n - 1)
                }
            }

            fn main() {
                let result = factorial(5);
                println!("Factorial: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "recursive function should compile successfully");

        let (server_js, _) = result.unwrap();
        assert!(server_js.contains("module.exports.factorial"));
        assert!(server_js.contains("factorial"));  // Recursive call
    }

    #[test]
    fn test_multiple_returns() {
        let source = r#"
            fn check_value(n: i32) -> String {
                if n < 0 {
                    return "negative";
                } else {
                    if n == 0 {
                        return "zero";
                    } else {
                        return "positive";
                    }
                }
            }

            fn main() {
                println!("Result: {}", check_value(5));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "multiple returns with if/else should compile");
    }

    #[test]
    fn test_if_without_else() {
        let source = r#"
            fn main() {
                let x = 5;
                if x > 0 {
                    println!("positive");
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "if without else should still compile");
    }

    #[test]
    fn test_complex_conditions() {
        let source = r#"
            fn check(a: i32, b: i32) -> bool {
                if a > 0 && b > 0 {
                    true
                } else {
                    false
                }
            }

            fn main() {
                let result = check(5, 3);
                println!("Result: {}", result);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "complex conditions should compile");
    }

    #[test]
    fn test_for_loop_exclusive_range() {
        let source = r#"
            fn main() {
                for i in 1..5 {
                    println!("{}", i);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "for loop with exclusive range should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should generate: for (let i = 1; i < 5; i++)
        assert!(client_js.contains("for"));
        assert!(client_js.contains("let i"));
        assert!(client_js.contains("i < 5"));
        assert!(client_js.contains("i++"));
    }

    #[test]
    fn test_for_loop_inclusive_range() {
        let source = r#"
            fn main() {
                for i in 1..=5 {
                    println!("{}", i);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "for loop with inclusive range should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should generate: for (let i = 1; i <= 5; i++)
        assert!(client_js.contains("for"));
        assert!(client_js.contains("let i"));
        assert!(client_js.contains("i <= 5"));
        assert!(client_js.contains("i++"));
    }

    #[test]
    fn test_for_loop_range_with_variables() {
        let source = r#"
            fn main() {
                let start = 10;
                let end = 20;
                for i in start..end {
                    println!("{}", i);
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "for loop with variable range should compile successfully");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("for"));
        assert!(client_js.contains("let i"));
        assert!(client_js.contains("start"));
        assert!(client_js.contains("end"));
    }

    #[test]
    fn test_nested_for_loops() {
        let source = r#"
            fn main() {
                for i in 1..3 {
                    for j in 1..3 {
                        println!("{} {}", i, j);
                    }
                }
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "nested for loops should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should have two for loops
        let for_count = client_js.matches("for (let").count();
        assert_eq!(for_count, 2, "should have exactly 2 for loops");
    }

    #[test]
    fn test_match_or_patterns() {
        let source = r#"
            fn classify(n: i32) -> String {
                match n {
                    1 => "one",
                    3 | 4 | 5 => "three to five",
                    _ => "other",
                }
            }

            fn main() {
                println!("{}", classify(3));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with OR patterns should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should generate: (__match_value === 3 || __match_value === 4 || __match_value === 5)
        assert!(client_js.contains("||"), "should have OR operator");
        assert!(client_js.contains("=== 3"), "should check for 3");
        assert!(client_js.contains("=== 4"), "should check for 4");
        assert!(client_js.contains("=== 5"), "should check for 5");
    }

    #[test]
    fn test_match_multiple_or_patterns() {
        let source = r#"
            fn classify(n: i32) -> String {
                match n {
                    1 | 2 => "one or two",
                    3 | 4 | 5 => "three to five",
                    6 | 7 | 8 | 9 => "six to nine",
                    _ => "other",
                }
            }

            fn main() {
                let result = classify(7);
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with multiple OR patterns should compile successfully");

        let (_, client_js) = result.unwrap();
        // Should have multiple OR conditions
        assert!(client_js.contains("=== 6 || "), "should check for 6");
        assert!(client_js.contains("=== 7 || "), "should check for 7");
        assert!(client_js.contains("=== 9"), "should check for 9");
    }

    #[test]
    fn test_match_single_pattern_still_works() {
        let source = r#"
            fn test(n: i32) -> String {
                match n {
                    1 => "one",
                    2 => "two",
                    _ => "other",
                }
            }

            fn main() {
                println!("{}", test(1));
            }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "match with single patterns should still work");

        let (_, client_js) = result.unwrap();
        assert!(client_js.contains("=== 1"), "should check for 1");
        assert!(client_js.contains("=== 2"), "should check for 2");
    }
}
