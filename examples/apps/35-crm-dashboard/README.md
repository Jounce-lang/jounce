# Sales CRM Dashboard

A complete SaaS sales representative dashboard built with Jounce, demonstrating real-world business application patterns.

## Features

### 📊 **Real-Time Metrics Dashboard**
- **Closing Ratio**: Percentage of deals won vs total deals
- **Total Deals**: Count of all deals in the pipeline
- **Won Deals**: Number of successfully closed deals
- **Total Revenue**: Sum of all won deal values

### 💼 **Deal Management**
- View all deals with company name, value, contact, and status
- Mark deals as "Won" or "Lost" with interactive buttons
- Real-time UI updates using Jounce's reactive system
- Visual feedback with hover effects and transitions

### 🎨 **Professional UI**
- Modern card-based layout
- Responsive grid system
- Smooth animations and transitions
- Color-coded status indicators

## Jounce Features Demonstrated

### ✅ **Computed Values**
```jounce
let closingRatio = computed(() => {
    let total = deals.value.length;
    let won = deals.value.filter((deal) => {
        return deal.status == "won";
    }).length;
    return Math.floor((won / total) * 100);
});
```

### ✅ **Array Methods**
- `.filter()` - Filter deals by status
- `.map()` - Transform arrays
- `.reduce()` - Calculate totals

### ✅ **Event Handlers with Parameters**
```jounce
fn markWon(dealId: i32) {
    deals.value = deals.value.map((deal) => {
        if (deal.id == dealId) {
            return { ...deal, status: "won" };
        }
        return deal;
    });
}
```

### ✅ **Conditional Rendering**
```jounce
{deal.status == "open" ? (
    <div>
        <button onClick={() => markWon(deal.id)}>Mark Won</button>
        <button onClick={() => markLost(deal.id)}>Mark Lost</button>
    </div>
) : null}
```

### ✅ **Top-Level Style Blocks**
Jounce uses Svelte-style top-level CSS blocks:
```jounce
style {
    .metric-card {
        background: white;
        border-radius: 12px;
        transition: transform 0.2s;
    }

    .metric-card:hover {
        transform: translateY(-4px);
    }
}
```

**Note**: Jounce uses `style { }` blocks, NOT `<style>` JSX tags. This enables:
- Pure CSS syntax (no escaping needed)
- Static CSS extraction (no runtime overhead)
- Standard CSS features (media queries, pseudo-classes, etc.)

## Running the Example

### Compile
```bash
cargo run --release -- compile examples/apps/35-crm-dashboard/main.jnc
```

### Run
```bash
cd dist
python3 -m http.server 3000
```

Then open [http://localhost:3000](http://localhost:3000)

## Code Structure

```
main.jnc
├── style { }              - CSS styling (top-level block)
├── component CRMDashboard
│   ├── deals signal       - Reactive data store
│   ├── computed metrics   - Auto-updating calculations
│   ├── event handlers     - User interactions
│   └── JSX template       - UI structure
└── component App          - Entry point
```

## Learning Points

### 1. **Reactive Data Flow**
Changes to `deals.value` automatically trigger:
- Computed value recalculation (`closingRatio`, `wonDeals`, etc.)
- UI re-rendering of affected components
- No manual DOM manipulation needed

### 2. **Immutable Updates**
```jounce
// ✅ Correct - Create new array
deals.value = deals.value.map(...)

// ❌ Wrong - Mutating signal won't trigger updates
deals.value[0].status = "won"
```

### 3. **Type Annotations Required**
```jounce
// ✅ Correct
fn markWon(dealId: i32) { ... }

// ❌ Wrong - Jounce requires explicit types
fn markWon(dealId) { ... }
```

### 4. **Object Literals in Jounce**
When updating objects, recreate them explicitly:
```jounce
return {
    id: deal.id,
    company: deal.company,
    value: deal.value,
    status: "won",
    contact: deal.contact
};
```

## Possible Enhancements

Want to extend this example? Try adding:

1. **Search/Filter** - Filter deals by company name or status
2. **Sort Options** - Sort by value, company name, or status
3. **Add New Deals** - Form to create new deals
4. **Deal Details** - Modal with full deal information
5. **Date Tracking** - Add created/closed dates
6. **Revenue Graph** - Visual chart of revenue over time
7. **Export to CSV** - Download deals data

## Related Examples

- **11-todo-list** - Similar interactive list pattern
- **10-expense-tracker** - Financial tracking with totals
- **26-user-profile** - Object manipulation patterns

## Build Info

- **Jounce Version**: v0.8.2+
- **Compiler Fix**: Component mounting priority (mounts `App` correctly)
- **Lines of Code**: ~290 (including styles and comments)
- **Single File**: ✅ All code and styles in one `.jnc` file
