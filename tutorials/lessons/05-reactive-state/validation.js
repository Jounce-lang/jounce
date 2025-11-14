/**
 * Validation for Lesson 5: Reactive State
 */

export function validate(code) {
  const cleanCode = code.replace(/\/\/.*/g, '').replace(/\/\*[\s\S]*?\*\//g, '');

  // Check for computed creation
  const hasComputed = cleanCode.includes('computed');

  // Check for lambda in computed
  const hasComputedLambda = /computed\s*\(\s*\(\s*\)\s*=>/.test(cleanCode);

  // Check for doubled value in JSX
  const hasDoubledValue = cleanCode.includes('doubled.value');

  // Check for signal
  const hasSignal = cleanCode.includes('signal');

  const passed = hasComputed && hasComputedLambda && hasDoubledValue && hasSignal;

  let feedback = '';

  if (!hasComputed) {
    feedback = '💡 Hint: Create a computed value with computed(() => ...)';
  } else if (!hasComputedLambda) {
    feedback = '💡 Hint: Use a lambda inside computed: () => count.value * 2';
  } else if (!hasDoubledValue) {
    feedback = '💡 Hint: Display the computed value with {doubled.value}';
  } else {
    feedback = '🎉 Perfect! You\'ve mastered computed values!';
  }

  return {
    passed: passed,
    feedback: feedback,
    hints: [
      'Create computed: let doubled = computed(() => count.value * 2)',
      'Display it: {doubled.value}',
      'It updates automatically when count changes!'
    ]
  };
}
