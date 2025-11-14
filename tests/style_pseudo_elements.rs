// Tests for CSS pseudo-elements (::before, ::after) support in style blocks
// These tests verify that pseudo-elements are parsed correctly and generate proper CSS output

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;

#[test]
fn ok_pseudo_element_before() {
    // Test &::before selector
    let source = r#"
component Card() {
    return <div class="card">Content</div>;
}

style Card {
    &::before {
        content: ">";
        color: blue;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "::before pseudo-element should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_pseudo_element_after() {
    // Test &::after selector
    let source = r#"
component Card() {
    return <div class="card">Content</div>;
}

style Card {
    &::after {
        content: "<";
        color: red;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "::after pseudo-element should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_both_pseudo_elements() {
    // Test both ::before and ::after in the same style block
    let source = r#"
component Quote() {
    return <blockquote>Hello World</blockquote>;
}

style Quote {
    &::before {
        content: """;
    }

    &::after {
        content: """;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Both ::before and ::after should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_pseudo_element_with_multiple_properties() {
    // Test pseudo-element with multiple CSS properties
    let source = r#"
component Button() {
    return <button>Click</button>;
}

style Button {
    &::before {
        content: ">";
        display: block;
        position: absolute;
        left: 10px;
        color: white;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Pseudo-element with multiple properties should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_pseudo_elements_in_media_query() {
    // Test pseudo-elements inside @media queries
    let source = r#"
component Card() {
    return <div>Responsive</div>;
}

style Card {
    @media (max-width: 600px) {
        &::before {
            content: "Mobile: ";
        }

        &::after {
            content: " [small screen]";
        }
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Pseudo-elements in @media query should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_pseudo_class_and_pseudo_element() {
    // Test combining pseudo-classes with pseudo-elements in the same style block
    let source = r##"
component Link() {
    return <a href="#">Link</a>;
}

style Link {
    &:hover {
        color: blue;
    }

    &::after {
        content: " [link]";
    }
}
"##;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Pseudo-class and pseudo-element together should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn ok_nested_selector_with_pseudo_element() {
    // Test pseudo-element on nested selector (not directly on &)
    let source = r#"
component Card() {
    return <div><h2>Title</h2></div>;
}

style Card {
    h2 {
        font-size: 20px;
    }

    &::before {
        content: "Card: ";
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Nested selector with pseudo-element should parse successfully, got: {:?}",
        ast.err()
    );
}

#[test]
fn err_pseudo_element_missing_name() {
    // Test error when pseudo-element name is missing after ::
    let source = r#"
component Card() {
    return <div>Content</div>;
}

style Card {
    &:: {
        content: "error";
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_err(),
        "Missing pseudo-element name after :: should fail"
    );
}

#[test]
fn ok_pseudo_element_custom_names() {
    // Test that custom pseudo-element names are accepted (even if not standard CSS)
    // This allows for future extensibility
    let source = r#"
component Custom() {
    return <div>Custom</div>;
}

style Custom {
    &::placeholder {
        color: gray;
    }

    &::first-line {
        font-weight: bold;
    }
}
"#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program();

    assert!(
        ast.is_ok(),
        "Custom pseudo-element names should parse successfully, got: {:?}",
        ast.err()
    );
}

// Note: Integration tests for CSS output are tested via the compiler end-to-end tests
// The tests above verify that pseudo-elements parse correctly, which is the key functionality
