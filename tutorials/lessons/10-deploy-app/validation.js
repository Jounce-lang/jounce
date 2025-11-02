export function validate(code) {
  // This lesson is always complete - just need to click the button
  const hasDeployButton = code.includes('Deploy to Production');

  return {
    passed: true,
    feedback: '🎉 Congratulations! You\'ve completed the Jounce tutorial! Click "Deploy to Production" to finish!'
  };
}
