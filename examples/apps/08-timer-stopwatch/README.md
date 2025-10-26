# App 8: Timer & Stopwatch ⏱️

**Complexity**: Simple
**Lines**: ~110
**Packages**: None (UI demo - DateTime + signal integration coming soon!)
**Time to Build**: 50 minutes

---

## 📖 Description

A sleek time tracking application featuring:
- **Dual Modes**: Stopwatch and Timer toggle
- **Large Time Display**: Monospace font with hours:minutes:seconds:milliseconds
- **Control Buttons**: Start, Pause, Reset with gradient colors
- **Lap Tracking**: Lap times with fastest/slowest badges
- **Quick Presets**: Egg timer, coffee break, meditation, pizza
- **Smooth Animations**: Button hover effects and pulse animations

---

## ✨ Features

- ✅ Mode selector (Stopwatch/Timer)
- ✅ Large monospace time display (00:00:00.000)
- ✅ Three control buttons (Start, Pause, Reset)
- ✅ Lap times section with 3 sample laps
- ✅ Fastest/slowest lap badges
- ✅ 4 quick preset timers
- ✅ Gradient button colors (green, orange, red, purple)
- ✅ Responsive mobile design

---

## 🚀 Quick Start

```bash
# Compile
cargo run -- compile examples/apps/08-timer-stopwatch/main.jnc

# Run
cd dist && node server.js
# Open http://localhost:3000
```

---

## 🎯 What This App Tests

### Language Features
- [x] **Button controls** - Multiple action buttons
- [x] **Monospace display** - Fixed-width time display
- [x] **Badge components** - Fastest/slowest indicators

### UI Patterns
- [x] **Toggle selector** - Mode switching UI
- [x] **Large numeric display** - Time readout
- [x] **Action buttons** - Start/pause/reset controls
- [x] **List items** - Lap times list
- [x] **Preset grid** - Quick timer buttons

---

## 💡 Key Concepts

### 1. Monospace Time Display

```css
.time-display {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
}
```

Monospace font ensures digits don't shift during updates.

### 2. Gradient Buttons

```css
.btn-primary {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
}
```

Each button has a distinct gradient color scheme.

### 3. Lap Badges

```jounce
<span class="lap-badge fastest">Fastest</span>
```

Conditional badges for fastest and slowest laps.

---

## 🔄 Future Enhancements

- [ ] Real-time updates with setInterval + signal
- [ ] Lap recording with arrays
- [ ] Timer countdown functionality
- [ ] Sound notifications
- [ ] localStorage persistence
- [ ] Custom timer durations
- [ ] Multiple timers

---

## ✅ Success Criteria

- [x] Compiles without errors
- [x] Mode selector renders
- [x] Time display shows 00:00:00.000
- [x] Control buttons render
- [x] Lap times display
- [x] Badges show correctly
- [x] Preset buttons render
- [x] Responsive on mobile

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0
