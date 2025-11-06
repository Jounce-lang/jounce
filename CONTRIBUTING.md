# Contributing to Jounce

First off, thank you for considering contributing to Jounce! 🎉

Jounce is an open-source, AI-native programming language, and we welcome contributions from everyone.

## 🤝 Code of Conduct

This project and everyone participating in it is governed by the [Jounce Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🎯 How Can I Contribute?

### Reporting Bugs

**Before submitting a bug report:**
- Check the [issue tracker](https://github.com/Jounce-lang/Jounce/issues) to see if it's already reported
- Collect information about the bug:
  - Stack trace
  - OS, Rust version, Node.js version
  - Input code that triggers the bug
  - Expected vs actual behavior

**Submit a bug report:**
1. Use the bug report template
2. Use a clear, descriptive title
3. Provide steps to reproduce
4. Include code examples
5. Explain expected behavior
6. Include screenshots if applicable

### Suggesting Features

We love feature suggestions! Please:
1. Use the feature request template
2. Explain the problem you're trying to solve
3. Describe the solution you'd like
4. Consider how it fits with Jounce's goals
5. Be open to discussion and iteration

### Contributing Code

**First-time contributors:**
- Look for issues labeled `good first issue`
- Ask questions in the issue before starting
- Start small to get familiar with the codebase

**Process:**
1. **Fork** the repository
2. **Clone** your fork locally
3. **Create a branch**: `git checkout -b feature/my-feature`
4. **Make changes**: Write code, tests, and docs
5. **Test**: Run `cargo test` and `./scripts/test_all_examples.sh`
6. **Commit**: Use clear commit messages
7. **Push**: Push to your fork
8. **Pull Request**: Open a PR with description

## 🛠️ Development Setup

### Prerequisites

- Rust 1.75+ (`rustup install stable`)
- Node.js 18+ (`node --version`)
- Git

### Building Locally

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/Jounce.git
cd Jounce

# Build the compiler
cargo build --release

# Run tests
cargo test --lib

# Test examples
./scripts/test_all_examples.sh

# Try compiling an example
cargo run --release -- compile examples/apps/01-click-counter/main.jnc
```

### Project Structure

```
Jounce/
├── src/              # Compiler source (Rust)
├── runtime/          # Client/server runtime (JavaScript)
├── examples/         # Example applications
├── templates/        # Starter templates
├── packages/         # Package ecosystem
├── tests/            # Integration tests
├── docs/             # Documentation
└── scripts/          # Build/test automation
```

**Key files:**
- `src/lib.rs` - Compiler entry point
- `src/parser.rs` - Parser (4000+ lines)
- `src/js_emitter.rs` - JavaScript code generation
- `src/lexer.rs` - Tokenization
- `runtime/reactivity.js` - Reactivity system

## 📝 Coding Guidelines

### Rust Code

**Style:**
- Run `cargo fmt` before committing
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable names
- Add comments for complex logic

**Testing:**
- Add tests for all new features
- Ensure all tests pass: `cargo test`
- Aim for 80%+ test coverage
- Include both unit and integration tests

**Runtime Testing (v0.8.2+):**
- Test runtime safety features in `runtime/reactivity.js`
- Verify frozen signal behavior with `Object.freeze()`
- Test dev-mode side effect detection in computed()
- Ensure production mode doesn't include dev checks
- Test that runtime errors provide helpful messages

**Example:**
```rust
// ✓ Good
fn parse_jsx_element(&mut self) -> Result<JsxElement, CompileError> {
    // Implementation
}

#[test]
fn test_parse_jsx_element() {
    let source = "<div>Hello</div>";
    let result = parse(source);
    assert!(result.is_ok());
}

// ✗ Bad
fn p(&mut self) -> Result<E, C> {  // Unclear names
    // No tests
}
```

### Jounce Code (.jnc files)

**Style:**
- Use 4-space indentation
- Follow component naming conventions (PascalCase)
- Use descriptive variable names

```jounce
// ✓ Good
component TodoList(items: Array<String>) {
    let count = signal(0);

    return <div class="todo-list">
        {items.value.map(item => <TodoItem text={item} />)}
    </div>;
}

// ✗ Bad
component tl(i) {  // Bad naming
let c=signal(0);   // No spacing
return <div>{i.value.map(x=><div>{x}</div>)}</div>;  // No formatting
}
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: Add JSX fragment support
fix: Resolve signal memory leak
docs: Update CONTRIBUTING.md
test: Add reactivity system tests
chore: Update dependencies
```

**Structure:**
```
<type>: <short description>

<optional body explaining the change>

<optional footer with issue references>
```

**Example:**
```
feat: Add error code E050 for missing braces

Added helpful error message with code snippet and suggestion
when users forget closing braces.

Closes #123
```

## 🧪 Testing

### Run All Tests
```bash
cargo test --lib
```

### Test Specific Module
```bash
cargo test parser
cargo test reactivity
```

### Test Examples
```bash
./scripts/test_all_examples.sh
```

### Add New Tests
```rust
// tests/integration_tests.rs
#[test]
fn test_my_feature() {
    let source = r#"
        component App() {
            // Test code
        }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile_source(source, BuildTarget::Client);

    assert!(result.is_ok());
}
```

## 📚 Documentation

**Update docs when you:**
- Add a new feature
- Change behavior
- Add a new API
- Fix a bug that might confuse users

**Documentation types:**
- Code comments (for complex logic)
- README updates (for new features)
- `docs/` guides (for major features)
- CHANGELOG.md (for all changes)

## 🔍 Review Process

**What we look for:**
- ✅ Tests pass (`cargo test`)
- ✅ Code is formatted (`cargo fmt`)
- ✅ Follows project conventions
- ✅ Includes tests for new code
- ✅ Documentation is updated
- ✅ Commit messages are clear
- ✅ No breaking changes (or clearly documented)

**Timeline:**
- Initial review: Within 2-3 days
- Follow-up: Based on complexity
- Merge: After approval + all checks pass

## 🚀 Release Process

**Versioning:**
- Follow [Semantic Versioning](https://semver.org/)
- Breaking changes = MAJOR (v1.0.0 → v2.0.0)
- New features = MINOR (v0.4.0 → v0.5.0)
- Bug fixes = PATCH (v0.4.0 → v0.4.1)

**Before release:**
1. All tests pass
2. CHANGELOG.md updated
3. Version bumped in `Cargo.toml`
4. Git tag created
5. Release notes written

## 💬 Getting Help

**Questions?**
- Check [Documentation](./docs/)
- Search [Issues](https://github.com/Jounce-lang/Jounce/issues)
- Ask in [Discussions](https://github.com/Jounce-lang/Jounce/discussions)
- Join our [Discord](https://discord.gg/jounce) (coming soon)

**Found a security issue?**
- See [SECURITY.md](SECURITY.md) for responsible disclosure

## 📋 Pull Request Checklist

Before submitting, ensure:

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if applicable)
- [ ] Commit messages follow conventions
- [ ] Branch is up to date with `main`
- [ ] PR description explains the change
- [ ] Links to related issue (if applicable)

## 🎉 Recognition

Contributors are recognized in:
- CHANGELOG.md (for significant contributions)
- README.md contributors section
- Release notes

## 📄 License

By contributing, you agree that your contributions will be licensed under the same license as the project (see [LICENSE](LICENSE)).

---

**Thank you for contributing to Jounce!** 🚀

Every contribution, no matter how small, helps make Jounce better for everyone.
