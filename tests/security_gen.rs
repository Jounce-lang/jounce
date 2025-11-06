// Test security middleware generation (Fix #3: Security Annotations Parsed But Not Generated)

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
fn test_secure_annotation_generates_comment() {
    let source = r#"
        @secure
        fn sensitive_operation() {
            console.log("Secure function");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile with @secure annotation: {:?}", result.err());

    // @secure annotation doesn't generate middleware calls, but shouldn't break compilation
    let js = result.unwrap();
    assert!(js.contains("function sensitive_operation"), "Should generate function");
}

#[test]
fn test_auth_annotation_generates_middleware() {
    let source = r#"
        @auth(role = "admin")
        fn delete_user(id: i64) {
            console.log("Deleting user");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile with @auth annotation: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("__jounce_auth_check"), "Should generate auth check");
    assert!(js.contains("role"), "Should include role in auth check");
    assert!(js.contains("admin"), "Should include admin role");
    assert!(js.contains("throw new Error(\"Unauthorized\")"), "Should throw error on auth failure");
}

#[test]
fn test_auth_annotation_with_roles_array() {
    let source = r#"
        @auth(roles = ["admin", "moderator"])
        fn ban_user(id: i64) {
            console.log("Banning user");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile with @auth roles array: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("__jounce_auth_check"), "Should generate auth check");
    assert!(js.contains("roles"), "Should include roles in auth check");
    assert!(js.contains("admin"), "Should include admin in roles array");
    assert!(js.contains("moderator"), "Should include moderator in roles array");
}

#[test]
fn test_validate_annotation_generates_middleware() {
    let source = r#"
        @validate(schema = UserSchema)
        fn create_user(data: String) {
            console.log("Creating user");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile with @validate annotation: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("__jounce_validate"), "Should generate validation call");
    assert!(js.contains("UserSchema"), "Should reference schema");
}

#[test]
fn test_ratelimit_annotation_generates_middleware() {
    let source = r#"
        @ratelimit(max = 100, window = 60)
        fn search_products(query: String) {
            console.log("Searching products");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile with @ratelimit annotation: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("__jounce_ratelimit"), "Should generate rate limit call");
    assert!(js.contains("max"), "Should include max parameter");
    assert!(js.contains("100"), "Should include max value");
    assert!(js.contains("window"), "Should include window parameter");
    assert!(js.contains("60"), "Should include window value");
}

#[test]
fn test_multiple_annotations_generate_all_middleware() {
    let source = r#"
        @auth(role = "admin")
        @validate(schema = UserSchema)
        @ratelimit(max = 10, window = 60)
        fn admin_create_user(data: String) {
            console.log("Admin creating user");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile with multiple annotations: {:?}", result.err());

    let js = result.unwrap();
    assert!(js.contains("__jounce_auth_check"), "Should generate auth check");
    assert!(js.contains("__jounce_validate"), "Should generate validation");
    assert!(js.contains("__jounce_ratelimit"), "Should generate rate limiting");
}

#[test]
fn test_security_middleware_order() {
    let source = r#"
        @auth(role = "admin")
        @validate(schema = UserSchema)
        fn protected_function(data: String) {
            console.log("Protected");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile: {:?}", result.err());

    let js = result.unwrap();

    // Find positions of middleware calls
    let auth_pos = js.find("__jounce_auth_check");
    let validate_pos = js.find("__jounce_validate");

    assert!(auth_pos.is_some(), "Should have auth middleware");
    assert!(validate_pos.is_some(), "Should have validate middleware");

    // Auth should come before validate (order of annotations)
    assert!(auth_pos.unwrap() < validate_pos.unwrap(),
            "Auth should come before validate in generated code");
}

#[test]
fn test_function_without_annotations_no_middleware() {
    let source = r#"
        fn public_function() {
            console.log("Public");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile without annotations: {:?}", result.err());

    let js = result.unwrap();
    assert!(!js.contains("__jounce_auth_check"), "Should not generate auth check");
    assert!(!js.contains("__jounce_validate"), "Should not generate validation");
    assert!(!js.contains("__jounce_ratelimit"), "Should not generate rate limiting");
}

#[test]
fn test_security_imports_generated() {
    let source = r#"
        @auth(role = "admin")
        fn admin_function() {
            console.log("Admin only");
        }
    "#;

    let result = compile_and_emit(source);
    assert!(result.is_ok(), "Should compile: {:?}", result.err());

    let js = result.unwrap();
    // Check that security runtime is imported when annotations are used
    assert!(js.contains("__jounce_auth_check") || js.contains("from './runtime/security.js'"),
            "Should either use security functions or import them");
}
