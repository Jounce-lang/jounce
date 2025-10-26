# App 10: Expense Tracker 💰

**Complexity**: Medium
**Lines**: ~155
**Packages**: None (UI demo - jounce-db integration coming soon!)
**Time to Build**: 60 minutes

---

## 📖 Description

A comprehensive expense tracking application featuring:
- **Summary Cards**: Total income, expenses, and balance
- **Transaction List**: Recent transactions with categories
- **Category Icons**: Visual categorization with emojis
- **Budget Tracking**: Category budgets with progress bars
- **Over-Budget Alerts**: Red warnings for exceeded budgets
- **Responsive Layout**: Two-column design (desktop) to single-column (mobile)

---

## ✨ Features

- ✅ 3 summary cards (income, expenses, balance)
- ✅ 5 recent transactions with icons
- ✅ Transaction categories (food, salary, transport, shopping, entertainment)
- ✅ Income (+) and expense (-) color coding (green/red)
- ✅ 4 category budgets with progress bars
- ✅ Over-budget warning (entertainment category)
- ✅ Responsive grid layout
- ✅ Hover animations on transactions

---

## 🚀 Quick Start

```bash
# Compile
cargo run -- compile examples/apps/10-expense-tracker/main.jnc

# Run
cd dist && node server.js
# Open http://localhost:3000
```

---

## 🎯 What This App Tests

### Language Features
- [x] **Dashboard layout** - Multi-section interface
- [x] **Data cards** - Summary and transaction cards
- [x] **Progress bars** - Budget tracking visualization

### UI Patterns
- [x] **Two-column layout** - Transactions + budgets
- [x] **List items** - Transaction history
- [x] **Icon categories** - Emoji-based categorization
- [x] **Conditional styling** - Income/expense colors
- [x] **Progress indicators** - Budget bars
- [x] **Warning states** - Over-budget alerts

---

## 💡 Key Concepts

### 1. Summary Cards Grid

```css
.summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
}
```

Auto-fit creates responsive card layout.

### 2. Income/Expense Color Coding

```jounce
<div class="transaction-item expense">
    <div class="transaction-amount">-$45.00</div>
</div>
```

Red for expenses, green for income.

### 3. Budget Progress Bars

```css
.budget-progress {
    width: 48%;
    background: linear-gradient(90deg, #10b981 0%, #059669 100%);
}
```

Dynamic width shows budget usage percentage.

---

## 🔄 Future Enhancements

- [ ] Add transaction with form
- [ ] Delete transactions
- [ ] Edit budgets
- [ ] Category filtering
- [ ] Date range selection
- [ ] Export to CSV
- [ ] Charts and graphs
- [ ] Recurring transactions
- [ ] Multiple accounts

---

## ✅ Success Criteria

- [x] Compiles without errors
- [x] Summary cards display
- [x] Transactions render with icons
- [x] Income/expense colors work
- [x] Budget bars show correctly
- [x] Over-budget warning displays
- [x] Responsive on mobile
- [x] Hover effects work

---

**Status**: ✅ Complete (UI Demo)
**Date**: October 25, 2025
**Jounce Version**: v0.8.0
