/**
 * Validation for Lesson 6: Components
 */

export function validate(code) {
  const cleanCode = code.replace(/\/\/.*/g, '').replace(/\/\*[\s\S]*?\*\//g, '').replace(/<!--[\s\S]*?-->/g, '');

  // Check for component definition
  const hasComponentDef = cleanCode.includes('component Button');

  // Check for component usage
  const hasComponentUsage = cleanCode.includes('<Button');

  // Check for self-closing syntax
  const hasSelfClosing = cleanCode.includes('<Button />');

  // Check for App component
  const hasApp = cleanCode.includes('component App');

  const passed = hasComponentDef && hasComponentUsage && hasApp;

  let feedback = '';

  if (!hasComponentUsage) {
    feedback = '💡 Hint: Use the Button component with <Button />';
  } else if (!hasSelfClosing) {
    feedback = '💡 Hint: Use self-closing syntax: <Button />';
  } else {
    feedback = '🎉 Great! You\'ve mastered reusable components!';
  }

  return {
    passed: passed,
    feedback: feedback,
    hints: [
      'Use the component: <Button />',
      'You can use it multiple times!',
      'Components make code reusable'
    ]
  };
}
