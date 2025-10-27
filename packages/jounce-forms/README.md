# jounce-forms

Form handling with validation, fields, and reactive state management.

## Features

- ✅ Reactive form state with signals
- ✅ Built-in validation rules
- ✅ Field-level validation
- ✅ Error messages and touched state
- ✅ Submit handling with loading states
- ✅ Input, textarea, select, checkbox fields
- ✅ Common validators (email, required, min/max length, pattern, numeric, URL)

## Quick Start

```jounce
use jounce::forms::*;

fn main() {
    // Create form with submit handler
    let form = Form::new((values) => {
        console::log("Form submitted:", values);
    })
    .with_validator("email", email_validator())
    .with_validator("password", min_length_validator(8, "Password"));

    // Render form
    mountComponent(
        <form onSubmit={(e) => form.handle_submit(e)}>
            {input_field(form, "email", "Email", "email")}
            {input_field(form, "password", "Password", "password")}
            {submit_button(form, "Sign Up")}
        </form>
    );
}
```

## API

### Form

- `Form::new(on_submit)` - Create new form
- `form.with_validator(field, rule)` - Add validation rule
- `form.handle_change(field, value)` - Handle field change
- `form.handle_blur(field)` - Handle field blur
- `form.handle_submit(event)` - Handle form submission
- `form.validate_all()` - Validate all fields
- `form.reset()` - Reset form state

### Field Helpers

- `input_field(form, name, label, type)` - Text/email/password input
- `textarea_field(form, name, label, rows)` - Textarea
- `select_field(form, name, label, options)` - Select dropdown
- `checkbox_field(form, name, label)` - Checkbox
- `submit_button(form, label)` - Submit button

### Validators

- `email_validator()` - Email validation
- `required_validator(field_name)` - Required field
- `min_length_validator(min, field_name)` - Minimum length
- `max_length_validator(max, field_name)` - Maximum length
- `pattern_validator(pattern, message)` - Regex pattern
- `numeric_validator()` - Numeric values
- `url_validator()` - URL validation

## Example

See `examples/forms-demo/main.jnc` for complete example.
