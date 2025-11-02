export function validate(code) {
  const hasProps = /component\s+Card\s*\([^)]+\)/.test(code);
  const hasPropUsage = code.includes('{title}') && code.includes('{subtitle}');
  const hasPropsInCall = code.includes('title=') && code.includes('subtitle=');

  return {
    passed: hasProps && hasPropUsage && hasPropsInCall,
    feedback: hasProps && hasPropUsage && hasPropsInCall ?
      '🎉 Perfect! You understand props!' :
      '💡 Hint: Add title and subtitle as props to the Card component'
  };
}
