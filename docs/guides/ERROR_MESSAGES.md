# Enhanced Error Messages

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md).

**Quick Win 3: Better Error Messages** ✅ **COMPLETE**

Jounce now provides beautiful, helpful error messages with:

- ✅ **Colored output** with ANSI formatting
- ✅ **Source location** (file:line:column)
- ✅ **Code snippets** with context lines
- ✅ **Visual indicators** (^ pointing to errors)
- ✅ **Error codes** (E001, E002, etc.)
- ✅ **Helpful suggestions** with 💡 icons
- ✅ **Example code** showing correct usage
- ✅ **Smart pattern matching** for common errors

---

## Before vs After

### Before (Raw Debug Output)
```
❌ Parsing failed: ParserError { message: "Unexpected closing brace '}'. Did you have an extra brace, or forget an opening brace '{'?", line: 10, column: 2 }
```

### After (Beautiful Diagnostics)
```
❌ Parsing failed:

error: expected , found Unexpected closing brace '}'. Did you have an extra brace, or forget an opening brace '{'?
  --> test_parse_error.jnc:10:2
   9 |     return <div>Test</div>;
  10 | }
   |      ^
  [E004]

💡 Missing closing brace [E050]
   Every opening brace { must have a matching closing brace }

📝 Example:
   if (condition) {
       doSomething();
   }  // ← closing brace required
```

---

## Error Database

**20+ Common Errors Covered:**

### General Errors (E100-E109)
- E100: Invalid file extension (must be .jnc)

### Parser Errors (E001-E009)
- E001: Missing semicolon
- E002: Unexpected token
- E003: Expected identifier
- E004: Expected expression

### Component Errors (E010-E019)
- E010: Invalid component syntax
- E011: Invalid component name (must be uppercase)

### JSX Errors (E020-E029)
- E020: Unclosed JSX tag
- E021: Invalid JSX attribute
- E022: Invalid JSX expression

### Reactivity Errors (E030-E039)
- E030: Accessing signal without .value
- E031: Invalid effect syntax

### Type Errors (E040-E049)
- E040: Type mismatch
- E041: Undefined variable
- E430: Array index must be an integer

### Brace/Paren Errors (E050-E059)
- E050: Missing closing brace
- E051: Missing closing parenthesis
- E052: Unexpected closing brace

### Import/Export Errors (E060-E069)
- E060: Import not found
- E061: Invalid export syntax

### Style Errors (E070-E079)
- E070: Invalid style syntax
- E_STY_001: Unsupported or malformed nested style rule
- E_STY_002: Media query must appear at top level
- E_STY_003: Media query cannot be nested inside selector
- E_STY_004: Invalid @keyframes syntax (selector must be 'from', 'to', or '<number>%')
- E_STY_005: Style nesting depth exceeds maximum (max depth: 3)

---

## Implementation

**Files Modified:**
- `src/error_help.rs` - New error help database (350+ lines)
- `src/lib.rs` - Added error_help module + enhanced Compiler::display_error
- `src/main.rs` - Updated parse error handling to use display_error

**Key Features:**

1. **ErrorHelp Database** - Maps error patterns to helpful suggestions
2. **Smart Pattern Matching** - Analyzes error messages to find relevant help
3. **Formatted Output** - Combines diagnostics with helpful suggestions
4. **Zero Runtime Cost** - All formatting happens at compile time

---

## Usage

Error help is **automatically included** in all compilation errors. No configuration needed!

When you encounter an error:
1. Read the **source location** to find where it happened
2. Look at the **code snippet** to see context
3. Check the **suggestion** for how to fix it
4. Use the **example** as a reference

---

## Future Improvements

- Add more error patterns as users report issues
- Provide links to documentation for complex errors
- Add "Did you mean...?" suggestions for typos
- Support multi-language error messages

---

**Status**: ✅ Complete (Quick Win 3)
**Time**: ~2 hours
**Impact**: High - Better DX for all users

---

## Error Code Details

### E430: Array Index Must Be An Integer

**Error Message**:
```
error[E430]: Array index must be an integer, got '{type}'
help: Cast non-integer values to i32, or use range loops which automatically type variables as integers
```

**What it means**: Array and vector indexes must be integer-typed (i32). Float, string, boolean, or other non-integer types cannot be used as indexes.

**Common Causes**:
- Using a float as an array index (`items[1.5]`)
- Using a string as an array index (`items["0"]`)
- Using boolean values as indexes (`items[true]`)
- Division resulting in a float (`items[price / count]` where price and count are floats)

**Fix**:
- Use integer literals or variables: `items[0]`, `items[i]`
- Cast non-integer expressions to `i32` (when cast methods are available)
- Use range loops which automatically type the variable as integer: `for i in 0..len { ... }`

**Example - Before**:
```jounce
fn example() {
    let items = vec![1, 2, 3];
    let key = "0";  // String, not integer
    let x = items[key];  // ❌ Error: Array index must be an integer, got 'string'
}
```

**Example - After**:
```jounce
fn example() {
    let items = vec![1, 2, 3];

    // ✅ Use integer index
    let x = items[0];

    // ✅ Use range loop (i is automatically integer-typed)
    for i in 0..items.len() {
        let item = items[i];
    }
}
```

**Note**: Range loop variables are automatically considered integer-typed at compile time.

---

### E100: Invalid File Extension

**Error Message**:
```
error[E100]: Jounce files must use the .jnc extension
```

**What it means**: Emitted when trying to compile a file that doesn't have the `.jnc` extension.

**Common Causes**:
- Using .js, .ts, or other extensions
- Typo in filename
- Attempting to compile non-Jounce files

**Fix**:
- Rename your file to have the `.jnc` extension
- Example: `app.js` → `app.jnc`

**Example - Before**:
```bash
jnc compile main.js  # ❌ Error: Invalid file extension
```

**Example - After**:
```bash
jnc compile main.jnc  # ✅ Compiles successfully
```

---

**Maintained by: The Jounce Project**

