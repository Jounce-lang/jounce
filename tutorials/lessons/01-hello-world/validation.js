/**
 * Validation for Lesson 1: Hello World
 *
 * Checks if the student's code contains a valid console.log statement
 * with the correct message.
 */

export function validate(code) {
  // Remove comments and whitespace for easier checking
  const cleanCode = code.replace(/\/\/.*/g, '').replace(/\/\*[\s\S]*?\*\//g, '').trim();

  // Check for console.log
  const hasConsoleLog = cleanCode.includes('console.log');

  // Check for the specific message
  const hasHelloJounce = cleanCode.includes('Hello, Jounce') ||
                         cleanCode.includes('Hello,Jounce') ||
                         cleanCode.includes("Hello, Jounce") ||
                         cleanCode.includes('Hello,Jounce');

  // Check for proper syntax (quotes)
  const hasQuotes = (cleanCode.includes('"') || cleanCode.includes("'"));

  // Determine pass/fail
  const passed = hasConsoleLog && hasHelloJounce && hasQuotes;

  // Generate helpful feedback
  let feedback = '';

  if (!hasConsoleLog) {
    feedback = '💡 Hint: Try using console.log() to print a message';
  } else if (!hasQuotes) {
    feedback = '💡 Hint: Don\'t forget to put quotes around your text: "Hello, Jounce!"';
  } else if (!hasHelloJounce) {
    feedback = '💡 Hint: The message should say "Hello, Jounce!" (check your spelling)';
  } else {
    feedback = '🎉 Perfect! You\'ve written your first Jounce program!';
  }

  return {
    passed: passed,
    feedback: feedback,
    hints: [
      'Use console.log() to print messages',
      'Text must be in quotes: "like this"',
      'Don\'t forget the semicolon at the end ;'
    ]
  };
}

/**
 * Test the student's code by running it
 * Returns the console output
 */
export function test(code) {
  const output = [];
  const mockConsole = {
    log: (...args) => output.push(args.join(' '))
  };

  try {
    // Replace console with mock
    const consoleBackup = console;
    global.console = mockConsole;

    // Execute the code
    eval(code);

    // Restore console
    global.console = consoleBackup;

    return {
      success: true,
      output: output.join('\n'),
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
