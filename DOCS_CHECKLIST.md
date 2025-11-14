# Jounce Documentation Checklist

> **Canonical Reference**: If this document conflicts with JOUNCE_SPEC.md, the spec wins. Current spec: v0.8.3 (2025-11-07).

**Purpose**: Ensure all documentation and examples stay aligned with JOUNCE_SPEC.md
**Last Updated**: 2025-11-08
**Spec Version**: v0.8.3 (2025-11-07)

---

## 10 Canonical Style Rules

These rules are enforced by JOUNCE_SPEC.md and **MUST** be followed in all code, docs, and examples:

### 1. `.jnc` Extension Required
All Jounce source files MUST use the `.jnc` file extension.
```jounce
// ✅ Correct
main.jnc

// ❌ Wrong
main.js, main.ts, main.jsx
```

---

### 2. Components Must Use Explicit `return`
Components MUST include explicit `return <JSX>;` statements.
```jounce
// ✅ Correct
component App() {
    let count = signal(0);
    return <div>{count.value}</div>;
}

// ❌ Wrong (implicit return)
component App() {
    let count = signal(0);
    <div>{count.value}</div>  // Missing return
}
```

---

### 3. DOM Events Must Be Lowercase
JSX event handler attributes MUST use lowercase DOM names (onclick, oninput, onchange).

**Important**: Prop names CAN be camelCase, but JSX attributes MUST be lowercase.

```jounce
// ✅ Correct (JSX attribute)
<button onclick={handleClick}>Click</button>

// ✅ Correct (prop name can be camelCase)
component Button(props: { onClick: Function }) {
    <button onclick={props.onClick}>Click</button>
}

// ❌ Wrong (JSX attribute must be lowercase)
<button onClick={handleClick}>Click</button>
<input onKeyPress={handler} />  // Use onkeypress
```

---

### 4. Use `signal()` Not `createSignal()`
Reactivity API uses Jounce-specific names, not React-like names.
```jounce
// ✅ Correct
let count = signal(0);
let doubled = computed(() => count.value * 2);
effect(() => console.log(count.value));
batch(() => { count.value = 10; });

// ❌ Wrong (React-like APIs)
let count = createSignal(0);
let doubled = createComputed(() => count.value * 2);
createEffect(() => console.log(count.value));
```

---

### 5. Prefix `await` Only
Async/await uses JavaScript-style prefix notation, not Rust-style postfix.
```jounce
// ✅ Correct
let result = await fetchData();

// ❌ Wrong (Rust-style postfix)
let result = fetchData().await;
```

**Note**: Prefer `@server` functions over async/await where possible.

---

### 6. Maximum 1-Level Style Nesting
Style blocks support max 1 level of nesting (pseudo-classes, pseudo-elements, child selectors).
```jounce
// ✅ Correct
style {
    .button {
        color: blue;

        &:hover {
            color: red;
        }
    }
}

// ❌ Wrong (2+ levels triggers E_STY_001)
style {
    .button {
        &:hover {
            &::after {  // Too deep!
                content: "...";
            }
        }
    }
}
```

---

### 7. `@server` and RPC Are Implemented
`@server` functions are **fully implemented** (v0.1.0), not planned.
```jounce
// ✅ Correct status in docs
**Implemented (v0.1.0)**:
- @server functions with automatic RPC generation

// ❌ Wrong status in docs
**Planned (v0.9.0+)**:
- @server functions  // This is FALSE
```

---

### 8. No JavaScript `for` Loops
Use Rust-style iteration, not JavaScript `for` loops.
```jounce
// ✅ Correct (Rust-style)
for i in 0..10 {
    console.log(i);
}

for item in items {
    console.log(item);
}

items.map(|item| <div>{item}</div>)

// ❌ Wrong (JS-style)
for (let i = 0; i < 10; i++) {
    console.log(i);
}
```

---

### 9. Explicit Error Codes
Style and parsing errors include domain-specific error codes.
```
✅ E_STY_001: Style nesting too deep
✅ E_STY_002: Invalid CSS selector
```

---

### 10. JOUNCE_SPEC.md Always Wins
**JOUNCE_SPEC.md** is the single source of truth. If any documentation or code conflicts with the spec, the spec wins.

```markdown
// ✅ Add this header to all new docs:
> **Canonical Reference**: If this document conflicts with JOUNCE_SPEC.md, the spec wins. Current spec: v0.8.3 (2025-11-07).
```

---

## Before Submitting Documentation PRs

