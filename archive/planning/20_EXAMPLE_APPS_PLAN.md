# 20 Example Apps - Comprehensive Testing Plan

**Purpose**: Build 20 progressively complex apps to stress-test Jounce features and discover edge cases before v1.0.

**Strategy**: Start simple, add complexity incrementally, stop when we find issues to fix.

---

## 🚨 CRITICAL REMINDERS - READ BEFORE EVERY APP 🚨

### **NO QUICK FIXES - DO IT THE RIGHT WAY, EVEN IF IT TAKES LONGER.**

### **WE ARE BUILDING IT TO COMPILE 1 .jnc APP! NOT SEVERAL FILES! NOT CSS FILES!! DO YOU UNDERSTAND!**

**These principles guide ALL development:**
- ✅ ONE .jnc FILE → `cargo run -- compile app.jnc` → WORKING APP
- ✅ NO manual post-compilation steps (copying files, editing HTML, etc.)
- ✅ NO build scripts to hide broken workflows
- ✅ NO separate .css or .js files for "convenience"
- ✅ FIX THE COMPILER if syntax is missing - don't work around it
- ✅ Fix the architecture, not the symptoms
- ✅ Implement features completely or not at all

**Keep this section at the top of CLAUDE.md as a permanent reminder!**

---

## 📊 Difficulty Levels

- **🟢 Beginner (1-3)**: Basic features, 50-100 lines
- **🟡 Intermediate (4-10)**: Multiple features combined, 100-200 lines
- **🟠 Advanced (11-16)**: Complex state, full-stack, 200-400 lines
- **🔴 Expert (17-20)**: Production-grade, real-world complexity, 400+ lines

---

## 🎯 Feature Coverage Matrix

| App # | Signals | Computed | Effects | Server Fns | DB | WebSocket | Router | Forms | Persist | Lifecycle | Error | Suspense |
|-------|---------|----------|---------|------------|-----|-----------|--------|-------|---------|-----------|-------|----------|
| 01    | ✅      | ❌       | ❌      | ❌         | ❌  | ❌        | ❌     | ❌    | ❌      | ❌        | ❌    | ❌       |
| 02    | ✅      | ✅       | ❌      | ❌         | ❌  | ❌        | ❌     | ❌    | ❌      | ❌        | ❌    | ❌       |
| 03    | ✅      | ✅       | ❌      | ❌         | ❌  | ❌        | ❌     | ✅    | ❌      | ❌        | ❌    | ❌       |
| 04    | ✅      | ✅       | ❌      | ❌         | ❌  | ❌        | ❌     | ✅    | ✅      | ❌        | ❌    | ❌       |
| 05    | ✅      | ✅       | ✅      | ❌         | ❌  | ❌        | ❌     | ✅    | ❌      | ✅        | ❌    | ❌       |
| 06    | ✅      | ✅       | ✅      | ✅         | ❌  | ❌        | ❌     | ❌    | ❌      | ✅        | ❌    | ✅       |
| 07    | ✅      | ✅       | ❌      | ✅         | ✅  | ❌        | ❌     | ✅    | ❌      | ✅        | ❌    | ✅       |
| 08    | ✅      | ✅       | ❌      | ✅         | ✅  | ❌        | ✅     | ✅    | ❌      | ✅        | ❌    | ✅       |
| 09    | ✅      | ✅       | ✅      | ✅         | ✅  | ❌        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 10    | ✅      | ✅       | ❌      | ✅         | ✅  | ❌        | ✅     | ✅    | ❌      | ✅        | ✅    | ✅       |
| 11    | ✅      | ✅       | ✅      | ✅         | ✅  | ✅        | ❌     | ❌    | ❌      | ✅        | ✅    | ✅       |
| 12    | ✅      | ✅       | ✅      | ✅         | ✅  | ✅        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 13    | ✅      | ✅       | ✅      | ✅         | ✅  | ❌        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 14    | ✅      | ✅       | ✅      | ✅         | ✅  | ❌        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 15    | ✅      | ✅       | ✅      | ✅         | ✅  | ✅        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 16    | ✅      | ✅       | ✅      | ✅         | ✅  | ❌        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 17    | ✅      | ✅       | ✅      | ✅         | ✅  | ✅        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 18    | ✅      | ✅       | ✅      | ✅         | ✅  | ❌        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 19    | ✅      | ✅       | ✅      | ✅         | ✅  | ✅        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |
| 20    | ✅      | ✅       | ✅      | ✅         | ✅  | ✅        | ✅     | ✅    | ✅      | ✅        | ✅    | ✅       |

