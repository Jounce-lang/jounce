# Jounce Examples Gallery

A visual catalog of all Jounce example applications.

## Features

- **47+ Example Apps** - Browse the entire collection
- **Category Filtering** - Filter by Basics, Forms, Utilities, etc.
- **Search** - Find apps by name, description, or features
- **Interactive Cards** - Click to view source code
- **Responsive Design** - Works on desktop and mobile

## Usage

### View Locally

```bash
# From the gallery directory
python3 -m http.server 8080

# Open http://localhost:8080 in your browser
```

### Regenerate Catalog

```bash
python3 generate_catalog.py
```

This scans `examples/apps/` and updates `data/apps.json`.

## Structure

```
gallery/
├── index.html              # Main gallery page
├── viewer.html             # App code viewer (TODO)
├── assets/
│   ├── gallery.css         # Styles
│   └── gallery.js          # Interactive features
├── data/
│   └── apps.json           # Generated catalog
├── generate_catalog.py     # Catalog generator script
└── README.md               # This file
```

## Categories

- **Basics** - Hello World, counters, simple buttons
- **Forms** - Input handling, validation
- **Utilities** - Calculators, converters
- **Interactivity** - Games, timers, animations
- **Real-World** - Todo apps, dashboards
- **Styling** - Themes, CSS examples
- **Advanced** - Server functions, databases

## Next Steps

- [ ] Create viewer.html for viewing source code
- [ ] Add screenshots
- [ ] Deploy to GitHub Pages
- [ ] Add "Copy Code" button
- [ ] Add live demo iframes
