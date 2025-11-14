/**
 * Validation for Lesson 4: Event Handlers
 */

export function validate(code) {
  const cleanCode = code.replace(/\/\/.*/g, '').replace(/\/\*[\s\S]*?\*\//g, '');

  // Check for onClick handler
  const hasOnClick = cleanCode.includes('onClick');

  // Check for lambda expression
  const hasLambda = cleanCode.includes('=>');

  // Check for .set() method
  const hasSet = cleanCode.includes('.set(');

  // Check for signal creation
  const hasSignal = cleanCode.includes('signal');

  const passed = hasOnClick && hasLambda && hasSet && hasSignal;

  let feedback = '';

  if (!hasOnClick) {
    feedback = '💡 Hint: Add an onClick handler to the button';
  } else if (!hasLambda) {
    feedback = '💡 Hint: Use a lambda expression: () => ...';
  } else if (!hasSet) {
    feedback = '💡 Hint: Use .set() to update the signal value';
  } else {
    feedback = '🎉 Excellent! Your button is now interactive!';
  }

  return {
    passed: passed,
    feedback: feedback,
    hints: [
      'Add onClick to the button',
      'Use a lambda: () => count.set(count.value + 1)',
      'The UI will update automatically!'
    ]
  };
}
