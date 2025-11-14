// Tests for @keyframes support in style blocks
// Verifies parsing, scoping, and CSS generation for CSS animations

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;

#[test]
fn ok_keyframes_from_to() {
    // Test basic keyframe with from/to selectors
    let source = r#"
component Box() {
    return <div class="box">Animated</div>;
}

style Box {
    animation: slideIn 0.3s ease-in;

    @keyframes slideIn {
        from {
            transform: translateX(-100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "@keyframes with from/to should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_keyframes_percentages() {
    // Test keyframes with percentage selectors
    let source = r#"
component Spinner() {
    return <div class="spinner">Loading</div>;
}

style Spinner {
    animation: rotate 2s linear infinite;

    @keyframes rotate {
        0% {
            transform: rotate(0deg);
        }
        50% {
            transform: rotate(180deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "@keyframes with percentage selectors should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_multiple_keyframes() {
    // Test multiple @keyframes in one style block
    let source = r#"
component Card() {
    return <div>Card</div>;
}

style Card {
    animation: fadeIn 0.3s, slideUp 0.5s;

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    @keyframes slideUp {
        from { transform: translateY(20px); }
        to { transform: translateY(0); }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Multiple @keyframes should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_keyframes_with_nested_selectors() {
    // Test @keyframes alongside nested selectors
    let source = r#"
component Button() {
    return <button>Click</button>;
}

style Button {
    padding: 10px;

    &:hover {
        animation: pulse 0.3s;
    }

    @keyframes pulse {
        0% { transform: scale(1); }
        50% { transform: scale(1.1); }
        100% { transform: scale(1); }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "@keyframes with nested selectors should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_keyframes_mixed_selectors() {
    // Test keyframes with mix of from/to and percentages
    let source = r#"
component Box() {
    return <div>Box</div>;
}

style Box {
    @keyframes bounce {
        from { transform: translateY(0); }
        50% { transform: translateY(-20px); }
        to { transform: translateY(0); }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Mixed from/to and percentage selectors should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_keyframes_multiple_properties() {
    // Test keyframe with multiple CSS properties
    let source = r#"
component Card() {
    return <div>Card</div>;
}

style Card {
    @keyframes entrance {
        from {
            transform: translateY(20px) scale(0.95);
            opacity: 0;
            filter: blur(10px);
        }
        to {
            transform: translateY(0) scale(1);
            opacity: 1;
            filter: blur(0);
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Keyframe with multiple properties should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn err_keyframes_invalid_selector() {
    // Test error when keyframe selector is invalid
    let source = r#"
component Box() {
    return <div>Box</div>;
}

style Box {
    @keyframes invalid {
        middle {
            opacity: 0.5;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Invalid keyframe selector should fail with E_STY_004"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_004"),
            "Error should contain E_STY_004 code, got: {}",
            err_str
        );
    }
}

#[test]
fn err_keyframes_missing_percent() {
    // Test error when percentage is missing % symbol
    let source = r#"
component Box() {
    return <div>Box</div>;
}

style Box {
    @keyframes bad {
        50 {
            opacity: 0.5;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Keyframe selector without % should fail with E_STY_004"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_004"),
            "Error should contain E_STY_004 code, got: {}",
            err_str
        );
    }
}

#[test]
fn ok_keyframes_animation_name_property() {
    // Test using animation-name property instead of animation shorthand
    let source = r#"
component Box() {
    return <div>Box</div>;
}

style Box {
    animation-name: fadeIn;
    animation-duration: 1s;

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "animation-name property should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn err_keyframes_negative_percent() {
    // Test error when percentage is negative
    let source = r#"
component Box() {
    return <div>Box</div>;
}

style Box {
    @keyframes bad {
        -10% {
            opacity: 0.5;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Negative percentage should fail with E_STY_004"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_004") || err_str.contains("between 0% and 100%"),
            "Error should mention valid percentage range, got: {}",
            err_str
        );
    }
}

#[test]
fn err_keyframes_over_100_percent() {
    // Test error when percentage is over 100%
    let source = r#"
component Box() {
    return <div>Box</div>;
}

style Box {
    @keyframes bad {
        110% {
            opacity: 0.5;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Percentage over 100% should fail with E_STY_004"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_004") || err_str.contains("between 0% and 100%"),
            "Error should mention valid percentage range, got: {}",
            err_str
        );
    }
}

// Note: Integration tests for CSS output are verified via the compiler end-to-end tests
// The tests above verify that keyframes parse correctly, which is the key functionality
