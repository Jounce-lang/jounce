// Tests for nested selector depth tracking in style blocks
// Verifies that nesting depth is enforced (max depth: 3)

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;

#[test]
fn ok_nesting_depth_1() {
    // Test single level of nesting (depth 1)
    let source = r#"
component Card() {
    return <div><p>Content</p></div>;
}

style Card {
    padding: 20px;

    p {
        color: blue;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Nesting depth 1 should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_nesting_depth_2() {
    // Test two levels of nesting (depth 2)
    let source = r#"
component Card() {
    return <div><div class="header"><h1>Title</h1></div></div>;
}

style Card {
    padding: 20px;

    .header {
        background: gray;

        h1 {
            color: white;
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Nesting depth 2 should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_nesting_depth_3() {
    // Test three levels of nesting (depth 3 - maximum allowed)
    let source = r#"
component Card() {
    return <div><div class="outer"><div class="inner"><span>Text</span></div></div></div>;
}

style Card {
    padding: 20px;

    .outer {
        margin: 10px;

        .inner {
            padding: 5px;

            span {
                font-weight: bold;
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
        "Nesting depth 3 should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_nesting_depth_3_with_ampersand() {
    // Test depth 3 with & selectors
    let source = r#"
component Card() {
    return <div><button><span>Click</span></button></div>;
}

style Card {
    padding: 20px;

    button {
        background: blue;

        span {
            color: white;

            &:hover {
                text-decoration: underline;
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
        "Nesting depth 3 with & should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn err_nesting_depth_4() {
    // Test four levels of nesting (depth 4 - should fail)
    let source = r#"
component Card() {
    return <div>Card</div>;
}

style Card {
    padding: 20px;

    .level1 {
        margin: 10px;

        .level2 {
            padding: 5px;

            .level3 {
                color: blue;

                .level4 {
                    font-weight: bold;
                }
            }
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Nesting depth 4 should fail with E_STY_005"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_005"),
            "Error should contain E_STY_005 code, got: {}",
            err_str
        );
    }
}

#[test]
fn err_nesting_depth_exceeded_with_elements() {
    // Test depth limit with element selectors
    let source = r#"
component Nav() {
    return <nav><ul><li><a><span>Link</span></a></li></ul></nav>;
}

style Nav {
    ul {
        li {
            a {
                span {
                    color: red;
                }
            }
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Nesting depth exceeding 3 should fail with E_STY_005"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_005"),
            "Error should contain E_STY_005 code, got: {}",
            err_str
        );
    }
}

#[test]
fn ok_media_query_does_not_count_as_depth() {
    // Test that @media does NOT count toward nesting depth
    let source = r#"
component Card() {
    return <div><div class="inner"><span>Text</span></div></div>;
}

style Card {
    padding: 20px;

    @media (max-width: 600px) {
        .inner {
            margin: 10px;

            span {
                font-size: 14px;

                &:hover {
                    color: blue;
                }
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
        "@media should not count as nesting depth, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_keyframes_does_not_count_as_depth() {
    // Test that @keyframes does NOT count toward nesting depth
    let source = r#"
component Card() {
    return <div><div class="inner"><span>Text</span></div></div>;
}

style Card {
    .inner {
        span {
            &:hover {
                animation: fadeIn 1s;
            }
        }
    }

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
        "@keyframes should not count as nesting depth, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_multiple_depth_3_branches() {
    // Test multiple independent branches at depth 3
    let source = r#"
component Layout() {
    return <div><header></header><main></main><footer></footer></div>;
}

style Layout {
    header {
        .logo {
            img {
                width: 100px;
            }
        }
    }

    main {
        .content {
            p {
                line-height: 1.6;
            }
        }
    }

    footer {
        .links {
            a {
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
        "Multiple branches at depth 3 should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_child_combinator_at_depth_3() {
    // Test child combinator (>) at maximum depth
    let source = r#"
component Card() {
    return <div><div class="wrapper"><div class="inner">Content</div></div></div>;
}

style Card {
    .wrapper {
        > .inner {
            > div {
                padding: 10px;
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
        "Child combinator at depth 3 should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn err_child_combinator_exceeds_depth() {
    // Test child combinator exceeding depth limit
    let source = r#"
component Card() {
    return <div>Card</div>;
}

style Card {
    > .a {
        > .b {
            > .c {
                > .d {
                    color: red;
                }
            }
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Child combinator exceeding depth 3 should fail with E_STY_005"
    );

    if let Err(err) = ast {
        let err_str = format!("{}", err);
        assert!(
            err_str.contains("E_STY_005"),
            "Error should contain E_STY_005 code, got: {}",
            err_str
        );
    }
}

#[test]
fn ok_pseudo_elements_at_depth_3() {
    // Test pseudo-elements at maximum depth
    let source = r#"
component Card() {
    return <div><p><span>Text</span></p></div>;
}

style Card {
    p {
        span {
            &::before {
                content: "→";
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
        "Pseudo-elements at depth 3 should parse successfully, got: {:?}",
        ast.err()
    );
}
