/**
 * Validation for Lesson 2: Variables & Signals
 */

export function validate(code) {
  const cleanCode = code.replace(/\/\/.*/g, '').replace(/\/\*[\s\S]*?\*\//g, '');

  // Check for createSignal
  const hasCreateSignal = cleanCode.includes('createSignal');

  // Check that they changed "World" to something else
  const stillHasWorld = cleanCode.includes('"World"') || cleanCode.includes("'World'");
  const hasCustomName = !stillHasWorld && /createSignal\s*\(\s*["'](.+)["']\s*\)/.test(cleanCode);

  // Check for .value usage in JSX
  const hasValueAccess = cleanCode.includes('.value');

  // Check for component structure
  const hasComponent = cleanCode.includes('component App');

  const passed = hasCreateSignal && hasCustomName && hasValueAccess && hasComponent;

  let feedback = '';

  if (!hasCreateSignal) {
    feedback = '💡 Hint: Use createSignal() to create a reactive variable';
  } else if (stillHasWorld) {
    feedback = '💡 Hint: Change "World" to your actual name';
  } else if (!hasValueAccess) {
    feedback = '💡 Hint: Access the signal value with .value in the JSX';
  } else {
    feedback = '🎉 Great! You\'ve created your first reactive signal!';
  }

  return {
    passed: passed,
    feedback: feedback,
    hints: [
      'Create a signal: let name = createSignal("YourName")',
      'Access the value in JSX: {name.value}',
      'Change "World" to your own name'
    ]
  };
}

export function test(code) {
  try {
    // Extract the signal value
    const match = code.match(/createSignal\s*\(\s*["'](.+)["']\s*\)/);
    const signalValue = match ? match[1] : null;

    return {
      success: true,
      signalValue: signalValue,
      output: `Signal created with value: "${signalValue}"`,
      error: null
    };
  } catch (error) {
    return {
      success: false,
      output: '',
      error: error.message
    };
  }
}
