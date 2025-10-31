# Jounce Tests

**Comprehensive test suite for the Jounce compiler**

---

## 📊 Test Coverage

**Status**: ✅ 635/635 tests passing (100%)

---

## 📂 Test Organization

### Integration Tests (tests/)
End-to-end compilation tests:
- **`integration/`** - Full compilation pipeline tests
- **`semantic/`** - Semantic analysis tests
- **`basic_tests.jnc`** - Basic language features
- **`example.test.jnc`** - Example test template

### Operator Tests (operators/)
Modern JavaScript operator tests:
- **`test_all_new_operators.jnc`** - All new operators
- **`test_logical_assign.jnc`** - `&&=`, `||=`, `??=`
- **`test_nullish_simple.jnc`** - `??` operator
- **`test_optional_chain.jnc`** - `?.` operator
- **`test_new_operators.jnc`** - Combined tests

### Module Tests (modules/)
Package and import tests:
- **`math/`** - Math package tests
- **`math.jnc`** - Math utilities test
- **`raven-router/`** - Router package tests

### Legacy Tests (legacy/)
Historical test files:
- **`legacy/`** - Original test files
- **`jsx/`** - JSX-specific tests
- **`features/`** - Feature tests

### Fixtures (fixtures/)
Test data and projects:
- **`test-project/`** - Sample project structure

### Package Tests (Individual packages)
- `test_crypto.jnc` - Cryptography
- `test_datetime.jnc` - Date/time
- `test_fs.jnc` - File system
- `test_json_parser.jnc` - JSON parsing
- `test_module_system.jnc` - Module system
- `test_yaml.jnc` - YAML parsing

---

## 🚀 Running Tests

### Run All Tests
```bash
cargo test --lib
```

### Run Specific Test Suite
```bash
# Integration tests
cargo test --test integration

# Operator tests
cargo test operators

# Module tests
cargo test modules
```

### Run Single Test
```bash
cargo test test_signal_basic
```

### Run With Output
```bash
cargo test -- --nocapture
```

### Run in Release Mode
```bash
cargo test --release
```

---

## 📝 Writing Tests

### Integration Test Template
```rust
// tests/integration_tests.rs
#[cfg(test)]
mod my_feature_tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        let source = r#"
            component App() {
                let count = signal(0);
                return <div>{count.value}</div>;
            }
        "#;

        let compiler = Compiler::new();
        let result = compiler.compile_source(source, BuildTarget::Client);

        assert!(result.is_ok());
        let wasm = result.unwrap();
        assert!(wasm.len() > 0);
    }
}
```

### .jnc Test File
```jounce
// tests/my_feature_test.jnc
component TestApp() {
    let value = signal(42);

    return <div class="test">
        <h1>Test: {value.value}</h1>
    </div>;
}

// Compile this file to test
```

### Test Naming
- Use descriptive names: `test_signal_reactivity`
- Not: `test1`, `test_a`, `mytest`
- Group related tests in modules

---

## 🎯 Test Categories

### Unit Tests (src/)
**Location**: In source files with `#[cfg(test)]`
```rust
// src/my_module.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_feature() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests (tests/)
**Location**: In `tests/` directory
- Test full compilation pipeline
- Test end-to-end workflows
- Test file I/O and caching

### Example Tests
**Location**: Compiled examples
```bash
./scripts/test_all_examples.sh
```

---

## 📊 Test Statistics

**Total Tests**: 635
**Pass Rate**: 100%
**Coverage Areas**:
- ✅ Lexing & Parsing
- ✅ Semantic Analysis
- ✅ Type Checking
- ✅ Code Generation
- ✅ Optimization
- ✅ Reactivity
- ✅ CSS Generation
- ✅ Module System
- ✅ Error Handling

---

## 🐛 Debugging Tests

### Run with backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

### Run single test with output
```bash
cargo test test_signal_basic -- --nocapture
```

### Run ignored tests
```bash
cargo test -- --ignored
```

### Show test times
```bash
cargo test -- --show-output
```

---

## 📝 Test Guidelines

**Good Tests:**
- ✅ Test one thing
- ✅ Have descriptive names
- ✅ Are independent
- ✅ Run quickly
- ✅ Are deterministic

**Bad Tests:**
- ❌ Test multiple things
- ❌ Have vague names
- ❌ Depend on other tests
- ❌ Are slow
- ❌ Are flaky

**Example:**
```rust
// ✓ Good
#[test]
fn test_signal_updates_trigger_effects() {
    // Clear test of one behavior
}

// ✗ Bad
#[test]
fn test_stuff() {
    // Vague name, unclear what it tests
}
```

---

## 🔧 Test Utilities

### Test Helpers
```rust
// Create test compiler
fn test_compiler() -> Compiler {
    Compiler::new()
}

// Compile test source
fn compile_test(source: &str) -> Result<Vec<u8>, CompileError> {
    test_compiler().compile_source(source, BuildTarget::Client)
}
```

### Common Assertions
```rust
// Success
assert!(result.is_ok());

// Specific value
assert_eq!(count.value, 42);

// Contains
assert!(output.contains("signal"));

// Not null
assert!(!wasm.is_empty());
```

---

## 📈 Continuous Integration

Tests run automatically on:
- Every commit
- Every pull request
- Before release

**Required**: All tests must pass before merging.

---

## 🔗 Resources

- [Testing Guide](../TESTING_GUIDE.md)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Test Examples](./integration/)

---

**Last Updated**: October 31, 2025 (v0.8.1)
**Total Tests**: 635
**Pass Rate**: 100%
