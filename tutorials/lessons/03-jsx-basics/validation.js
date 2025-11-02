/**
 * Validation for Lesson 3: JSX Basics
 */

export function validate(code) {
  const cleanCode = code.replace(/<!--[\s\S]*?-->/g, '');

  // Check for paragraph tag
  const hasParagraph = cleanCode.includes('<p');

  // Check for button tag
  const hasButton = cleanCode.includes('<button');

  // Check for component structure
  const hasComponent = cleanCode.includes('component App');

  // Check for proper JSX nesting
  const hasDiv = cleanCode.includes('<div');

  const passed = hasParagraph && hasButton && hasComponent && hasDiv;

  let feedback = '';

  if (!hasParagraph) {
    feedback = '💡 Hint: Add a <p> tag for the paragraph';
  } else if (!hasButton) {
    feedback = '💡 Hint: Add a <button> tag';
  } else {
    feedback = '🎉 Perfect! You\'ve built your first UI with JSX!';
  }

  return {
    passed: passed,
    feedback: feedback,
    hints: [
      'Use <p>text</p> for paragraphs',
      'Use <button>text</button> for buttons',
      'Make sure all tags are properly closed'
    ]
  };
}
