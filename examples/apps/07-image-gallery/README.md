# App 7: Image Gallery 📷

**Complexity**: Simple
**Lines**: ~170
**Packages**: None (UI demo - jounce-image integration coming soon!)
**Time to Build**: 55 minutes

---

## 📖 Description

A beautiful image gallery showcasing:
- **Responsive Grid Layout**: Auto-adjusts columns based on screen size
- **Category Filters**: All, Nature, Urban, People, Architecture
- **Image Cards**: Gradient placeholders with emoji icons
- **Image Info**: Title, category, dimensions, likes, views
- **Hover Effects**: Card lift and icon scale animations
- **Gradient Header**: Text with gradient background clip

---

## ✨ Features

- ✅ 9 image cards in responsive grid
- ✅ 5 category filter buttons (All is active)
- ✅ Gradient placeholder images (4 color schemes)
- ✅ Image metadata (title, category, dimensions)
- ✅ Engagement stats (likes, views)
- ✅ Hover animations (card lift, icon scale)
- ✅ Responsive: 1 column (mobile), 2 (tablet), 3 (desktop)
- ✅ Gradient text header effect

---

## 🚀 Quick Start

```bash
# Compile
cargo run -- compile examples/apps/07-image-gallery/main.jnc

# Run
cd dist && node server.js
# Open http://localhost:3000
```

---

## 🎯 What This App Tests

### Language Features
- [x] **Grid layouts** - Responsive image grid
- [x] **Card components** - Image cards with info
- [x] **Filter buttons** - Category filtering UI

### UI Patterns
- [x] **Responsive grid** - Auto-fill columns
- [x] **Gradient backgrounds** - Multiple color schemes
- [x] **Hover states** - Card lift and scale
- [x] **Aspect ratios** - 16:9 image containers
- [x] **Gradient text** - Background-clip text effect

---

## 💡 Key Concepts

### 1. Auto-Fill Grid

```css
.gallery {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 30px;
}
```

Automatically creates columns based on available width.

### 2. Aspect Ratio Container

```css
.image-placeholder {
    aspect-ratio: 16 / 9;
}
```

Maintains consistent image proportions.

### 3. Gradient Text

```css
h1 {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}
```

Text with gradient fill effect.

### 4. Card Hover Lift

```css
.gallery-item:hover {
    transform: translateY(-8px);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
}
```

Card lifts up on hover with enhanced shadow.

---

## 🎨 Design Notes

**Color Schemes:**
- Nature: Teal to pink gradient
- Urban: Purple gradient
- People: Pink to yellow gradient
- Architecture: Cyan to dark purple

**Responsive Breakpoints:**
- Mobile (<769px): 1 column
- Tablet (769-1024px): 2 columns
- Desktop (1025px+): 3 columns

---

## 🔄 Future Enhancements

- [ ] Real image loading with jounce-image
- [ ] Lightbox on image click
- [ ] Lazy loading with intersection observer
- [ ] Filter functionality with signal
- [ ] Infinite scroll pagination
- [ ] Image upload with jounce-storage
- [ ] Search and tags

---

## ✅ Success Criteria

- [x] Compiles without errors
- [x] 9 images render in grid
- [x] Responsive grid works
- [x] Filter buttons render
- [x] Hover animations work
- [x] Image info displays
- [x] Gradient text effect works

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0