---

## 🟢 BEGINNER APPS (1-3)

### **App 01: Click Counter** ⏱️ 15 min
**Difficulty**: 🟢 Beginner
**Lines**: ~50
**Features**: Signals only

**Description**:
Simple counter with increment/decrement buttons. Tests basic signal reactivity.

**Key Features**:
- `let count = signal(0)`
- Button click handlers
- Display reactive value

**Edge Cases to Test**:
- Negative numbers
- Large numbers (1000+)
- Rapid clicking

**Success Criteria**:
- ✅ Count updates on click
- ✅ No lag with rapid clicks
- ✅ Negative numbers display correctly

---

### **App 02: Temperature Converter** ⏱️ 20 min
**Difficulty**: 🟢 Beginner
**Lines**: ~70
**Features**: Signals + Computed

**Description**:
Convert between Celsius, Fahrenheit, and Kelvin. Tests computed values.

**Key Features**:
- Input for Celsius
- Computed Fahrenheit: `computed(() => (celsius.value * 9/5) + 32)`
- Computed Kelvin: `computed(() => celsius.value + 273.15)`

**Edge Cases to Test**:
- Decimal inputs (23.5)
- Negative temperatures
- Zero (absolute zero in different units)
- Empty input

**Success Criteria**:
- ✅ All three values update together
- ✅ Correct conversions
- ✅ Handles decimals

---

### **App 03: BMI Calculator** ⏱️ 25 min
**Difficulty**: 🟢 Beginner
**Lines**: ~90
**Features**: Signals + Computed + Basic Forms

**Description**:
Calculate BMI from height and weight. Tests form inputs with validation.

**Key Features**:
- Weight input (kg)
- Height input (cm)
- Computed BMI: `weight / (height/100)^2`
- Category label (Underweight, Normal, Overweight, Obese)

**Edge Cases to Test**:
- Zero values
- Very large values (300kg, 250cm)
- Empty inputs
- Non-numeric input

**Success Criteria**:
- ✅ BMI calculates correctly
- ✅ Category updates automatically
- ✅ Handles invalid input gracefully

---

## 🟡 INTERMEDIATE APPS (4-10)

### **App 04: Expense Tracker** ⏱️ 30 min
**Difficulty**: 🟡 Intermediate
**Lines**: ~120
**Features**: Signals + Computed + Forms + Persistent

**Description**:
Track expenses with categories, compute totals, persist to localStorage.

**Key Features**:
- Array of expenses: `persistentSignal("expenses", [])`
- Add expense form (amount, category, description)
- Computed total
- Filter by category
- Delete expense

**Edge Cases to Test**:
- Adding duplicate expenses
- Deleting all expenses
- Very long descriptions
- Special characters in description
- localStorage full scenario

**Success Criteria**:
- ✅ Expenses persist after reload
- ✅ Total updates correctly
- ✅ Filter works
- ✅ Delete works

---

### **App 05: Pomodoro Timer** ⏱️ 35 min
**Difficulty**: 🟡 Intermediate
**Lines**: ~150
**Features**: Signals + Computed + Effects + Forms + Lifecycle

**Description**:
25-minute work timer with 5-minute breaks. Tests `effect()` with timers.

**Key Features**:
- Timer state: work/break
- Countdown with `effect()` and `setInterval`
- Start/pause/reset buttons
- Session counter
- Audio notification (onMount to load audio)

**Edge Cases to Test**:
- Pausing mid-timer
- Resetting during countdown
- Multiple starts without pause
- Browser tab backgrounded (timer continues?)
- Component unmount (cleanup interval)

