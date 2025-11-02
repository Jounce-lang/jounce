# 🧪 Jounce Testing Guide

Complete guide for testing the Jounce compiler and reactivity examples.

---

## 🚀 Quick Start (30 seconds)

```bash
# Run the automated test script
./test_all_examples.sh

# Expected output:
# 🎉 All tests passed! Jounce is working perfectly!
```

**That's it!** If the script passes, everything works.

---

## 📋 Manual Testing (Step-by-Step)

### **Prerequisites**

Make sure you have:
- **Rust** installed: `rustc --version` (should show 1.70+)
- **Node.js** installed: `node --version` (should show 18+)
- **Terminal** open in the Jounce directory

### **Step 1: Build the Compiler**

```bash
cargo build --release
```

**Expected output**:
```
   Compiling jounce v0.4.0
    Finished `release` profile [optimized] target(s) in X.XXs
```

✅ **Success**: Compiler built without errors

---

### **Step 2: Run All Tests**

```bash
cargo test --lib
```

**Expected output**:
```
test result: ok. 635 passed; 0 failed; 10 ignored; 0 measured
```

✅ **Success**: 635/635 tests passing

---

### **Step 3: Test Your First App (Counter)**

**Compile**:
```bash
cargo run --release -- compile test_fine_grained_reactivity.jnc
```

**Expected output**:
```
🔥 Compiling full-stack application: test_fine_grained_reactivity.jnc
   📦 Output: server.js + client.js + app.wasm
   ...
✨ Compilation complete! (X.XXms)
```

**Run**:
```bash
cd dist
node server.js
```

**Expected output**:
```
🚀 Jounce server running at http://localhost:3000
Press Ctrl+C to stop
```

**Test in Browser**:
1. Open `http://localhost:3000`
2. You should see: **"Counter: 0"** and **"Double: 0"**
3. Click **"Increment"** button
4. Counter should change to 1, 2, 3...
5. Double should change to 2, 4, 6...

✅ **Success**: If numbers update when you click, reactivity works!

**Stop the server**: Press `Ctrl+C` in terminal

---

### **Step 4: Test Shopping Cart**

```bash
# Go back to main directory
cd ..

# Compile
cargo run --release -- compile examples/reactivity/shopping-cart.jnc

# Run
cd dist && node server.js
```

**In browser** (`http://localhost:3000`):
- Check: Should see 3 products (Laptop, Mouse, Keyboard)
- Check: Total price should be visible
- Check: Item count should show total quantity

✅ **Success**: All computed values display correctly

Press `Ctrl+C` to stop.

---

### **Step 5: Test Form Validation**

```bash
cd ..
cargo run --release -- compile examples/reactivity/form-validation-simple.jnc
cd dist && node server.js
```

**In browser**:
1. **Type in email field** - watch "Current value:" update as you type
2. **Type in password** - watch "Length: X characters" update
3. **Type in confirm password** - watch for "✅ Passwords match!"

✅ **Success**: Text updates AS YOU TYPE (no need to press Enter)

Press `Ctrl+C` to stop.

---

### **Step 6: Test Search & Filter**

```bash
cd ..
cargo run --release -- compile examples/reactivity/search-filter.jnc
cd dist && node server.js
```

**In browser**:
1. Type "laptop" in search → Results filter instantly
2. Change category dropdown → Results update
3. Move price slider → "Max Price" updates
4. Check "Show in-stock only" → Results filter

✅ **Success**: Results update instantly without clicking "Search"

Press `Ctrl+C` to stop.

---

### **Step 7: Test Dashboard**

```bash
cd ..
cargo run --release -- compile examples/reactivity/dashboard.jnc
cd dist && node server.js
```

**In browser**:
- Check the 4 stat cards display values
- Check "Total Revenue" shows a dollar amount
- Check "Average Sale" is calculated correctly
- Check "Quick Stats" section shows computed metrics

✅ **Success**: All metrics display and are calculated

Press `Ctrl+C` to stop.

---

### **Step 8: Test Theme Switcher**

```bash
cd ..
cargo run --release -- compile examples/reactivity/theme-switcher.jnc
cd dist && node server.js
```

**In browser**:
1. Click "☀️ Light" button
2. Click "🌙 Dark" button
3. Click "🎨 Auto" button
4. Check "Theme Info" section updates
5. **Refresh the page (F5)**

✅ **Success**: Theme persists after page reload (localStorage works!)

Press `Ctrl+C` to stop.

---

### **Step 9: Test Todo App (Full-Stack)**

```bash
cd ..
cargo run --release -- compile examples/apps/todo-app/main_reactive.jnc
cd dist && node server.js
```

**In browser**:
1. Should see 3 sample todos
2. Should see "Total: 3 todos"
3. Type "New task" in input
4. Click "Add" button
5. Check: New todo appears
6. Check: Count updates to "Total: 4 todos"

✅ **Success**: Count updates automatically (no manual DOM code!)

Press `Ctrl+C` to stop.

---

## 🔬 Verify Reactive Code Generation

