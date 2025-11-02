// Test security middleware generation in JavaScript emitter (Phase 17)

use jounce_compiler::lexer::Lexer;
use jounce_compiler::parser::Parser;
use jounce_compiler::js_emitter::JSEmitter;

#[test]
fn test_auth_middleware_generation() {
    let source = r#"
        @auth(role = "admin")
        fn delete_user(id: i64) {
            console.log("Deleting user", id);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include auth check
    assert!(js.contains("__jounce_auth_check"), "Generated JS should contain auth check");
    assert!(js.contains("role: \"admin\""), "Generated JS should contain role requirement");

    // Should throw error on unauthorized
    assert!(js.contains("throw new Error"), "Generated JS should throw error on auth failure");
}

#[test]
fn test_validate_middleware_generation() {
    let source = r#"
        @validate(schema = UserSchema)
        fn create_user(data: UserData) {
            db.insert("users", data);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include validation check
    assert!(js.contains("__jounce_validate"), "Generated JS should contain validation check");
    assert!(js.contains("UserSchema"), "Generated JS should reference UserSchema");
}

#[test]
fn test_ratelimit_middleware_generation() {
    let source = r#"
        @ratelimit(max = 100, window = 60000)
        fn search_products(query: String) {
            return db.search("products", query);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include rate limit check
    assert!(js.contains("__jounce_ratelimit"), "Generated JS should contain rate limit check");
    assert!(js.contains("max: 100"), "Generated JS should contain max limit");
    assert!(js.contains("window: 60000"), "Generated JS should contain time window");
}

#[test]
fn test_sanitize_middleware_generation() {
    let source = r#"
        @sanitize(fields = ["content", "comment"])
        fn post_comment(content: String, comment: String) {
            db.insert("comments", { content, comment });
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include sanitization
    assert!(js.contains("__jounce_sanitize"), "Generated JS should contain sanitization");
}

#[test]
fn test_secure_middleware_generation() {
    let source = r#"
        @secure
        fn process_payment(amount: f64) {
            payment_gateway.charge(amount);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include HTTPS check
    assert!(js.contains("__jounce_require_https") || js.contains("protocol"),
            "Generated JS should contain HTTPS enforcement");
}

#[test]
fn test_multiple_annotations_middleware_generation() {
    let source = r#"
        @secure
        @auth(role = "admin")
        @validate(schema = UserSchema)
        @ratelimit(max = 10, window = 60000)
        fn admin_create_user(data: UserData) {
            db.insert("users", data);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include all middleware checks
    assert!(js.contains("__jounce_require_https") || js.contains("protocol"),
            "Should contain HTTPS check");
    assert!(js.contains("__jounce_auth_check"),
            "Should contain auth check");
    assert!(js.contains("__jounce_validate"),
            "Should contain validation");
    assert!(js.contains("__jounce_ratelimit"),
            "Should contain rate limiting");

    // Verify order - security checks should come before function body
    let https_pos = js.find("protocol").or_else(|| js.find("__jounce_require_https"));
    let auth_pos = js.find("__jounce_auth_check");
    let validate_pos = js.find("__jounce_validate");
    let ratelimit_pos = js.find("__jounce_ratelimit");
    let body_pos = js.find("db.insert");

    if let (Some(body), Some(_https), Some(_auth), Some(_validate), Some(_rate)) =
        (body_pos, https_pos, auth_pos, validate_pos, ratelimit_pos) {
        // All middleware should come before the function body
        assert!(https_pos.unwrap() < body, "HTTPS check should come before function body");
        assert!(auth_pos.unwrap() < body, "Auth check should come before function body");
        assert!(validate_pos.unwrap() < body, "Validation should come before function body");
        assert!(ratelimit_pos.unwrap() < body, "Rate limiting should come before function body");
    }
}

#[test]
fn test_function_without_annotations_no_middleware() {
    let source = r#"
        fn regular_function() {
            console.log("No security");
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should NOT include any security middleware
    assert!(!js.contains("__jounce_auth_check"), "Should not contain auth check");
    assert!(!js.contains("__jounce_validate"), "Should not contain validation");
    assert!(!js.contains("__jounce_ratelimit"), "Should not contain rate limiting");
    assert!(!js.contains("__jounce_sanitize"), "Should not contain sanitization");
}

#[test]
fn test_auth_with_permission_middleware() {
    let source = r#"
        @auth(permission = "users:delete")
        fn delete_user(id: i64) {
            db.delete("users", id);
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include permission check
    assert!(js.contains("__jounce_auth_check"), "Should contain auth check");
    assert!(js.contains("permission: \"users:delete\""), "Should contain permission requirement");
}

#[test]
fn test_auth_with_roles_array_middleware() {
    let source = r#"
        @auth(roles = ["admin", "moderator"])
        fn ban_user(id: i64) {
            db.update("users", id, { banned: true });
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include auth check with array of roles
    assert!(js.contains("__jounce_auth_check"), "Should contain auth check");
    assert!(js.contains("roles:"), "Should contain roles field");
    assert!(js.contains("\"admin\""), "Should contain admin role");
    assert!(js.contains("\"moderator\""), "Should contain moderator role");
}

#[test]
fn test_server_function_with_annotations() {
    let source = r#"
        @auth(role = "admin")
        @server
        fn server_admin_function() {
            return "admin data";
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should include auth middleware
    assert!(js.contains("__jounce_auth_check"), "Should contain auth check");

    // Should be an exported function (server functions are exported)
    assert!(js.contains("export"), "Should be exported");
}

#[test]
fn test_security_imports_generated() {
    let source = r#"
        @auth(role = "admin")
        fn protected_function() {
            console.log("Protected");
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should require security runtime (server.js uses CommonJS)
    assert!(js.contains("require"), "Should have require statements");
    assert!(js.contains("__jounce_auth_check"), "Should require auth check function");
    assert!(js.contains("./runtime/security.js"), "Should require from security runtime");
}

#[test]
fn test_no_security_imports_without_annotations() {
    let source = r#"
        fn regular_function() {
            console.log("Regular");
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(&mut lexer, source);
    let ast = parser.parse_program().expect("Failed to parse");

    let emitter = JSEmitter::new(&ast);
    let js = emitter.generate_server_js();

    // Should NOT import security runtime if no annotations used
    assert!(!js.contains("./runtime/security.js") || !js.contains("__jounce_auth_check"),
            "Should not import security runtime when no annotations are used");
}
