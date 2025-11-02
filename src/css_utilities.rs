/// CSS Utility Class Generator
/// Generates a Tailwind-inspired utility class library
/// Usage: Auto-included in all Jounce compilations

use std::fmt::Write;

/// Generate all utility CSS classes
pub fn generate_utilities() -> String {
    let mut css = String::with_capacity(50_000);

    // Header comment
    writeln!(css, "/* Jounce CSS Utilities v1.0 */").unwrap();
    writeln!(css, "/* Auto-generated utility classes */\n").unwrap();

    // Reset and base styles
    write_reset(&mut css);

    // Layout utilities
    write_display_utilities(&mut css);
    write_position_utilities(&mut css);
    write_flexbox_utilities(&mut css);
    write_grid_utilities(&mut css);

    // Spacing utilities
    write_spacing_utilities(&mut css);

    // Sizing utilities
    write_sizing_utilities(&mut css);

    // Typography utilities
    write_typography_utilities(&mut css);

    // Color utilities
    write_color_utilities(&mut css);

    // Border utilities
    write_border_utilities(&mut css);

    // Effect utilities
    write_effect_utilities(&mut css);

    // Component utilities
    write_component_utilities(&mut css);

    // Responsive utilities
    write_responsive_utilities(&mut css);

    css
}

/// Basic CSS reset
fn write_reset(css: &mut String) {
    writeln!(css, "/* Reset */").unwrap();
    writeln!(css, "*, *::before, *::after {{").unwrap();
    writeln!(css, "  box-sizing: border-box;").unwrap();
    writeln!(css, "  margin: 0;").unwrap();
    writeln!(css, "  padding: 0;").unwrap();
    writeln!(css, "}}\n").unwrap();
}

/// Display utilities (flex, block, inline, grid, etc.)
fn write_display_utilities(css: &mut String) {
    writeln!(css, "/* Display */").unwrap();

    let displays = [
        "block", "inline-block", "inline", "flex", "inline-flex",
        "grid", "inline-grid", "none", "contents"
    ];

    for display in displays {
        writeln!(css, ".{} {{ display: {}; }}", display, display).unwrap();
    }

    writeln!(css, ".hidden {{ display: none; }}").unwrap();
    writeln!(css).unwrap();
}

/// Position utilities
fn write_position_utilities(css: &mut String) {
    writeln!(css, "/* Position */").unwrap();

    let positions = ["static", "relative", "absolute", "fixed", "sticky"];
    for pos in positions {
        writeln!(css, ".{} {{ position: {}; }}", pos, pos).unwrap();
    }

    writeln!(css).unwrap();
}

/// Flexbox utilities
fn write_flexbox_utilities(css: &mut String) {
    writeln!(css, "/* Flexbox */").unwrap();

    // Flex direction
    writeln!(css, ".flex-row {{ flex-direction: row; }}").unwrap();
    writeln!(css, ".flex-col {{ flex-direction: column; }}").unwrap();
    writeln!(css, ".flex-row-reverse {{ flex-direction: row-reverse; }}").unwrap();
    writeln!(css, ".flex-col-reverse {{ flex-direction: column-reverse; }}").unwrap();

    // Flex wrap
    writeln!(css, ".flex-wrap {{ flex-wrap: wrap; }}").unwrap();
    writeln!(css, ".flex-nowrap {{ flex-wrap: nowrap; }}").unwrap();

    // Justify content
    writeln!(css, ".justify-start {{ justify-content: flex-start; }}").unwrap();
    writeln!(css, ".justify-end {{ justify-content: flex-end; }}").unwrap();
    writeln!(css, ".justify-center {{ justify-content: center; }}").unwrap();
    writeln!(css, ".justify-between {{ justify-content: space-between; }}").unwrap();
    writeln!(css, ".justify-around {{ justify-content: space-around; }}").unwrap();
    writeln!(css, ".justify-evenly {{ justify-content: space-evenly; }}").unwrap();

    // Align items
    writeln!(css, ".items-start {{ align-items: flex-start; }}").unwrap();
    writeln!(css, ".items-end {{ align-items: flex-end; }}").unwrap();
    writeln!(css, ".items-center {{ align-items: center; }}").unwrap();
    writeln!(css, ".items-baseline {{ align-items: baseline; }}").unwrap();
    writeln!(css, ".items-stretch {{ align-items: stretch; }}").unwrap();

    // Align self
    writeln!(css, ".self-auto {{ align-self: auto; }}").unwrap();
    writeln!(css, ".self-start {{ align-self: flex-start; }}").unwrap();
    writeln!(css, ".self-end {{ align-self: flex-end; }}").unwrap();
    writeln!(css, ".self-center {{ align-self: center; }}").unwrap();
    writeln!(css, ".self-stretch {{ align-self: stretch; }}").unwrap();

    // Gap
    for i in 0..=12 {
        let rem = i as f32 * 0.25;
        writeln!(css, ".gap-{} {{ gap: {}rem; }}", i, rem).unwrap();
    }

    writeln!(css).unwrap();
}