**Success Criteria**:
- ✅ Timer counts down correctly
- ✅ Switches to break automatically
- ✅ Pause/resume works
- ✅ No memory leaks (onUnmount cleans up)

---

### **App 06: Weather Widget** ⏱️ 40 min
**Difficulty**: 🟡 Intermediate
**Lines**: ~130
**Features**: Signals + Computed + Effects + Server Functions + Lifecycle + Suspense

**Description**:
Fetch weather data from API, display current conditions and forecast.

**Key Features**:
- Server function: `get_weather(city: string)` (uses fetch to OpenWeather API)
- Loading state with Suspense
- City search input
- Display temp, conditions, humidity
- 5-day forecast

**Edge Cases to Test**:
- Invalid city name
- Network error (API down)
- Empty search
- Special characters in city name ("São Paulo")
- API rate limit

**Success Criteria**:
- ✅ Suspense shows loading state
- ✅ Weather data displays correctly
- ✅ Error handling for bad city names
- ✅ Handles network errors gracefully

---

### **App 07: Contact Book** ⏱️ 45 min
**Difficulty**: 🟡 Intermediate
**Lines**: ~180
**Features**: Signals + Computed + Server Fns + Database + Forms + Lifecycle + Suspense

**Description**:
Full CRUD contact manager with SQLite database.

**Key Features**:
- Server functions: `add_contact()`, `get_contacts()`, `update_contact()`, `delete_contact()`
- SQLite schema: id, name, email, phone
- Add contact form with validation
- Edit inline or modal
- Search/filter contacts
- Pagination (10 per page)

**Edge Cases to Test**:
- Duplicate emails
- Invalid email formats
- Empty fields
- SQL injection attempts ("'; DROP TABLE--")
- Very long names (500+ chars)
- Special characters in all fields

**Success Criteria**:
- ✅ CRUD operations work
- ✅ Database persists data
- ✅ Validation works
- ✅ Search filters correctly
- ✅ Pagination works

---

### **App 08: Blog Platform** ⏱️ 50 min
**Difficulty**: 🟡 Intermediate
**Lines**: ~220
**Features**: Signals + Computed + Server Fns + Database + Router + Forms + Lifecycle + Suspense

**Description**:
Multi-page blog with posts, comments, and tags. Tests routing.

**Key Features**:
- Routes: `/`, `/post/:id`, `/new-post`, `/tag/:tag`
- Server functions: posts CRUD, comments CRUD, tags
- SQLite: posts, comments, tags tables
- Rich text editor (textarea with preview)
- Comment threads
- Tag filtering

**Edge Cases to Test**:
- Navigating to non-existent post ID
- Markdown/HTML in post content
- Nested comments (replies)
- Empty posts
- Very long posts (10,000+ words)
- Duplicate tags
- URL manipulation

**Success Criteria**:
- ✅ Routing works without page reload
- ✅ Posts display correctly
- ✅ Comments save and display
- ✅ Tags filter posts
- ✅ Markdown renders

---

### **App 09: E-Commerce Cart** ⏱️ 55 min
**Difficulty**: 🟡 Intermediate
**Lines**: ~250
**Features**: Signals + Computed + Effects + Server Fns + Database + Router + Forms + Persistent + Lifecycle + Error Boundaries + Suspense

**Description**:
Full shopping cart with products, cart, and checkout. Tests complex state.

**Key Features**:
- Routes: `/products`, `/cart`, `/checkout`
- Server functions: `get_products()`, `create_order()`
- SQLite: products, orders, order_items tables
- Product listing with filters (price range, category)
- Add to cart (persisted to localStorage)
- Cart subtotal/tax/total computed
- Checkout form with validation
- ErrorBoundary for failed product loads

**Edge Cases to Test**:
- Adding same product multiple times
- Removing items from cart
- Changing quantities (0, negative, 1000+)
- Checkout with empty cart
- Invalid credit card format
- Out of stock items
- Price changes while in cart

