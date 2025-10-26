# App 9: Quiz App 🧠

**Complexity**: Simple
**Lines**: ~100
**Packages**: None (UI demo - quiz logic with signal coming soon!)
**Time to Build**: 55 minutes

---

## 📖 Description

An interactive quiz interface demonstrating:
- **Progress Tracking**: Visual progress bar and question counter
- **Multiple Choice Questions**: Four answer options (A, B, C, D)
- **Answer Feedback**: Correct/wrong indicators with checkmarks/X
- **Explanations**: Educational feedback after answering
- **Score Display**: Real-time score tracking
- **Navigation**: Previous/Next buttons

---

## ✨ Features

- ✅ Quiz header with title and progress bar
- ✅ Progress indicator (Question 2 of 5, 40%)
- ✅ Multiple choice answers with letter badges
- ✅ Correct/wrong answer highlighting (green/red)
- ✅ Answer explanation box with code examples
- ✅ Score card with percentage and breakdown
- ✅ Previous/Next navigation buttons
- ✅ Gradient design elements

---

## 🚀 Quick Start

```bash
# Compile
cargo run -- compile examples/apps/09-quiz-app/main.jnc

# Run
cd dist && node server.js
# Open http://localhost:3000
```

---

## 🎯 What This App Tests

### Language Features
- [x] **Progress bars** - Visual completion indicator
- [x] **Button lists** - Multiple choice options
- [x] **Conditional styling** - Correct/wrong states

### UI Patterns
- [x] **Quiz interface** - Question and answer layout
- [x] **Badge components** - Letter indicators
- [x] **Feedback boxes** - Explanation section
- [x] **Score tracking** - Real-time statistics
- [x] **Navigation controls** - Previous/next buttons

---

## 💡 Key Concepts

### 1. Progress Bar

```css
.progress-fill {
    width: 40%;
    background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
}
```

Dynamic width shows quiz completion percentage.

### 2. Answer States

```jounce
<button class="answer-btn correct selected">
    <span class="answer-check">✓</span>
</button>
```

Visual feedback for correct/wrong/selected answers.

### 3. Score Breakdown

Three categories: Correct (green), Wrong (red), Unanswered (gray).

---

## 🔄 Future Enhancements

- [ ] Dynamic questions with signal arrays
- [ ] Answer selection with onClick
- [ ] Automatic progress updates
- [ ] Timer per question
- [ ] Quiz completion screen
- [ ] Question categories/difficulty
- [ ] Results sharing

---

## ✅ Success Criteria

- [x] Compiles without errors
- [x] Progress bar renders
- [x] Question displays
- [x] 4 answer options render
- [x] Correct/wrong highlighting works
- [x] Explanation box shows
- [x] Score card displays
- [x] Navigation buttons render

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0
