# Form Validation Template

A complete form with real-time validation built with Jounce.

## What's Included

- ✅ Multi-field form (name, email, password)
- ✅ Real-time validation
- ✅ Error messages
- ✅ Submit handling
- ✅ Success state
- ✅ Form reset

## Features

**Validation Rules**:
- Name: Required, min 2 characters
- Email: Required, must contain @
- Password: Required, min 8 characters

**User Experience**:
- Real-time error messages
- Disabled submit button when invalid
- Success confirmation
- Form reset after submission

**Technical Concepts**:
- Computed values for validation
- Form event handling (onSubmit, preventDefault)
- Two-way data binding
- Conditional rendering
- Dynamic CSS classes

## Getting Started

```bash
# Compile the app
jnc compile main.jnc

# Run development server
jnc dev

# Open http://localhost:3000
```

## What You'll Learn

- Form handling in Jounce
- Real-time validation with computed values
- Error message display
- Form submission and preventDefault
- Success/error states
- Two-way binding with inputs

## Customize It!

Try these challenges:

1. **Add more fields** - Phone, address, etc.
2. **Add regex validation** - Strong password rules
3. **Add async validation** - Check if email exists
4. **Add field masking** - Format phone numbers
5. **Add multi-step form** - Wizard with steps
6. **Add file upload** - Profile picture
7. **Add autocomplete** - Suggest addresses

## Advanced Features

- Password strength meter
- Confirm password field
- Remember me checkbox
- Terms & conditions
- Social login buttons

## Learn More

- [Forms Guide](https://docs.jounce.dev/guide/forms)
- [Validation Patterns](https://docs.jounce.dev/cookbook/validation)
- [Computed Values](https://docs.jounce.dev/guide/computed)
- [Tutorial](https://tutorial.jounce.dev)

## Deploy

```bash
# Deploy to production
jnc deploy --platform vercel
```

**Happy form building!** 🚀