**Success Criteria**:
- ✅ Cart persists across sessions
- ✅ Totals calculate correctly
- ✅ Checkout validates input
- ✅ Orders save to database
- ✅ Error boundaries catch failures

---

### **App 10: Quiz Game** ⏱️ 1 hour
**Difficulty**: 🟡 Intermediate
**Lines**: ~200
**Features**: Signals + Computed + Server Fns + Database + Router + Forms + Lifecycle + Error Boundaries + Suspense

**Description**:
Trivia quiz with questions from database, scoring, and leaderboard.

**Key Features**:
- Routes: `/`, `/quiz/:category`, `/leaderboard`
- Server functions: `get_questions(category)`, `submit_score()`
- SQLite: questions, scores tables
- Multiple choice questions
- Timer per question
- Score calculation
- Leaderboard with top 10
- Error boundary for missing questions

**Edge Cases to Test**:
- No questions in category
- Timer expires (auto-submit)
- Closing browser mid-quiz
- Duplicate score submissions
- Invalid category
- Cheating detection (rapid answers)

**Success Criteria**:
- ✅ Questions load and display
- ✅ Timer works correctly
- ✅ Score calculates properly
- ✅ Leaderboard updates
- ✅ Handles missing data gracefully

---

## 🟠 ADVANCED APPS (11-16)

### **App 11: Real-Time Chat** ⏱️ 1.5 hours
**Difficulty**: 🟠 Advanced
**Lines**: ~300
**Features**: Signals + Computed + Effects + Server Fns + Database + WebSocket + Lifecycle + Error Boundaries + Suspense

**Description**:
Multi-room chat with WebSocket for real-time messaging.

**Key Features**:
- WebSocket for live messages
- Server functions: `get_rooms()`, `get_messages(room_id)`, `create_room()`
- SQLite: rooms, messages, users tables
- Room list with unread counts
- Message history (paginated)
- Typing indicators
- Online users list
- Error boundary for WebSocket failures

**Edge Cases to Test**:
- WebSocket disconnect/reconnect
- Messages sent while offline
- Very long messages (10,000+ chars)
- Rapid message sending (spam)
- Room with no messages
- Multiple tabs open (sync state?)
- Special characters/emoji in messages

**Success Criteria**:
- ✅ Messages appear in real-time
- ✅ Reconnects after disconnect
- ✅ Typing indicators work
- ✅ Message history loads
- ✅ Handles offline gracefully

---

### **App 12: Project Management Board** ⏱️ 2 hours
**Difficulty**: 🟠 Advanced
**Lines**: ~400
**Features**: ALL FEATURES (Signals, Computed, Effects, Server Fns, DB, WebSocket, Router, Forms, Persistent, Lifecycle, Error, Suspense)

**Description**:
Trello-style kanban board with drag-and-drop, real-time collaboration.

**Key Features**:
- Routes: `/boards`, `/board/:id`
- WebSocket for real-time updates
- Server functions: boards CRUD, tasks CRUD, `move_task()`
- SQLite: boards, columns, tasks tables
- Drag-and-drop tasks between columns
- Persistent board state
- Assign users to tasks
- Due dates and priorities
- Filters and search
- Error boundaries for failed updates

**Edge Cases to Test**:
- Dragging to invalid column
- Concurrent edits from multiple users
- Moving tasks while WebSocket disconnected
- Deleting column with tasks
- Very long task descriptions
- Invalid due dates
- Moving 100+ tasks at once

**Success Criteria**:
- ✅ Drag-and-drop works smoothly
- ✅ Real-time updates from other users
- ✅ State persists correctly
- ✅ Filters work
- ✅ Handles conflicts gracefully

---

### **App 13: Recipe Book** ⏱️ 1.5 hours
**Difficulty**: 🟠 Advanced
**Lines**: ~350
**Features**: Signals + Computed + Effects + Server Fns + Database + Router + Forms + Persistent + Lifecycle + Error Boundaries + Suspense

**Description**:
Recipe manager with ingredients, instructions, ratings, and meal planning.