/// Grid utilities
fn write_grid_utilities(css: &mut String) {
    writeln!(css, "/* Grid */").unwrap();

    // Grid columns
    for i in 1..=12 {
        writeln!(css, ".grid-cols-{} {{ grid-template-columns: repeat({}, minmax(0, 1fr)); }}", i, i).unwrap();
    }

    // Grid rows
    for i in 1..=6 {
        writeln!(css, ".grid-rows-{} {{ grid-template-rows: repeat({}, minmax(0, 1fr)); }}", i, i).unwrap();
    }

    // Column span
    for i in 1..=12 {
        writeln!(css, ".col-span-{} {{ grid-column: span {} / span {}; }}", i, i, i).unwrap();
    }
    writeln!(css, ".col-span-full {{ grid-column: 1 / -1; }}").unwrap();

    writeln!(css).unwrap();
}

/// Spacing utilities (margin and padding)
fn write_spacing_utilities(css: &mut String) {
    writeln!(css, "/* Spacing */").unwrap();

    for i in 0..=16 {
        let rem = i as f32 * 0.25;

        // Margin
        writeln!(css, ".m-{} {{ margin: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".mt-{} {{ margin-top: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".mr-{} {{ margin-right: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".mb-{} {{ margin-bottom: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".ml-{} {{ margin-left: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".mx-{} {{ margin-left: {}rem; margin-right: {}rem; }}", i, rem, rem).unwrap();
        writeln!(css, ".my-{} {{ margin-top: {}rem; margin-bottom: {}rem; }}", i, rem, rem).unwrap();

        // Padding
        writeln!(css, ".p-{} {{ padding: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".pt-{} {{ padding-top: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".pr-{} {{ padding-right: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".pb-{} {{ padding-bottom: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".pl-{} {{ padding-left: {}rem; }}", i, rem).unwrap();
        writeln!(css, ".px-{} {{ padding-left: {}rem; padding-right: {}rem; }}", i, rem, rem).unwrap();
        writeln!(css, ".py-{} {{ padding-top: {}rem; padding-bottom: {}rem; }}", i, rem, rem).unwrap();
    }

    // Auto margins
    writeln!(css, ".m-auto {{ margin: auto; }}").unwrap();
    writeln!(css, ".mx-auto {{ margin-left: auto; margin-right: auto; }}").unwrap();
    writeln!(css, ".my-auto {{ margin-top: auto; margin-bottom: auto; }}").unwrap();

    writeln!(css).unwrap();
}

/// Sizing utilities (width and height)
fn write_sizing_utilities(css: &mut String) {
    writeln!(css, "/* Sizing */").unwrap();

    // Common sizes
    let sizes = [
        ("full", "100%"),
        ("screen", "100vh"),
        ("min", "min-content"),
        ("max", "max-content"),
        ("fit", "fit-content"),
    ];

    for (name, value) in sizes {
        writeln!(css, ".w-{} {{ width: {}; }}", name, value).unwrap();
        writeln!(css, ".h-{} {{ height: {}; }}", name, value).unwrap();
    }

    // Fractional sizes
    for i in 1..=12 {
        let percent = (i as f32 / 12.0) * 100.0;
        writeln!(css, ".w-{}/12 {{ width: {}%; }}", i, percent).unwrap();
    }

    // Min/Max sizes
    writeln!(css, ".min-w-0 {{ min-width: 0; }}").unwrap();
    writeln!(css, ".min-h-0 {{ min-height: 0; }}").unwrap();
    writeln!(css, ".max-w-none {{ max-width: none; }}").unwrap();
    writeln!(css, ".max-w-xs {{ max-width: 20rem; }}").unwrap();
    writeln!(css, ".max-w-sm {{ max-width: 24rem; }}").unwrap();
    writeln!(css, ".max-w-md {{ max-width: 28rem; }}").unwrap();
    writeln!(css, ".max-w-lg {{ max-width: 32rem; }}").unwrap();
    writeln!(css, ".max-w-xl {{ max-width: 36rem; }}").unwrap();
    writeln!(css, ".max-w-2xl {{ max-width: 42rem; }}").unwrap();
    writeln!(css, ".max-w-4xl {{ max-width: 56rem; }}").unwrap();
    writeln!(css, ".max-w-full {{ max-width: 100%; }}").unwrap();

    writeln!(css).unwrap();
}

/// Typography utilities
fn write_typography_utilities(css: &mut String) {
    writeln!(css, "/* Typography */").unwrap();

    // Font sizes
    let sizes = [
        ("xs", "0.75rem"),
        ("sm", "0.875rem"),
        ("base", "1rem"),
        ("lg", "1.125rem"),
        ("xl", "1.25rem"),
        ("2xl", "1.5rem"),
        ("3xl", "1.875rem"),
        ("4xl", "2.25rem"),
        ("5xl", "3rem"),
    ];

    for (name, size) in sizes {
        writeln!(css, ".text-{} {{ font-size: {}; }}", name, size).unwrap();
    }

    // Font weights
    writeln!(css, ".font-thin {{ font-weight: 100; }}").unwrap();
    writeln!(css, ".font-light {{ font-weight: 300; }}").unwrap();
    writeln!(css, ".font-normal {{ font-weight: 400; }}").unwrap();
    writeln!(css, ".font-medium {{ font-weight: 500; }}").unwrap();
    writeln!(css, ".font-semibold {{ font-weight: 600; }}").unwrap();
    writeln!(css, ".font-bold {{ font-weight: 700; }}").unwrap();

    // Text alignment
    writeln!(css, ".text-left {{ text-align: left; }}").unwrap();
    writeln!(css, ".text-center {{ text-align: center; }}").unwrap();
    writeln!(css, ".text-right {{ text-align: right; }}").unwrap();
    writeln!(css, ".text-justify {{ text-align: justify; }}").unwrap();

    // Line height
    writeln!(css, ".leading-none {{ line-height: 1; }}").unwrap();
    writeln!(css, ".leading-tight {{ line-height: 1.25; }}").unwrap();
    writeln!(css, ".leading-normal {{ line-height: 1.5; }}").unwrap();
    writeln!(css, ".leading-relaxed {{ line-height: 1.75; }}").unwrap();
    writeln!(css, ".leading-loose {{ line-height: 2; }}").unwrap();

    writeln!(css).unwrap();
}

/// Color utilities
fn write_color_utilities(css: &mut String) {
    writeln!(css, "/* Colors */").unwrap();

    let colors = [
        ("primary", "#0066cc"),
        ("secondary", "#6c757d"),
        ("success", "#28a745"),
        ("danger", "#dc3545"),
        ("warning", "#ffc107"),
        ("info", "#17a2b8"),
        ("light", "#f8f9fa"),
        ("dark", "#343a40"),
        ("white", "#ffffff"),
        ("black", "#000000"),
        ("gray", "#6c757d"),
    ];

    for (name, color) in colors {
        writeln!(css, ".text-{} {{ color: {}; }}", name, color).unwrap();
        writeln!(css, ".bg-{} {{ background-color: {}; }}", name, color).unwrap();
        writeln!(css, ".border-{} {{ border-color: {}; }}", name, color).unwrap();
    }

    writeln!(css).unwrap();
}

/// Border utilities
fn write_border_utilities(css: &mut String) {
    writeln!(css, "/* Borders */").unwrap();

    // Border width
    writeln!(css, ".border {{ border-width: 1px; border-style: solid; }}").unwrap();
    writeln!(css, ".border-0 {{ border-width: 0; }}").unwrap();
    writeln!(css, ".border-2 {{ border-width: 2px; border-style: solid; }}").unwrap();
    writeln!(css, ".border-4 {{ border-width: 4px; border-style: solid; }}").unwrap();

    // Border radius
    writeln!(css, ".rounded-none {{ border-radius: 0; }}").unwrap();
    writeln!(css, ".rounded-sm {{ border-radius: 0.125rem; }}").unwrap();
    writeln!(css, ".rounded {{ border-radius: 0.25rem; }}").unwrap();
    writeln!(css, ".rounded-md {{ border-radius: 0.375rem; }}").unwrap();
    writeln!(css, ".rounded-lg {{ border-radius: 0.5rem; }}").unwrap();
    writeln!(css, ".rounded-xl {{ border-radius: 0.75rem; }}").unwrap();
    writeln!(css, ".rounded-full {{ border-radius: 9999px; }}").unwrap();

    writeln!(css).unwrap();
}

/// Effect utilities (shadow, opacity, etc.)
fn write_effect_utilities(css: &mut String) {
    writeln!(css, "/* Effects */").unwrap();

    // Box shadow
    writeln!(css, ".shadow-none {{ box-shadow: none; }}").unwrap();
    writeln!(css, ".shadow-sm {{ box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05); }}").unwrap();
    writeln!(css, ".shadow {{ box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06); }}").unwrap();
    writeln!(css, ".shadow-md {{ box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06); }}").unwrap();
    writeln!(css, ".shadow-lg {{ box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05); }}").unwrap();
    writeln!(css, ".shadow-xl {{ box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04); }}").unwrap();

    // Opacity
    for i in (0..=100).step_by(10) {
        let opacity = i as f32 / 100.0;
        writeln!(css, ".opacity-{} {{ opacity: {}; }}", i, opacity).unwrap();
    }

    // Transition
    writeln!(css, ".transition {{ transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms; }}").unwrap();
    writeln!(css, ".transition-none {{ transition-property: none; }}").unwrap();

    // Cursor
    writeln!(css, ".cursor-auto {{ cursor: auto; }}").unwrap();
    writeln!(css, ".cursor-pointer {{ cursor: pointer; }}").unwrap();
    writeln!(css, ".cursor-not-allowed {{ cursor: not-allowed; }}").unwrap();

    writeln!(css).unwrap();
}

/// Component utilities (buttons, cards, etc.)
fn write_component_utilities(css: &mut String) {
    writeln!(css, "/* Components */").unwrap();

    // Container
    writeln!(css, ".container {{ width: 100%; max-width: 1200px; margin-left: auto; margin-right: auto; padding-left: 1rem; padding-right: 1rem; }}").unwrap();

    // Buttons
    writeln!(css, ".btn {{").unwrap();
    writeln!(css, "  display: inline-block;").unwrap();
    writeln!(css, "  padding: 0.5rem 1rem;").unwrap();
    writeln!(css, "  font-weight: 500;").unwrap();
    writeln!(css, "  text-align: center;").unwrap();
    writeln!(css, "  border: 1px solid transparent;").unwrap();
    writeln!(css, "  border-radius: 0.25rem;").unwrap();
    writeln!(css, "  cursor: pointer;").unwrap();
    writeln!(css, "  transition: all 0.15s;").unwrap();
    writeln!(css, "}}").unwrap();

    writeln!(css, ".btn-primary {{ background-color: #0066cc; color: white; border-color: #0066cc; }}").unwrap();
    writeln!(css, ".btn-primary:hover {{ background-color: #0052a3; }}").unwrap();
    writeln!(css, ".btn-secondary {{ background-color: #6c757d; color: white; border-color: #6c757d; }}").unwrap();
    writeln!(css, ".btn-secondary:hover {{ background-color: #5a6268; }}").unwrap();
    writeln!(css, ".btn-success {{ background-color: #28a745; color: white; border-color: #28a745; }}").unwrap();
    writeln!(css, ".btn-danger {{ background-color: #dc3545; color: white; border-color: #dc3545; }}").unwrap();

    writeln!(css, ".btn-sm {{ padding: 0.25rem 0.5rem; font-size: 0.875rem; }}").unwrap();
    writeln!(css, ".btn-lg {{ padding: 0.75rem 1.5rem; font-size: 1.125rem; }}").unwrap();

    // Card
    writeln!(css, ".card {{").unwrap();
    writeln!(css, "  background-color: white;").unwrap();
    writeln!(css, "  border: 1px solid #e1e4e8;").unwrap();
    writeln!(css, "  border-radius: 0.5rem;").unwrap();
    writeln!(css, "  padding: 1.5rem;").unwrap();
    writeln!(css, "  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);").unwrap();
    writeln!(css, "}}").unwrap();

    // Badge
    writeln!(css, ".badge {{").unwrap();
    writeln!(css, "  display: inline-block;").unwrap();
    writeln!(css, "  padding: 0.25rem 0.5rem;").unwrap();
    writeln!(css, "  font-size: 0.75rem;").unwrap();
    writeln!(css, "  font-weight: 600;").unwrap();
    writeln!(css, "  line-height: 1;").unwrap();
    writeln!(css, "  border-radius: 0.25rem;").unwrap();
    writeln!(css, "}}").unwrap();

    writeln!(css).unwrap();
}

/// Responsive utilities
fn write_responsive_utilities(css: &mut String) {
    writeln!(css, "/* Responsive */").unwrap();

    // Mobile-first responsive breakpoints
    writeln!(css, "@media (min-width: 640px) {{").unwrap();
    writeln!(css, "  .sm\\:block {{ display: block; }}").unwrap();
    writeln!(css, "  .sm\\:hidden {{ display: none; }}").unwrap();
    writeln!(css, "  .sm\\:flex {{ display: flex; }}").unwrap();
    writeln!(css, "}}").unwrap();

    writeln!(css, "@media (min-width: 768px) {{").unwrap();
    writeln!(css, "  .md\\:block {{ display: block; }}").unwrap();
    writeln!(css, "  .md\\:hidden {{ display: none; }}").unwrap();
    writeln!(css, "  .md\\:flex {{ display: flex; }}").unwrap();
    writeln!(css, "}}").unwrap();

    writeln!(css, "@media (min-width: 1024px) {{").unwrap();
    writeln!(css, "  .lg\\:block {{ display: block; }}").unwrap();
    writeln!(css, "  .lg\\:hidden {{ display: none; }}").unwrap();
    writeln!(css, "  .lg\\:flex {{ display: flex; }}").unwrap();
    writeln!(css, "}}").unwrap();

    writeln!(css).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_utilities() {
        let css = generate_utilities();

        // Should contain various utility classes
        assert!(css.contains(".flex"));
        assert!(css.contains(".grid"));
        assert!(css.contains(".m-4"));
        assert!(css.contains(".p-8"));
        assert!(css.contains(".text-center"));
        assert!(css.contains(".btn-primary"));
        assert!(css.contains(".shadow-lg"));

        // Should not be empty
        assert!(css.len() > 10_000);
    }

    #[test]
    fn test_spacing_utilities() {
        let css = generate_utilities();

        // Check margin utilities exist
        assert!(css.contains(".m-0"));
        assert!(css.contains(".m-4"));
        assert!(css.contains(".mx-auto"));
        assert!(css.contains(".my-8"));

        // Check padding utilities exist
        assert!(css.contains(".p-0"));
        assert!(css.contains(".p-4"));
        assert!(css.contains(".px-6"));
        assert!(css.contains(".py-3"));
    }
}
