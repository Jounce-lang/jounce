export function validate(code) {
  const hasNameInput = code.includes('type="text"');
  const hasEmailInput = code.includes('type="email"') || (code.match(/<input/g) || []).length >= 2;

  return {
    passed: hasNameInput && hasEmailInput,
    feedback: hasEmailInput ?
      '🎉 Your form is complete!' :
      '💡 Hint: Add an email input field'
  };
}