**Key Features**:
- Routes: `/recipes`, `/recipe/:id`, `/meal-plan`
- Server functions: recipes CRUD, `rate_recipe()`, `add_to_meal_plan()`
- SQLite: recipes, ingredients, ratings, meal_plan tables
- Search by ingredients
- Filter by cuisine, prep time, difficulty
- Step-by-step mode
- Shopping list generator
- Nutritional info calculator

**Edge Cases to Test**:
- Recipe with 100+ ingredients
- Fractional measurements (1/3 cup)
- Unit conversions (metric/imperial)
- Missing ingredients
- Invalid ratings (negative, > 5)
- Meal plan conflicts (same day)
- Special diet filters (vegan, gluten-free)

**Success Criteria**:
- ✅ Search works efficiently
- ✅ Ratings update correctly
- ✅ Shopping list generates properly
- ✅ Filters combine correctly
- ✅ Handles missing data

---

### **App 14: Fitness Tracker** ⏱️ 1.5 hours
**Difficulty**: 🟠 Advanced
**Lines**: ~320
**Features**: Signals + Computed + Effects + Server Fns + Database + Router + Forms + Persistent + Lifecycle + Error Boundaries + Suspense

**Description**:
Track workouts, progress, and goals with charts and statistics.

**Key Features**:
- Routes: `/dashboard`, `/workouts`, `/workout/:id`, `/progress`
- Server functions: workouts CRUD, `get_stats(date_range)`
- SQLite: workouts, exercises, sets, user_stats tables
- Workout builder (exercises + sets/reps)
- Progress charts (weight lifted over time)
- Personal records tracker
- Goal setting and tracking
- Rest timer

**Edge Cases to Test**:
- Workout with 50+ exercises
- Invalid reps/weight (0, negative)
- Incomplete workouts
- Chart with no data
- Very long date ranges (5+ years)
- Goals with past dates
- Concurrent workout tracking (multiple tabs)

**Success Criteria**:
- ✅ Workouts save correctly
- ✅ Charts render accurately
- ✅ PRs calculate correctly
- ✅ Goals track progress
- ✅ Rest timer works

---

### **App 15: Social Media Feed** ⏱️ 2 hours
**Difficulty**: 🟠 Advanced
**Lines**: ~380
**Features**: ALL FEATURES

**Description**:
Twitter-like feed with posts, likes, comments, followers, real-time updates.

**Key Features**:
- Routes: `/feed`, `/profile/:username`, `/post/:id`
- WebSocket for real-time likes/comments
- Server functions: posts CRUD, `like_post()`, `follow_user()`, `get_feed()`
- SQLite: users, posts, likes, comments, follows tables
- Infinite scroll feed
- Image uploads
- @ mentions and # hashtags
- Notifications
- Error boundaries for failed posts

**Edge Cases to Test**:
- Posting while offline (queue?)
- Like spam (rapid clicking)
- Self-following
- Circular follows (A→B→C→A)
- Posts with 1000+ likes
- Very long posts (10,000 chars)
- Invalid image formats
- Mention non-existent users

**Success Criteria**:
- ✅ Feed loads and scrolls infinitely
- ✅ Real-time likes/comments appear
- ✅ Follow/unfollow works
- ✅ Notifications show up
- ✅ Handles offline gracefully

---

### **App 16: Markdown Editor** ⏱️ 1.5 hours
**Difficulty**: 🟠 Advanced
**Lines**: ~280
**Features**: Signals + Computed + Effects + Server Fns + Database + Router + Forms + Persistent + Lifecycle + Error Boundaries + Suspense

**Description**:
Rich markdown editor with live preview, save, and export.

**Key Features**:
- Routes: `/editor`, `/docs`, `/doc/:id`
- Server functions: docs CRUD, `export_pdf()`, `export_html()`
- SQLite: documents, versions tables
- Split-pane editor (markdown | preview)
- Syntax highlighting
- Auto-save every 5 seconds
- Version history
- Export to PDF/HTML/Markdown
- Collaborative editing (cursor positions)

