// Test struct literals with spread syntax
// Regression test for bug where struct spreads generated invalid JS

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::js_emitter::JSEmitter;

#[test]
fn test_struct_with_spread_basic() {
    let source = r#"
        struct Task {
            id: i32,
            label: string,
            done: bool,
        }

        component App() {
            let tasks = signal<Vec<Task>>(vec![
                Task { id: 1, label: "Test", done: false },
            ]);

            fn toggleTask(id: i32) {
                tasks.value = tasks.value.map((t) => {
                    if t.id == id {
                        return Task {
                            ...t,
                            done: !t.done,
                        };
                    }
                    return t;
                });
            }

            return <div>{tasks.value.len().to_string()}</div>;
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Struct with spread should parse successfully, got: {:?}",
        ast.err()
    );

    // Generate JavaScript to verify correct codegen
    let program = ast.unwrap();
    let emitter = JSEmitter::new(&program);
    let client_js = emitter.generate_client_js();

    // Verify that the spread is compiled correctly as an object literal
    // Should generate: { ...t, done: !t.done }
    // NOT: Task; { ...t, done: !t.done }
    assert!(
        client_js.contains("{ ...t, done:"),
        "Struct spread not compiled correctly - should generate object literal with spread"
    );

    // Verify there's no orphaned struct name before the object literal
    assert!(
        !client_js.contains("return Task;"),
        "Invalid code: struct name separated from object literal"
    );

    println!("✓ Struct spread compiled correctly to valid JavaScript");
}

#[test]
fn test_struct_with_multiple_spreads() {
    let source = r#"
        struct Point {
            x: i32,
            y: i32,
        }

        component App() {
            let p1 = Point { x: 1, y: 2 };
            let p2 = Point { ...p1, x: 10 };
            let p3 = Point { ...p2, y: 20 };

            return <div>{p3.x.to_string()}</div>;
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Multiple struct spreads should parse successfully, got: {:?}",
        ast.err()
    );

    let program = ast.unwrap();
    let emitter = JSEmitter::new(&program);
    let client_js = emitter.generate_client_js();

    // Check that both spreads are compiled correctly
    assert!(
        client_js.matches("{ ...").count() >= 2,
        "Multiple spreads not compiled correctly"
    );

    println!("✓ Multiple struct spreads compiled correctly");
}

#[test]
fn test_struct_spread_with_multiple_fields() {
    let source = r#"
        struct User {
            id: i32,
            name: string,
            email: string,
            active: bool,
        }

        component App() {
            let user = User { id: 1, name: "Alice", email: "alice@example.com", active: true };
            let updated = User {
                ...user,
                email: "newemail@example.com",
                active: false,
            };

            return <div>{updated.name}</div>;
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Struct with multiple field spreads should parse, got: {:?}",
        ast.err()
    );

    let program = ast.unwrap();
    let emitter = JSEmitter::new(&program);
    let client_js = emitter.generate_client_js();

    // Verify correct object literal syntax with spread
    assert!(
        client_js.contains("{ ...user,"),
        "Struct spread with multiple fields not compiled correctly"
    );

    println!("✓ Struct spread with multiple overridden fields compiled correctly");
}

#[test]
fn test_struct_spread_in_nested_expression() {
    let source = r#"
        struct Config {
            enabled: bool,
            value: i32,
        }

        component App() {
            let configs = vec![
                Config { enabled: true, value: 10 },
                Config { enabled: false, value: 20 },
            ];

            let updated = configs.map((c) => Config {
                ...c,
                enabled: !c.enabled,
            });

            return <div>{updated.len().to_string()}</div>;
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Struct spread in nested expression should parse, got: {:?}",
        ast.err()
    );

    let program = ast.unwrap();
    let emitter = JSEmitter::new(&program);
    let client_js = emitter.generate_client_js();

    // Verify spread works inside lambda/map
    assert!(
        client_js.contains("{ ...c,"),
        "Struct spread in nested expression not compiled correctly"
    );

    println!("✓ Struct spread in nested expression compiled correctly");
}