Let's confirm the compiler generates reactive wrappers:

```bash
# Compile the counter
cargo run --release -- compile test_fine_grained_reactivity.jnc

# Count reactive wrappers
grep -o "__reactive" dist/client.js | wc -l
```

**Expected**: Should show **2** (one for count.value, one for count.value * 2)

**View the actual code**:
```bash
grep -A 10 "function Counter" dist/client.js
```

**You should see**:
```javascript
(() => {
  const __reactive = signal("");
  effect(() => {
    __reactive.value = count.value;
  });
  return __reactive;
})()
```

✅ **Success**: If you see `__reactive` and `effect()`, fine-grained reactivity is working!

---

## 📊 Performance Testing

### **Test: Compilation Speed**

```bash
time cargo run --release -- compile examples/reactivity/dashboard.jnc
```

**Expected**: Should complete in **under 1 second** (actual compilation ~10-15ms)

### **Test: Bundle Sizes**

```bash
du -h dist/*.js
```

**Expected output**:
- `client.js`: ~30-50KB
- `server.js`: ~20-30KB
- `reactivity.js`: ~6KB

✅ **Success**: All bundles are reasonably sized

---

## 🐛 Troubleshooting

### **Problem: "cargo: command not found"**

**Solution**: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

### **Problem: "Port 3000 already in use"**

**Solution 1**: Kill existing process
```bash
lsof -i :3000  # Find the process
kill -9 <PID>  # Kill it
```

**Solution 2**: Use a different port
```bash
# Edit dist/server-runtime.js
# Change: const PORT = 3000;
# To:     const PORT = 3001;
```

---

### **Problem: "Cannot find module 'better-sqlite3'"**

**Solution**: Install dependencies
```bash
cd dist
npm install better-sqlite3
cd ..
```

---

### **Problem: Compilation errors**

**Check syntax**:
```bash
cargo run --release -- compile <file.jnc>
```

Look for error messages like:
- "Unexpected token" → Syntax error in your .jnc file
- "Expected bool, got any" → Type error
- "ParserError" → Parser can't understand the code

**Common fixes**:
- Check for missing semicolons
- Check for mismatched braces `{}`
- Check for reserved keywords (don't use `theme` as parameter name)

---

### **Problem: Reactivity not working**

**Debug steps**:

1. **Check if reactive wrappers are generated**:
```bash
grep "__reactive" dist/client.js
```
If nothing shows, the compiler didn't detect reactive expressions.

2. **Check browser console** (F12):
- Look for JavaScript errors
- Check if signals are defined

3. **Verify you're using `.value`**:
```jounce
// ❌ Wrong
{count}

// ✅ Correct
{count.value}
```

---

## ✅ Success Checklist

Run through this to confirm everything works:

- [ ] `cargo build --release` succeeds
- [ ] `cargo test --lib` shows 635/635 passing
- [ ] `./test_all_examples.sh` passes all tests
- [ ] Counter example compiles and runs
- [ ] Clicking "Increment" updates the number
- [ ] `grep "__reactive" dist/client.js` shows results
- [ ] All 6 examples compile successfully
- [ ] Form validation updates as you type
- [ ] Search filters work instantly
- [ ] Theme switcher persists after reload
- [ ] Todo count updates automatically

**If all ✅, Jounce is working perfectly!**

---

## 🎯 What to Test For

### **1. Automatic DOM Updates**

When you change a signal value:
```jounce
count.value = count.value + 1;
```

The UI should update **without**:
- Calling any update functions
- Refreshing the page
- Manual DOM manipulation

### **2. Real-Time Reactivity**

When you type in an input:
```jounce
<input oninput={(e) => { email.value = e.target.value; }} />
<p>You typed: {email.value}</p>
```

The text should update **as you type**, character by character.

### **3. Computed Values**

When a dependency changes:
```jounce
let total = computed(() => items.value.reduce(...));
```

The computed value should **recalculate automatically**.

### **4. Persistence**

With persistentSignal:
```jounce
let theme = persistentSignal("theme", "light");
```

The value should **survive page reload** (stored in localStorage).

---

## 🚀 Quick Commands Reference

```bash
# Build compiler
cargo build --release

# Run tests
cargo test --lib

# Test all examples (automated)
./test_all_examples.sh

# Compile an app
cargo run --release -- compile <file.jnc>

# Run the app
cd dist && node server.js

# Check generated code
grep "__reactive" dist/client.js

# Stop server
Ctrl+C
```

---

## 📚 Next Steps

After confirming everything works:

1. **Build your own app** - Use the examples as templates
2. **Experiment** - Try modifying the examples
3. **Read the docs** - Check `examples/reactivity/README.md`
4. **Share feedback** - Report any issues you find

---

## 🎉 You're Ready!

If you've completed all the tests above, you now know:
- ✅ How to compile Jounce apps
- ✅ How to run the development server
- ✅ How to verify reactivity works
- ✅ How to debug common issues
- ✅ How to test all examples

**Happy coding with Jounce!** 🚀