**Edge Cases to Test**:
- Very large documents (100,000+ chars)
- Rapid typing (auto-save throttling)
- Invalid markdown syntax
- Nested lists (10+ levels)
- Code blocks with syntax highlighting
- Simultaneous edits (conflict resolution)
- Browser crash (recovery from auto-save)

**Success Criteria**:
- ✅ Preview updates in real-time
- ✅ Auto-save works without lag
- ✅ Version history restores correctly
- ✅ Export produces valid files
- ✅ Handles large docs smoothly

---

## 🔴 EXPERT APPS (17-20)

### **App 17: Video Streaming Platform** ⏱️ 3 hours
**Difficulty**: 🔴 Expert
**Lines**: ~500+
**Features**: ALL FEATURES + File Uploads + Video Processing

**Description**:
YouTube-style platform with video upload, playback, comments, subscriptions.

**Key Features**:
- Routes: `/`, `/watch/:id`, `/channel/:id`, `/upload`
- WebSocket for real-time view counts
- Server functions: videos CRUD, `upload_video()`, `process_video()`, `subscribe()`
- SQLite: videos, channels, subscriptions, watch_history tables
- Video player with controls
- Thumbnails generation
- View counter (increments on load)
- Recommended videos algorithm
- Comments with threading

**Edge Cases to Test**:
- Very large video files (>1GB)
- Unsupported video formats
- Upload progress interruption
- Seeking in video (buffering)
- Autoplay next video
- Concurrent uploads
- View count race conditions

**Success Criteria**:
- ✅ Videos upload and process
- ✅ Playback is smooth
- ✅ View counts accurate
- ✅ Recommendations relevant
- ✅ Handles large files

---

### **App 18: Code Playground** ⏱️ 3 hours
**Difficulty**: 🔴 Expert
**Lines**: ~450+
**Features**: Signals + Computed + Effects + Server Fns + Database + Router + Forms + Persistent + Lifecycle + Error Boundaries + Suspense + Code Execution

**Description**:
CodePen-style editor for HTML/CSS/JS with live preview and sharing.

**Key Features**:
- Routes: `/`, `/pen/:id`, `/user/:username`
- Server functions: `save_pen()`, `fork_pen()`, `run_code()`
- SQLite: pens, users, forks tables
- Monaco editor for HTML/CSS/JS
- Live preview iframe
- Console output capture
- Save and share links
- Fork existing pens
- Syntax validation

**Edge Cases to Test**:
- Infinite loops in JS (timeout?)
- XSS attempts in preview
- Very large code (100,000 lines)
- Invalid CSS/JS syntax
- Rapid typing (debounce preview)
- Console.log spam (1000+ logs)
- iframe sandbox escapes

**Success Criteria**:
- ✅ Preview updates live
- ✅ Code saves correctly
- ✅ Sharing works
- ✅ Fork creates copy
- ✅ Sandboxing prevents exploits

---

### **App 19: Multiplayer Game** ⏱️ 4 hours
**Difficulty**: 🔴 Expert
**Lines**: ~600+
**Features**: ALL FEATURES + Game Loop + Collision Detection

**Description**:
Real-time multiplayer snake game with WebSocket synchronization.

**Key Features**:
- WebSocket for player positions
- Server functions: `create_game()`, `join_game()`, `get_leaderboard()`
- SQLite: games, players, scores tables
- Canvas rendering
- Game loop (60 FPS)
- Collision detection
- Scoring system
- Leaderboard
- Spectator mode

**Edge Cases to Test**:
- Network lag (interpolation?)
- Player disconnect mid-game
- Simultaneous collisions
- Cheating (client-side manipulation)
- Game with 100+ players
- Very fast movement (missed collisions)
- Browser tab backgrounded (pause game?)

**Success Criteria**:
- ✅ Game runs at 60 FPS
- ✅ Multiplayer syncs correctly
- ✅ Collisions detected accurately
- ✅ Leaderboard updates
- ✅ Handles lag gracefully

---

### **App 20: Business Dashboard** ⏱️ 4 hours
**Difficulty**: 🔴 Expert
**Lines**: ~550+
**Features**: ALL FEATURES + Complex Charts + Data Aggregation

