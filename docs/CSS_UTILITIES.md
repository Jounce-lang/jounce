# CSS Utilities Reference

**Version:** 1.0
**Total Classes:** 457
**File Size:** 19KB
**Auto-included:** Yes (in every compilation)

Jounce includes a comprehensive set of Tailwind-inspired utility classes for rapid UI development. All utilities are automatically included in `styles.css` with zero configuration required.

---

## 📚 Table of Contents

1. [Layout](#layout)
2. [Flexbox](#flexbox)
3. [Grid](#grid)
4. [Spacing](#spacing)
5. [Sizing](#sizing)
6. [Typography](#typography)
7. [Colors](#colors)
8. [Borders](#borders)
9. [Effects](#effects)
10. [Components](#components)
11. [Responsive](#responsive)

---

## Layout

### Display

```jounce
<div class="block">Block element</div>
<div class="inline-block">Inline-block</div>
<div class="inline">Inline</div>
<div class="flex">Flexbox container</div>
<div class="inline-flex">Inline flex</div>
<div class="grid">Grid container</div>
<div class="inline-grid">Inline grid</div>
<div class="hidden">Hidden (display: none)</div>
<div class="none">None</div>
<div class="contents">Contents</div>
```

### Position

```jounce
<div class="static">Static</div>
<div class="relative">Relative</div>
<div class="absolute">Absolute</div>
<div class="fixed">Fixed</div>
<div class="sticky">Sticky</div>
```

---

## Flexbox

### Direction

```jounce
<div class="flex flex-row">Row (default)</div>
<div class="flex flex-col">Column</div>
<div class="flex flex-row-reverse">Row reverse</div>
<div class="flex flex-col-reverse">Column reverse</div>
```

### Wrap

```jounce
<div class="flex flex-wrap">Wrap</div>
<div class="flex flex-nowrap">No wrap</div>
```

### Justify Content

```jounce
<div class="flex justify-start">Start</div>
<div class="flex justify-end">End</div>
<div class="flex justify-center">Center</div>
<div class="flex justify-between">Space between</div>
<div class="flex justify-around">Space around</div>
<div class="flex justify-evenly">Space evenly</div>
```

### Align Items

```jounce
<div class="flex items-start">Flex start</div>
<div class="flex items-end">Flex end</div>
<div class="flex items-center">Center</div>
<div class="flex items-baseline">Baseline</div>
<div class="flex items-stretch">Stretch</div>
```

### Align Self

```jounce
<div class="self-auto">Auto</div>
<div class="self-start">Start</div>
<div class="self-end">End</div>
<div class="self-center">Center</div>
<div class="self-stretch">Stretch</div>
```

### Gap

```jounce
<div class="flex gap-0">No gap</div>
<div class="flex gap-1">0.25rem gap</div>
<div class="flex gap-2">0.5rem gap</div>
<div class="flex gap-4">1rem gap</div>
<div class="flex gap-8">2rem gap</div>
<div class="flex gap-12">3rem gap</div>
<!-- Available: gap-0 through gap-12 -->
```

---

## Grid

### Grid Template Columns

```jounce
<div class="grid grid-cols-1">1 column</div>
<div class="grid grid-cols-2">2 columns</div>
<div class="grid grid-cols-3">3 columns</div>
<div class="grid grid-cols-4">4 columns</div>
<!-- Available: grid-cols-1 through grid-cols-12 -->
```

### Grid Template Rows

```jounce
<div class="grid grid-rows-1">1 row</div>
<div class="grid grid-rows-2">2 rows</div>
<div class="grid grid-rows-3">3 rows</div>
<!-- Available: grid-rows-1 through grid-rows-6 -->
```

### Column Span

```jounce
<div class="col-span-1">Span 1 column</div>
<div class="col-span-2">Span 2 columns</div>
<div class="col-span-6">Span 6 columns</div>
<div class="col-span-full">Span all columns</div>
<!-- Available: col-span-1 through col-span-12 -->
```

---

## Spacing

### Margin

```jounce
<!-- All sides -->
<div class="m-0">No margin</div>
<div class="m-4">1rem margin</div>
<div class="m-8">2rem margin</div>

<!-- Individual sides -->
<div class="mt-4">Margin top</div>
<div class="mr-4">Margin right</div>
<div class="mb-4">Margin bottom</div>
<div class="ml-4">Margin left</div>

<!-- Horizontal/Vertical -->
<div class="mx-4">Margin left+right</div>
<div class="my-4">Margin top+bottom</div>

<!-- Auto -->
<div class="mx-auto">Center horizontally</div>

<!-- Available: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 16 -->
<!-- Each number = 0.25rem (e.g., m-4 = 1rem) -->
```

### Padding

```jounce
<!-- All sides -->
<div class="p-0">No padding</div>
<div class="p-4">1rem padding</div>
<div class="p-8">2rem padding</div>

<!-- Individual sides -->
<div class="pt-4">Padding top</div>
<div class="pr-4">Padding right</div>
<div class="pb-4">Padding bottom</div>
<div class="pl-4">Padding left</div>

<!-- Horizontal/Vertical -->
<div class="px-4">Padding left+right</div>
<div class="py-4">Padding top+bottom</div>

<!-- Available: 0 through 16 (same as margin) -->
```

---

## Sizing

### Width

```jounce
<div class="w-full">100% width</div>
<div class="w-screen">100vw width</div>
<div class="w-min">min-content</div>
<div class="w-max">max-content</div>
<div class="w-fit">fit-content</div>

<!-- Fractional widths -->
<div class="w-1/12">8.33%</div>
<div class="w-2/12">16.67%</div>
<div class="w-6/12">50%</div>
<div class="w-12/12">100%</div>
```

### Height

```jounce
<div class="h-full">100% height</div>
<div class="h-screen">100vh height</div>
<div class="h-min">min-content</div>
<div class="h-max">max-content</div>
<div class="h-fit">fit-content</div>
```

### Max Width

```jounce
<div class="max-w-none">No max width</div>
<div class="max-w-xs">20rem (320px)</div>
<div class="max-w-sm">24rem (384px)</div>
<div class="max-w-md">28rem (448px)</div>
<div class="max-w-lg">32rem (512px)</div>
<div class="max-w-xl">36rem (576px)</div>
<div class="max-w-2xl">42rem (672px)</div>
<div class="max-w-4xl">56rem (896px)</div>
<div class="max-w-full">100%</div>
```

---

## Typography

### Font Size

```jounce
<p class="text-xs">Extra small (0.75rem)</p>
<p class="text-sm">Small (0.875rem)</p>
<p class="text-base">Base (1rem)</p>
<p class="text-lg">Large (1.125rem)</p>
<p class="text-xl">Extra large (1.25rem)</p>
<p class="text-2xl">2X large (1.5rem)</p>
<p class="text-3xl">3X large (1.875rem)</p>
<p class="text-4xl">4X large (2.25rem)</p>
<p class="text-5xl">5X large (3rem)</p>
```

### Font Weight

```jounce
<p class="font-thin">Thin (100)</p>
<p class="font-light">Light (300)</p>
<p class="font-normal">Normal (400)</p>
<p class="font-medium">Medium (500)</p>
<p class="font-semibold">Semibold (600)</p>
<p class="font-bold">Bold (700)</p>
```

### Text Alignment

```jounce
<p class="text-left">Left aligned</p>
<p class="text-center">Center aligned</p>
<p class="text-right">Right aligned</p>
<p class="text-justify">Justified</p>
```

### Line Height

```jounce
<p class="leading-none">1</p>
<p class="leading-tight">1.25</p>
<p class="leading-normal">1.5</p>
<p class="leading-relaxed">1.75</p>
<p class="leading-loose">2</p>
```

---

## Colors

### Text Colors

```jounce
<p class="text-primary">#0066cc</p>
<p class="text-secondary">#6c757d</p>
<p class="text-success">#28a745</p>
<p class="text-danger">#dc3545</p>
<p class="text-warning">#ffc107</p>
<p class="text-info">#17a2b8</p>
<p class="text-light">#f8f9fa</p>
<p class="text-dark">#343a40</p>
<p class="text-white">#ffffff</p>
<p class="text-black">#000000</p>
<p class="text-gray">#6c757d</p>
```

### Background Colors

```jounce
<div class="bg-primary">Primary background</div>
<div class="bg-secondary">Secondary background</div>
<div class="bg-success">Success background</div>
<div class="bg-danger">Danger background</div>
<div class="bg-warning">Warning background</div>
<div class="bg-info">Info background</div>
<div class="bg-light">Light background</div>
<div class="bg-dark">Dark background</div>
<div class="bg-white">White background</div>
<div class="bg-black">Black background</div>
```

### Border Colors

```jounce
<div class="border border-primary">Primary border</div>
<div class="border border-success">Success border</div>
<div class="border border-danger">Danger border</div>
<!-- Same color names as text/background -->
```

---

## Borders

### Border Width

```jounce
<div class="border">1px border</div>
<div class="border-0">No border</div>
<div class="border-2">2px border</div>
<div class="border-4">4px border</div>
```

### Border Radius

```jounce
<div class="rounded-none">No radius</div>
<div class="rounded-sm">0.125rem</div>
<div class="rounded">0.25rem</div>
<div class="rounded-md">0.375rem</div>
<div class="rounded-lg">0.5rem</div>
<div class="rounded-xl">0.75rem</div>
<div class="rounded-full">Fully rounded (circle/pill)</div>
```

---

## Effects

### Box Shadow

```jounce
<div class="shadow-none">No shadow</div>
<div class="shadow-sm">Small shadow</div>
<div class="shadow">Default shadow</div>
<div class="shadow-md">Medium shadow</div>
<div class="shadow-lg">Large shadow</div>
<div class="shadow-xl">Extra large shadow</div>
```

### Opacity

```jounce
<div class="opacity-0">0% (invisible)</div>
<div class="opacity-10">10%</div>
<div class="opacity-25">25%</div>
<div class="opacity-50">50%</div>
<div class="opacity-75">75%</div>
<div class="opacity-100">100% (fully visible)</div>
<!-- Available: 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100 -->
```

### Transition

```jounce
<div class="transition">All properties transition</div>
<div class="transition-none">No transition</div>
```

### Cursor

```jounce
<div class="cursor-auto">Auto cursor</div>
<div class="cursor-pointer">Pointer cursor</div>
<div class="cursor-not-allowed">Not allowed cursor</div>
```

---

## Components

### Container

```jounce
<div class="container">
    Centered, max-width container with padding
</div>
```

### Buttons

```jounce
<!-- Button variants -->
<button class="btn btn-primary">Primary</button>
<button class="btn btn-secondary">Secondary</button>
<button class="btn btn-success">Success</button>
<button class="btn btn-danger">Danger</button>

<!-- Button sizes -->
<button class="btn btn-sm">Small</button>
<button class="btn">Default</button>
<button class="btn btn-lg">Large</button>

<!-- Combined -->
<button class="btn btn-primary btn-lg rounded">
    Large Primary Button
</button>
```

### Card

```jounce
<div class="card">
    Card with white background, border, and shadow
</div>
```

### Badge

```jounce
<span class="badge bg-primary text-white">NEW</span>
<span class="badge bg-success">Active</span>
```

---

## Responsive

### Breakpoints

- **sm:** 640px and up
- **md:** 768px and up
- **lg:** 1024px and up

### Usage

```jounce
<!-- Hidden on mobile, visible on medium screens and up -->
<div class="hidden md:block">
    Only visible on tablets and desktops
</div>

<!-- Stacked on mobile, side-by-side on desktop -->
<div class="flex flex-col lg:flex-row">
    <div>Item 1</div>
    <div>Item 2</div>
</div>

<!-- Currently supported -->
<div class="sm:block">Show on small screens+</div>
<div class="sm:hidden">Hide on small screens+</div>
<div class="md:flex">Flex on medium screens+</div>
<div class="lg:grid">Grid on large screens+</div>
```

---

## Examples

### Centered Card with Button

```jounce
<div class="container mx-auto p-8">
    <div class="card p-6 shadow-lg rounded-lg max-w-md mx-auto">
        <h1 class="text-2xl font-bold text-center mb-4">
            Welcome
        </h1>
        <p class="text-gray text-center mb-6">
            Get started with Jounce utilities
        </p>
        <button class="btn btn-primary w-full rounded">
            Get Started
        </button>
    </div>
</div>
```

### Responsive Grid

```jounce
<div class="container mx-auto p-4">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div class="card p-4 shadow">Card 1</div>
        <div class="card p-4 shadow">Card 2</div>
        <div class="card p-4 shadow">Card 3</div>
    </div>
</div>
```

### Flexbox Layout

```jounce
<div class="flex justify-between items-center p-4 bg-dark text-white">
    <div class="font-bold">Logo</div>
    <div class="flex gap-4">
        <a class="text-light">Home</a>
        <a class="text-light">About</a>
        <a class="text-light">Contact</a>
    </div>
</div>
```

---

## Performance

- **Total Classes:** 457
- **File Size:** 19KB (uncompressed)
- **Gzipped:** ~5KB (estimated)
- **Load Time:** < 10ms on modern browsers
- **Zero Runtime Cost:** Pure CSS, no JavaScript

---

## Tips

1. **Combine utilities** for complex layouts:
   ```jounce
   <div class="flex items-center justify-between p-4 bg-light rounded shadow">
   ```

2. **Use components** for common patterns:
   ```jounce
   <button class="btn btn-primary btn-lg">
   ```

3. **Responsive design** with breakpoint prefixes:
   ```jounce
   <div class="block md:flex lg:grid">
   ```

4. **Custom CSS** still works! Add your own styles alongside utilities.

---

## Next Steps

- Check out [examples/apps/34-utility-classes-test/](../examples/apps/34-utility-classes-test/) for a live demo
- All utilities are automatically included in every Jounce compilation
- Zero configuration required - just start using them!

**Happy styling! 🎨**