### ✅ Canonical Reference
- [ ] Add canonical reference header to new docs
- [ ] Reference current spec version (v0.8.3)
- [ ] State that spec supersedes this document

### ✅ Code Examples
- [ ] All event handlers use lowercase (onclick, oninput, onchange)
- [ ] Use `signal()` not `createSignal()`
- [ ] Use `computed()` not `createComputed()`
- [ ] Components use explicit `return <...>;`
- [ ] Loops use Rust-style syntax (for i in 0..10)
- [ ] No async/await syntax (use @server functions instead)
- [ ] Style blocks show max 1-level nesting

### ✅ Feature Status
- [ ] @server functions marked as **Implemented v0.1.0** (not planned)
- [ ] signal/computed/effect/batch marked as **Implemented**
- [ ] Check JOUNCE_SPEC.md for current status of all features
- [ ] Mark new features as "Planned" with version number

### ✅ Version Consistency
- [ ] Update version to v0.8.3 (or current)
- [ ] Update date to current date
- [ ] Test passing status: "580/580 tests passing" (or current)

---

## Before Submitting Example PRs

### ✅ File Requirements
- [ ] File has `.jnc` extension
- [ ] File compiles successfully: `cargo run --release -- compile yourfile.jnc`
- [ ] Run verification script: `./scripts/verify-examples.sh`

### ✅ Syntax Requirements
- [ ] Event handlers: onclick, oninput, onchange (lowercase)
- [ ] Reactivity: signal(), computed(), effect() (not createSignal)
- [ ] Components: explicit return statements
- [ ] Loops: Rust-style (for i in 0..10 or items.map)
- [ ] Style blocks: Max 1-level nesting

### ✅ Template Files (Extra Critical)
- [ ] Templates must be perfect (users copy them)
- [ ] Test with `jnc init` if applicable
- [ ] Double-check all syntax (use checklist above)

---

## Automated Checks

### Run Before Push
```bash
# Verify all examples compile
./scripts/verify-examples.sh

# Run full test suite
cargo test --lib

# Check for syntax violations
grep -rn "onClick\|createSignal\|createComputed" \
  templates/ examples/apps/ tutorials/lessons/ --include="*.jnc"
```

### Expected Results
- ✅ verify-examples.sh: 0 failures
- ✅ cargo test: 580/580 passing (or current count)
- ✅ grep checks: 0 matches

---

## Common Mistakes to Avoid

### ❌ Wrong: CamelCase Events (in JSX attributes)
```jounce
<button onClick={handler}>  // ❌ Wrong
```
### ✅ Correct: Lowercase Events (in JSX attributes)
```jounce
<button onclick={handler}>  // ✅ Correct
```
**Note**: Props can be camelCase (onClick: Function), only JSX attributes must be lowercase.

### ❌ Wrong: createSignal
```jounce
let count = createSignal(0);  // ❌ Wrong (React-like)
```
### ✅ Correct: signal
```jounce
let count = signal(0);  // ✅ Correct (Jounce)
```

### ❌ Wrong: createComputed
```jounce
let doubled = createComputed(() => count.value * 2);  // ❌ Wrong
```
### ✅ Correct: computed
```jounce
let doubled = computed(() => count.value * 2);  // ✅ Correct
```

### ❌ Wrong: JS For Loops
```jounce
for (let i = 0; i < 10; i++) { }  // ❌ Wrong
```
### ✅ Correct: Rust For Loops
```jounce
for i in 0..10 { }  // ✅ Correct
```

### ❌ Wrong: @server as Planned
```markdown
**Planned (v0.9.0+)**:
- @server functions  // ❌ Wrong
```
### ✅ Correct: @server as Implemented
```markdown
**Implemented (v0.1.0+)**:
- @server functions  // ✅ Correct
```

---

## Reference Documents

- **JOUNCE_SPEC.md** - Canonical language specification (THE BOSS)
- **docs/archive/consistency-pass-*-*.md** - Detailed verification reports
- **scripts/verify-examples.sh** - Automated example verification
- **CONTRIBUTING.md** - General contribution guidelines

---

## Questions?

If unsure about syntax or feature status:
1. Check **JOUNCE_SPEC.md** first (it's authoritative)
2. Look at **templates/tutorial-starters/** for examples
3. Run **./scripts/verify-examples.sh** to test
4. Ask in GitHub Discussions

**Remember**: When in doubt, JOUNCE_SPEC.md wins! 🎯
