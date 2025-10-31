# Form App Template

**Production-ready form handling** - Learn validation, error states, and form submission!

## Features

✅ **Multi-field forms** - Text, email, password, checkbox inputs
✅ **Real-time validation** - Validate on blur
✅ **Error messages** - Clear, helpful feedback
✅ **Submit handling** - Prevent default, show loading
✅ **Success state** - Confirmation message
✅ **Form reset** - Clear all fields
✅ **Conditional styling** - Error borders and messages

## Quick Start

```bash
# 1. Copy this template
cp -r templates/form-app my-form-app
cd my-form-app

# 2. Compile and run
jnc compile main.jnc
cd dist && node server.js

# 3. Open browser
open http://localhost:3000
```

## What You'll Learn

### 1. Form Validation
```jounce
let validateEmail = || {
    if (email.value == "") {
        emailError.value = "Email is required";
        return false;
    }
    let emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(email.value)) {
        emailError.value = "Please enter a valid email";
        return false;
    }
    emailError.value = "";
    return true;
};
```

### 2. Form Submission
```jounce
let handleSubmit = |e| {
    e.preventDefault();  // Prevent page reload

    // Validate all fields
    let isValid = validateName() &&
                 validateEmail() &&
                 validatePassword();

    if (isValid) {
        // Submit form
        submitToAPI();
    }
};
```

### 3. Input Handling
```jounce
<input
    type="text"
    value={name.value}
    oninput={(e) => name.value = e.target.value}
    onblur={validateName}  // Validate when user leaves field
/>
```

### 4. Error Display
```jounce
<input
    class={`border-2 ${nameError.value ? "border-danger" : "border-gray"}`}
/>
{nameError.value && (
    <p class="text-danger text-sm">
        {nameError.value}
    </p>
)}
```

### 5. Loading States
```jounce
<button disabled={submitting.value}>
    {submitting.value ? "Submitting..." : "Submit"}
</button>
```

### 6. Success State
```jounce
{submitted.value ? (
    <div>Success message</div>
) : (
    <form>...</form>
)}
```

## Form Fields

| Field | Type | Validation |
|-------|------|------------|
| Name | text | Required, min 2 chars |
| Email | email | Required, valid format |
| Password | password | Required, min 8 chars |
| Confirm | password | Must match password |
| Terms | checkbox | Must be checked |

## Code Structure

- **State** (lines 5-19): Form fields and error states
- **Validation** (lines 22-86): Individual field validators
- **Submit** (lines 88-110): Form submission logic
- **Reset** (lines 112-123): Clear form function
- **Render** (lines 125+): JSX with conditional rendering

## Customization Ideas

1. **Add more fields** - Phone, address, date of birth
2. **Custom validation** - Password strength meter
3. **API integration** - Real backend submission
4. **Field dependencies** - Show/hide based on other fields
5. **Multi-step form** - Wizard with progress bar
6. **File upload** - Image or document upload
7. **Autosave** - Save to localStorage on input

## Advanced Patterns

```jounce
// Password strength indicator
let passwordStrength = computed(() => {
    let strength = 0;
    if (password.value.length >= 8) strength++;
    if (/[A-Z]/.test(password.value)) strength++;
    if (/[0-9]/.test(password.value)) strength++;
    if (/[^A-Za-z0-9]/.test(password.value)) strength++;
    return strength;
});

// Form dirty state
let isDirty = computed(() => {
    return name.value != "" ||
           email.value != "" ||
           password.value != "";
});

// Warn before leaving
effect(() => {
    if (isDirty.value) {
        window.onbeforeunload = () => "You have unsaved changes!";
    } else {
        window.onbeforeunload = null;
    }
});
```

## Validation Patterns

```jounce
// Email
/^[^\s@]+@[^\s@]+\.[^\s@]+$/

// Phone (US)
/^\d{3}-\d{3}-\d{4}$/

// URL
/^https?:\/\/.+\..+$/

// Zip Code
/^\d{5}(-\d{4})?$/

// Credit Card
/^\d{4} \d{4} \d{4} \d{4}$/
```

## Learn More

- [Forms Guide](../../docs/FORMS.md)
- [Validation Patterns](../../docs/VALIDATION.md)
- [Event Handling](../../docs/EVENTS.md)

---

**Difficulty:** Intermediate
**Time:** 20 minutes
**Lines:** 280