**Description**:
Comprehensive analytics dashboard with real-time data, charts, and reports.

**Key Features**:
- Routes: `/dashboard`, `/reports`, `/settings`
- WebSocket for real-time metrics
- Server functions: `get_metrics(date_range)`, `export_report()`, `aggregate_data()`
- SQLite: metrics, events, users, sessions tables
- Multiple chart types (line, bar, pie, heatmap)
- Date range picker
- Filter and drill-down
- Export to CSV/PDF
- Alert thresholds (email/SMS when metric > X)

**Edge Cases to Test**:
- Chart with 10,000+ data points
- Very long date ranges (10+ years)
- Real-time updates while viewing
- Concurrent report exports
- Invalid date ranges (end before start)
- Missing data (null values)
- Aggregation edge cases (division by zero)

**Success Criteria**:
- ✅ Charts render smoothly
- ✅ Real-time updates don't lag
- ✅ Filters work correctly
- ✅ Exports produce valid files
- ✅ Alerts trigger properly

---

## 🎯 Testing Strategy

### **Phase 1: Quick Wins (Apps 1-3)** - 1 hour
Build 3 simple apps to verify basics work. If these fail, we have fundamental issues.

### **Phase 2: Feature Coverage (Apps 4-10)** - 4-5 hours
Build 7 intermediate apps that combine features. This tests feature interactions.

### **Phase 3: Advanced Scenarios (Apps 11-16)** - 9-12 hours
Build 6 advanced apps with complex state and real-world patterns. This tests scalability.

### **Phase 4: Production Grade (Apps 17-20)** - 14-16 hours
Build 4 expert apps that push limits. This tests production readiness.

### **Stop Condition**:
**Stop after finding 5-10 issues across any phase.** Document issues, prioritize, and fix before continuing.

---

## 📊 Expected Issues to Find

Based on the complexity, here are likely issues we'll discover:

### **Compiler Issues**:
- [ ] Complex nested expressions not detected as reactive
- [ ] Type inference failures with deeply nested signals
- [ ] RPC generation errors with complex types
- [ ] Code splitting issues with shared state

### **Runtime Issues**:
- [ ] Memory leaks with effects not cleaning up
- [ ] WebSocket reconnection failures
- [ ] Race conditions with concurrent updates
- [ ] Signal updates not batching correctly

### **Performance Issues**:
- [ ] Slow compilation with large files (500+ lines)
- [ ] Laggy UI with 1000+ reactive nodes
- [ ] Database query N+1 problems
- [ ] Bundle size too large (>500KB)

### **Developer Experience Issues**:
- [ ] Confusing error messages
- [ ] Missing features (async/await?)
- [ ] Verbose syntax for common patterns
- [ ] Poor debugging experience

---

## 📋 Issue Tracking Template

For each issue found, document:

```markdown
## Issue #X: [Brief Description]

**App**: App #X - [Name]
**Severity**: Critical / Important / Minor
**Category**: Compiler / Runtime / Performance / DX

**Description**:
[What went wrong]

**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]
3. [Error occurs]

**Expected Behavior**:
[What should happen]

**Actual Behavior**:
[What actually happens]

**Workaround** (if any):
[Temporary fix]

**Fix Priority**: High / Medium / Low
**Estimated Fix Time**: [hours]
```

---

## ✅ Success Metrics

After building all 20 apps (or stopping at issues), we should have:

1. **✅ Feature Coverage**: Every feature used in at least 3 apps
2. **✅ Edge Cases Found**: 20+ edge cases documented
3. **✅ Issues Found**: 5-10 real issues discovered and documented
4. **✅ Performance Baseline**: Compile times < 100ms per app
5. **✅ Developer Confidence**: Can recommend Jounce for production

---

## 🛠️ IMPROVEMENTS NEEDED, IDEAS, AND BUGS

**This section tracks issues discovered while building the 20 apps.**

### **🔴 CRITICAL BUGS**
*(Stop everything and fix immediately)*

