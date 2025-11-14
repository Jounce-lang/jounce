// Style & @media Regression Tests
// Ensures @media support and nesting limits remain correct

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;

#[test]
fn ok_media_top_level() {
    // @media at top level inside style block should parse
    let source = r#"
component Card() {
    return <div class="card">Hello</div>;
}

style Card {
    .card {
        padding: 20px;
        background: white;
    }

    @media (max-width: 768px) {
        .card {
            padding: 10px;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Top-level @media should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_multiple_top_level_media() {
    // Multiple @media blocks at top level should parse
    let source = r#"
component Responsive() {
    return <div class="container">Test</div>;
}

style Responsive {
    .container {
        padding: 40px;
    }

    @media (max-width: 1024px) {
        .container {
            padding: 30px;
        }
    }

    @media (max-width: 768px) {
        .container {
            padding: 20px;
        }
    }

    @media (max-width: 480px) {
        .container {
            padding: 10px;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Multiple top-level @media blocks should parse, got: {:?}",
        ast.err()
    );
}

#[test]
#[ignore] // TODO: Pseudo-selectors (:hover) not yet supported
fn ok_media_with_complex_selectors() {
    // @media with pseudo-classes and child combinators
    let source = r#"
component Nav() {
    return <nav><ul><li><a>Link</a></li></ul></nav>;
}

style Nav {
    nav {
        background: white;
    }

    @media (max-width: 768px) {
        nav {
            background: #f5f5f5;
        }

        ul {
            display: flex;
        }

        li {
            margin: 0 10px;
        }

        a:hover {
            color: blue;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "@media with complex selectors should parse, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_max_depth_nesting() {
    // Reasonable nesting depth should work
    // One level: .card { &:hover { .child { ... } } }
    let source = r#"
component Card() {
    return <div class="card"><span class="child">Test</span></div>;
}

style Card {
    .card {
        padding: 20px;

        &:hover {
            background: #f0f0f0;

            .child {
                color: blue;
            }
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Reasonable nesting depth should parse, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_media_with_nested_selectors() {
    // @media containing nested selectors should be fine
    let source = r#"
component Button() {
    return <button class="btn"><span>Click</span></button>;
}

style Button {
    @media (max-width: 768px) {
        .btn {
            padding: 8px;

            span {
                font-size: 14px;
            }

            &:hover {
                background: blue;
            }
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "@media with nested selectors should parse, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_media_query_variations() {
    // Test different @media query syntaxes
    let source = r#"
component Responsive() {
    return <div>Test</div>;
}

style Responsive {
    @media screen and (min-width: 768px) {
        .container {
            width: 750px;
        }
    }

    @media (orientation: landscape) {
        .landscape {
            display: block;
        }
    }

    @media print {
        .no-print {
            display: none;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Various @media query syntaxes should parse, got: {:?}",
        ast.err()
    );
}

#[test]
#[ignore] // TODO: Pseudo-selectors (:focus, :disabled, etc.) not yet supported
fn ok_pseudo_selectors_and_combinators() {
    // Test various pseudo-selectors and combinators
    let source = r#"
component Form() {
    return <form><input /><label></label></form>;
}

style Form {
    input:focus {
        border-color: blue;
    }

    input:disabled {
        opacity: 0.5;
    }

    label + input {
        margin-left: 10px;
    }

    input:first-child {
        margin-top: 0;
    }

    div > span {
        color: red;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Pseudo-selectors and combinators should parse, got: {:?}",
        ast.err()
    );
}

// Note: Tests for failure cases (nested @media, @media in selector, excessive nesting)
// would require checking compiler error output, which is handled by the formatter tests.
// These tests focus on ensuring valid syntax continues to parse correctly.
