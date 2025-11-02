export function validate(code) {
  const hasStyleBlock = code.includes('style App');
  const hasBackgroundColor = code.includes('background-color');

  return {
    passed: hasStyleBlock,
    feedback: hasStyleBlock ?
      '🎉 Great styling!' :
      '💡 Hint: Add a style block for the App component'
  };
}
