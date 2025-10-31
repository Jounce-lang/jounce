# Dashboard Template

**Multi-component layout** - Learn component composition and grid layouts!

## Features

✅ **Component composition** - Reusable StatCard and ActivityItem
✅ **Grid layouts** - Responsive grid with breakpoints
✅ **Component props** - Pass data to child components
✅ **Conditional styling** - Dynamic colors based on trend
✅ **Utility classes** - Layout with Tailwind-inspired CSS

## Quick Start

```bash
# 1. Copy this template
cp -r templates/dashboard my-dashboard
cd my-dashboard

# 2. Compile and run
jnc compile main.jnc
cd dist && node server.js

# 3. Open browser
open http://localhost:3000
```

## What You'll Learn

### 1. Component Composition
```jounce
// Parent component
component App() {
    return <div>
        <StatCard
            title="Total Users"
            value={users.value}
            change="+12.5%"
            trend="up"
        />
    </div>;
}

// Child component
component StatCard(title: String, value: String, change: String, trend: String) {
    return <div class="card">
        <h3>{title}</h3>
        <p>{value}</p>
    </div>;
}
```

### 2. Responsive Grid
```jounce
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
    // Mobile: 1 column
    // Tablet: 2 columns
    // Desktop: 4 columns
</div>
```

### 3. Component Props with Types
```jounce
component StatCard(
    title: String,
    value: String,
    change: String,
    trend: String,
    icon: String
) {
    let isPositive = trend == "up";
    // ...
}
```

### 4. Conditional Styling
```jounce
<span class={`font-semibold ${isPositive ? "text-success" : "text-danger"}`}>
    {change}
</span>
```

### 5. Layout Patterns
```jounce
// Header with flex
<div class="flex justify-between items-center">
    <div>...</div>
    <button>...</button>
</div>

// Card grid
<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <div class="lg:col-span-2">...</div>  // Takes 2 columns
    <div>...</div>                         // Takes 1 column
</div>
```

## Components

### App (Main Component)
- Manages dashboard state
- Renders header, stats, and content sections
- Coordinates child components

### StatCard
- Displays a single metric
- Props: title, value, change, trend, icon
- Dynamic styling based on trend

### ActivityItem
- Shows user activity
- Props: user, action, time
- Avatar with user initial

## Layout Structure

```
Dashboard
├── Header (flex row)
│   ├── Title & subtitle
│   └── Action button
├── Stats Grid (4 columns)
│   ├── StatCard (Users)
│   ├── StatCard (Revenue)
│   ├── StatCard (Orders)
│   └── StatCard (Conversion)
└── Content Grid (3 columns)
    ├── Recent Activity (2 cols)
    │   └── ActivityItem list
    └── Quick Actions (1 col)
        └── Button list
```

## Customization Ideas

1. **Add charts** - Line/bar charts for trends
2. **Real data** - Fetch from API
3. **Dark mode** - Toggle theme
4. **Filters** - Date range picker
5. **Export** - PDF/CSV downloads
6. **Notifications** - Toast messages
7. **Settings** - User preferences

## Advanced Patterns

```jounce
// Data fetching
let stats = signal({});
let loading = signal(true);

effect(() => {
    fetch("/api/stats")
        .then(res => res.json())
        .then(data => {
            stats.value = data;
            loading.value = false;
        });
});

// Auto-refresh
effect(() => {
    let interval = setInterval(() => {
        fetchStats();
    }, 30000);  // Every 30 seconds

    return () => clearInterval(interval);
});

// Derived metrics
let growthRate = computed(() => {
    return ((revenue.value - lastRevenue.value) / lastRevenue.value) * 100;
});
```

## Responsive Breakpoints

| Breakpoint | Width | Columns | Use Case |
|------------|-------|---------|----------|
| Mobile | < 640px | 1 | Stack vertically |
| Tablet (md) | ≥ 768px | 2 | Side by side |
| Desktop (lg) | ≥ 1024px | 3-4 | Full layout |

## Component Reusability

The StatCard and ActivityItem components can be reused throughout your app:

```jounce
// In another component
<StatCard
    title="Downloads"
    value="5,423"
    change="+15%"
    trend="up"
    icon="⬇️"
/>

<ActivityItem
    user="Admin"
    action="updated settings"
    time="Just now"
/>
```

## Learn More

- [Component Guide](../../docs/COMPONENTS.md)
- [Grid Layouts](../../docs/LAYOUTS.md)
- [CSS Utilities](../../docs/CSS_UTILITIES.md)

---

**Difficulty:** Intermediate
**Time:** 15 minutes
**Lines:** 140
**Components:** 3 (App, StatCard, ActivityItem)
