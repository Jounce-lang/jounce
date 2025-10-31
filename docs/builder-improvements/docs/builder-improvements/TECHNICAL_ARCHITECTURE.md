# Technical Architecture - Builder Improvements

High-level system design for all phases.

---

## System Overview

```
┌──────────────────────────────────────────────────┐
│  User Interface Layer                            │
│  - VSCode / Web Editor                           │
│  - Visual Builder (Phase 3)                      │
│  - CLI                                           │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────────────────┐
│  Development Tools Layer                         │
│  - File Watcher (Phase 2)                        │
│  - Hot Reload Server (Phase 2)                   │
│  - Error Formatter                               │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────────────────┐
│  Jounce Compiler (Rust)                          │
│  - Lexer → Parser → AST                          │
│  - Type Checker                                  │
│  - Code Generator (JS + CSS + WASM)             │
│  - Incremental Compilation Cache                 │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────────────────┐
│  Component Library (Phase 1)                     │
│  - 50+ Pre-built Components                      │
│  - Templates                                     │
│  - Utility CSS                                   │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────────────────┐
│  Runtime Layer                                   │
│  - Reactivity System (signals)                   │
│  - Virtual DOM (h function)                      │
│  - Component Lifecycle                           │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────────────────┐
│  Output                                          │
│  - client.js (application code)                  │
│  - client-runtime.js (framework)                 │
│  - reactivity.js (signals)                       │
│  - styles.css                                    │
│  - app.wasm (optional)                           │
└──────────────────────────────────────────────────┘
```

---

## Key Design Decisions

### 1. Single Source of Truth
- `.jnc` file is always the source
- Visual builder generates `.jnc` code
- No "locked" visual-only mode
- Git-friendly (text diffs work)

### 2. Incremental Compilation
- Cache compiled modules by content hash
- Only recompile changed files
- Target: < 500ms for most changes

### 3. Component Registry
- JSON manifest of available components
- Versioned (semver)
- Local + remote sources

### 4. Hot Reload Protocol
- WebSocket for browser communication
- Debounced file watching (100ms)
- Graceful error recovery

---

## Data Structures

### Component Tree (JSON):
```json
{
  "id": "comp-1",
  "type": "button",
  "props": {
    "variant": "primary",
    "onClick": "handleClick"
  },
  "children": ["Click Me"],
  "meta": {
    "line": 15,
    "column": 8
  }
}
```

### Compilation Cache:
```rust
struct CacheEntry {
    source_hash: String,
    compiled_output: String,
    dependencies: Vec<String>,
    timestamp: SystemTime,
}
```

---

## Performance Targets

| Operation | Target | Current |
|-----------|--------|---------|
| Compile (small file) | < 200ms | ~500ms |
| Compile (large file) | < 1s | ~2s |
| Hot reload (end-to-end) | < 500ms | N/A |
| Visual builder render | < 16ms (60fps) | N/A |

---

## Security Considerations

1. **Code Generation**: Sanitize user input in visual builder
2. **File Watching**: Only watch project directory
3. **WebSocket**: Localhost only (no external connections)
4. **Templates**: Vet community contributions

---

## Scalability

### Phase 1-2: Single User
- Local file system
- No database needed

### Phase 3: Visual Builder
- Option A: Local (Electron app)
- Option B: Cloud (auth + database)

### Phase 4: AI Assistant
- API rate limiting
- Cost management ($0.01-0.10 per generation)
- Caching of common requests

---

See individual phase docs for implementation details!