#### **Issue #1: Ternary Operator Precedence Bug**
**App**: App 01 - Click Counter
**Severity**: 🔴 Critical
**Category**: Compiler (JSEmitter)
**Discovered**: October 27, 2025

**Description**:
Ternary operators inside JSX expressions generate JavaScript with incorrect parentheses placement, causing syntax errors.

**Steps to Reproduce**:
1. Write JSX with conditional: `{count.value > 0 ? <span>Positive</span> : <span>Negative</span>}`
2. Compile the app
3. Check generated JavaScript

**Expected Behavior**:
```javascript
(count.value > 0) ? h('span', ...) : h('span', ...)
```

**Actual Behavior**:
```javascript
count.value > (0 ? h('span', ...) : h('span', ...))
```

The parentheses wrap `0` and the ternary instead of wrapping the comparison expression.

**Impact**:
- JavaScript syntax error
- App won't run in browser
- Conditional rendering broken

**Workaround**:
Avoid ternary operators in JSX for now.

**Fix Priority**: 🔥 IMMEDIATE
**Estimated Fix Time**: 30-60 minutes
**Fix Location**: `src/js_emitter.rs` - ternary expression generation logic

---

#### **Issue #2: HTML Entities Not Supported** - `App 03`
- Parser crashes on `&lt;`, `&gt;`, `&amp;`, etc.
- Fix: `src/lexer.rs` or `src/parser.rs`
- Time: 1-2 hours

#### **Issue #3: Numbers in JSX Text Cause Parser Errors** - `App 03`
- Can't write `<p>Age: 25</p>` - parser crashes on numbers in text
- Fix: `src/parser.rs` - JSX text parsing
- Time: 1-2 hours

#### **Issue #4 & #5: Method Calls on `.value` Not Reactive** - `Apps 03, 04`
- `.toFixed()`, `.map()` not wrapped in effects
- `.value.map()` lists don't update when array changes
- Fix: `src/reactive_analyzer.rs` - detect MethodCall expressions
- Time: 1 hour

---

### **🟡 IMPORTANT IMPROVEMENTS NEEDED**
*(Should fix before v1.0)*

#### **Issue #6: JSX Comments Not Supported** - `App 05`
- `{/* comment */}` causes parser error
- Fix: `src/parser.rs` - JSX expression parsing
- Time: 30-45 minutes

#### **Issue #7: Async/Await Not Supported** - `App 08`
- Must use `.then()` chains (verbose)
- Fix: `src/parser.rs` + `src/js_emitter.rs`
- Time: 3-4 hours

#### **Issue #8: Object Spread Not Supported** - `App 09`
- Can't use `{...obj1, ...obj2}`
- Fix: `src/parser.rs` - spread in object literals
- Time: 2-3 hours

#### **Issue #9: Template Literals Not Supported** - `App 10`
- Can't use backticks: `` `Hello ${name}` ``
- Fix: `src/lexer.rs` + `src/parser.rs`
- Time: 3-4 hours

#### **Issue #10: Destructuring Not Supported** - `App 11`
- Can't use `let { name, age } = user;`
- Fix: `src/parser.rs` - destructuring patterns
- Time: 4-5 hours

---

### **🟢 NICE-TO-HAVE IDEAS**
*(Can defer to future versions)*

**None yet!** 🎉

---

### **📝 KNOWN LIMITATIONS**
*(Documented in RETROSPECTIVE.md)*

- ⏸️ No async/await syntax (use `.then()` chains)
- ⏸️ No source maps (debug generated JS)
- ⏸️ No LSP (no IDE autocomplete)
- ⏸️ No code splitting (single bundle)
- ⏸️ No HMR (full page reload)
- ⏸️ No CSS-in-JS syntax (use inline styles)
- ⏸️ No object spread operator (use manual merge)

---

### **✅ FIXED ISSUES**
*(Document fixes here as we go)*

**None yet!** Ready to find them! 🚀

---

**Let's start building! 🚀**

**First Step**: App 01 - Click Counter (15 min)
