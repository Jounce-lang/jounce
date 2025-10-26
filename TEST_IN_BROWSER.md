# Testing Jounce Apps in Browser

## Color Picker App Test

### 1. Start HTTP Server

```bash
cd dist
python3 -m http.server 8080
```

Or on macOS/Linux:
```bash
cd dist && python -m SimpleHTTPServer 8080
```

### 2. Open in Browser

Navigate to: `http://localhost:8080/test_color_picker.html`

### 3. What You Should See

**Visual:**
- Purple gradient background
- White container with rounded corners
- "🎨 Reactive Color Picker" title
- Color preview box (initially purple: rgb(128, 64, 192))
- Hex color display: #8040c0
- Three sliders labeled Red (128), Green (64), Blue (192)
- Instructions at bottom

**Functionality:**
- ✅ Move Red slider → Preview color changes, hex updates, value updates
- ✅ Move Green slider → Same reactive behavior
- ✅ Move Blue slider → Same reactive behavior
- ✅ All updates happen in real-time without page reload

**Console Output:**
```
Jounce client initialized
✅ Reactive color picker initialized!
```

### 4. Verify CSS

Open DevTools → Elements → inspect `.container`

Should see:
```css
.container {
  max-width: 600px;  /* ✅ NOT "600 px" */
  margin: 0 auto;    /* ✅ Space between values */
  padding: 20px;     /* ✅ NOT "20 px" */
  ...
}
```

### 5. Verify Reactivity

1. Open DevTools → Console
2. Type: `red.value = 255`
3. Should see: Preview turns more red, hex updates to #ff40c0, slider moves to 255

### 6. Check for Errors

**Console should have NO errors**

Common issues:
- ❌ "signal is not defined" → reactivity.js not loaded
- ❌ "mountComponent is not defined" → client-runtime.js not loaded
- ❌ "ColorPicker is not a function" → client.js not loaded correctly

### Success Criteria

✅ Page loads without errors
✅ Sliders are interactive
✅ Color preview updates when sliders move
✅ Hex value updates automatically
✅ RGB values display correctly
✅ CSS is properly formatted (no "600 px" spacing issues)
✅ Console shows initialization message

## Next: Test Todo App

After color picker works, test the todo app:

```bash
# Compile todo app
cd ..
cargo run -- compile test_interactive_todo.jnc

# Start server
cd dist
python3 -m http.server 8080

# Open: http://localhost:8080/index.html
```

### Todo App Success Criteria

✅ Add task with Enter key
✅ Add task with button click
✅ Toggle task completion (checkbox)
✅ Delete individual tasks
✅ Clear completed tasks
✅ Task count updates automatically
✅ Empty state shows when no tasks

---

**Note**: If you encounter issues, check:
1. All runtime files copied (client-runtime.js, reactivity.js)
2. Correct import paths in client.js
3. No CORS errors (use http-server, not file://)
4. Browser console for JavaScript errors
