// Test security annotations parsing (Phase 17)

use jounce::lexer::Lexer;
use jounce::parser::Parser;

#[test]
fn test_simple_secure_annotation() {
    let source = r#"
        @secure
        fn sensitive_operation() {
            console.log("Secure function");
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse @secure annotation: {:?}", ast.err());
    let program = ast.unwrap();

    // Find the function statement
    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some(), "Function not found in AST");
    let func = func.unwrap();

    assert_eq!(func.annotations.len(), 1, "Expected 1 annotation");
    assert_eq!(func.annotations[0].name.value, "secure");
    assert_eq!(func.annotations[0].arguments.len(), 0);
}

#[test]
fn test_auth_annotation_with_role() {
    let source = r#"
        @auth(role = "admin")
        fn delete_user(id: i64) {
            db.delete("users", id);
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse @auth annotation: {:?}", ast.err());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    assert_eq!(func.annotations.len(), 1);
    assert_eq!(func.annotations[0].name.value, "auth");
    assert_eq!(func.annotations[0].arguments.len(), 1);
    assert_eq!(func.annotations[0].arguments[0].name, "role");

    match &func.annotations[0].arguments[0].value {
        jounce::ast::AnnotationValue::String(s) => assert_eq!(s, "admin"),
        _ => panic!("Expected string value for role"),
    }
}

#[test]
fn test_auth_annotation_with_roles_array() {
    let source = r#"
        @auth(roles = ["admin", "moderator"])
        fn ban_user(id: i64) {
            // Implementation
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse @auth with array: {:?}", ast.err());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    assert_eq!(func.annotations.len(), 1);
    assert_eq!(func.annotations[0].arguments[0].name, "roles");

    match &func.annotations[0].arguments[0].value {
        jounce::ast::AnnotationValue::Array(arr) => {
            assert_eq!(arr.len(), 2);
            match &arr[0] {
                jounce::ast::AnnotationValue::String(s) => assert_eq!(s, "admin"),
                _ => panic!("Expected string in array"),
            }
            match &arr[1] {
                jounce::ast::AnnotationValue::String(s) => assert_eq!(s, "moderator"),
                _ => panic!("Expected string in array"),
            }
        }
        _ => panic!("Expected array value for roles"),
    }
}

#[test]
fn test_validate_annotation_with_schema() {
    let source = r#"
        @validate(schema = UserSchema)
        fn create_user(data: UserData) {
            // Implementation
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse @validate annotation: {:?}", ast.err());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    assert_eq!(func.annotations.len(), 1);
    assert_eq!(func.annotations[0].name.value, "validate");
    assert_eq!(func.annotations[0].arguments[0].name, "schema");

    match &func.annotations[0].arguments[0].value {
        jounce::ast::AnnotationValue::Identifier(id) => assert_eq!(id, "UserSchema"),
        _ => panic!("Expected identifier value for schema"),
    }
}

#[test]
fn test_ratelimit_annotation() {
    let source = r#"
        @ratelimit(max = 100, window = 60)
        fn search_products(query: String) {
            // Implementation
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse @ratelimit annotation: {:?}", ast.err());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    assert_eq!(func.annotations.len(), 1);
    assert_eq!(func.annotations[0].name.value, "ratelimit");
    assert_eq!(func.annotations[0].arguments.len(), 2);

    // Check max argument
    let max_arg = func.annotations[0].arguments.iter().find(|a| a.name == "max");
    assert!(max_arg.is_some());
    match &max_arg.unwrap().value {
        jounce::ast::AnnotationValue::Integer(n) => assert_eq!(*n, 100),
        _ => panic!("Expected integer value for max"),
    }

    // Check window argument
    let window_arg = func.annotations[0].arguments.iter().find(|a| a.name == "window");
    assert!(window_arg.is_some());
    match &window_arg.unwrap().value {
        jounce::ast::AnnotationValue::Integer(n) => assert_eq!(*n, 60),
        _ => panic!("Expected integer value for window"),
    }
}

#[test]
fn test_multiple_annotations() {
    let source = r#"
        @secure
        @auth(role = "admin")
        @validate(schema = UserSchema)
        @ratelimit(max = 10, window = 60)
        fn admin_create_user(data: UserData) {
            // Implementation
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse multiple annotations: {:?}", ast.err());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    // Should have 4 annotations
    assert_eq!(func.annotations.len(), 4);

    // Verify each annotation
    assert_eq!(func.annotations[0].name.value, "secure");
    assert_eq!(func.annotations[1].name.value, "auth");
    assert_eq!(func.annotations[2].name.value, "validate");
    assert_eq!(func.annotations[3].name.value, "ratelimit");
}

#[test]
fn test_annotation_with_server_decorator() {
    let source = r#"
        @auth(role = "admin")
        @server
        fn server_admin_function() {
            // Implementation
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok(), "Failed to parse annotation with @server: {:?}", ast.err());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    // Should have 1 annotation (security) + server flag
    assert_eq!(func.annotations.len(), 1);
    assert_eq!(func.annotations[0].name.value, "auth");
    assert!(func.is_server, "Expected is_server to be true");
}

#[test]
fn test_function_without_annotations() {
    let source = r#"
        fn regular_function() {
            console.log("No annotations");
        }
    "#;

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    assert!(ast.is_ok());
    let program = ast.unwrap();

    let func = program.statements.iter().find_map(|stmt| {
        if let jounce::ast::Statement::Function(f) = stmt {
            Some(f)
        } else {
            None
        }
    });

    assert!(func.is_some());
    let func = func.unwrap();

    // Should have no annotations
    assert_eq!(func.annotations.len(), 0);
}
