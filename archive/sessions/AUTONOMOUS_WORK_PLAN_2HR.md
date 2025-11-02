# 2-Hour Autonomous Work Plan

**Date**: November 1, 2025
**Duration**: 2 hours (120 minutes)
**Goal**: Prepare Sprint 15.1 deliverables while you're away

---

## 🎯 Objective

Create all the **lesson content, starter code, and documentation** for the Interactive Tutorial System (Sprint 15.1). When you return, everything will be ready to build.

---

## ⏱️ Time-Boxed Tasks (120 minutes)

### Task 1: Create All 10 Lesson Files (60 mins)
**Location**: `/tutorials/lessons/`

For each lesson, create:
- Starter code (what students begin with)
- Solution code (completed version)
- Instructions (step-by-step guide)
- Learning outcomes
- Validation logic (how to check if correct)

**Files to create**:
```
tutorials/
├── lessons/
│   ├── 01-hello-world/
│   │   ├── starter.jnc
│   │   ├── solution.jnc
│   │   ├── instructions.md
│   │   └── validation.js
│   ├── 02-variables-signals/
│   │   ├── starter.jnc
│   │   ├── solution.jnc
│   │   ├── instructions.md
│   │   └── validation.js
│   ├── 03-jsx-basics/
│   ├── 04-event-handlers/
│   ├── 05-reactive-state/
│   ├── 06-components/
│   ├── 07-props-composition/
│   ├── 08-styling/
│   ├── 09-forms-validation/
│   └── 10-deploy-app/
└── README.md
```

**Time allocation**: 6 minutes per lesson × 10 lessons = 60 minutes

---

### Task 2: Create "Getting Started" Quick Guide (20 mins)
**Location**: `/docs/GETTING_STARTED_QUICK.md`

**Content**:
- 5-minute quick start
- Installation (3 different methods)
- First app in 10 lines of code
- Common pitfalls and solutions
- "Next Steps" section linking to tutorial

**Format**: Clear, concise, copy-paste friendly

**Time allocation**: 20 minutes

---

### Task 3: Create Starter Templates (20 mins)
**Location**: `/templates/tutorial-starters/`

Create 5 basic templates students can use after tutorial:

1. **blank** - Empty Jounce app
2. **counter** - Simple counter (post-tutorial practice)
3. **todo** - Todo app (post-tutorial practice)
4. **form** - Form with validation (post-tutorial practice)
5. **dashboard** - Dashboard layout (post-tutorial practice)

Each template includes:
- `main.jnc` (working code)
- `README.md` (how to use)
- One-click deploy button

**Time allocation**: 4 minutes per template × 5 templates = 20 minutes

---

### Task 4: Create Tutorial Landing Page Content (10 mins)
**Location**: `/docs/TUTORIAL_LANDING_PAGE.md`

**Content**:
- Hero section copy
- Benefits (why take this tutorial?)
- What you'll learn (10 lessons overview)
- Time commitment (50 minutes)
- Call-to-action (Start Tutorial button)
- Social proof placeholders
- FAQ section

**Time allocation**: 10 minutes

---

### Task 5: Create Certificate Template (10 mins)
**Location**: `/templates/certificate/`

**Files**:
- `certificate.svg` (template)
- `certificate-generator.md` (how it works)
- Example filled certificate

**Content**:
- "Jounce Certified Developer" header
- Student name placeholder
- Date completed
- Badge/logo
- Share buttons (Twitter, LinkedIn)

**Time allocation**: 10 minutes

---

## 📁 Deliverables Checklist

By the end of 2 hours, you'll have:

- [ ] 10 lesson folders with starter/solution code
- [ ] 10 instruction.md files (step-by-step guides)
- [ ] 10 validation.js files (auto-grading logic)
- [ ] Getting Started Quick Guide (5-min onboarding)
- [ ] 5 starter templates (post-tutorial practice)
- [ ] Tutorial landing page content
- [ ] Certificate template (SVG + generator)
- [ ] Complete tutorials/README.md (overview)

**Total files**: ~50 files created

---

## 🔧 Implementation Details

### Lesson File Structure

Each lesson folder contains:

**starter.jnc** - Starting point for students
```jounce
// Lesson 1: Hello World
// Welcome to Jounce! Let's write your first program.

// TODO: Write console.log("Hello, Jounce!");

```

**solution.jnc** - Completed version
```jounce
// Lesson 1: Hello World - SOLUTION
console.log("Hello, Jounce!");
```

**instructions.md** - Step-by-step guide
```markdown
# Lesson 1: Hello World

## Goal
Write your first Jounce program

## Instructions
1. Delete the TODO comment
2. Type: `console.log("Hello, Jounce!");`
3. Click "Run" to see the output

## Expected Output
```
Hello, Jounce!
```

## Learning Outcomes
- Understand the editor
- See live output
- Feel the instant feedback loop
```

**validation.js** - Auto-grading logic
```javascript
// Validation for Lesson 1
export function validate(code) {
  const hasConsoleLog = code.includes('console.log');
  const hasHelloJounce = code.includes('Hello, Jounce');

  return {
    passed: hasConsoleLog && hasHelloJounce,
    feedback: hasConsoleLog ?
      'Great! You used console.log!' :
      'Hint: Try using console.log()'
  };
}
```

---

## 🎨 Content Writing Guidelines

### Tone & Voice
- **Encouraging**: "Great job!", "You're doing amazing!"
- **Simple**: No jargon, explain everything
- **Concise**: Short sentences, clear instructions
- **Fun**: Use emojis sparingly (🎉, 🚀, ✅)

### Structure
- **Title**: Clear lesson name
- **Goal**: One sentence (what they'll learn)
- **Instructions**: Numbered steps (max 5)
- **Expected Output**: What they should see
- **Learning Outcomes**: Bullet points

### Code Examples
- **Well-commented**: Explain every concept
- **Progressive**: Build on previous lessons
- **Working**: All examples must run
- **Simple**: One concept per lesson

---

## 📊 Progress Tracking

I'll create a summary document as I work:

**WORK_LOG.md** - Real-time progress updates
```markdown
## 2-Hour Work Session - Nov 1, 2025

**Started**: [timestamp]
**Status**: In Progress

### Completed
- [ ] Task 1: Lesson files (0/10)
- [ ] Task 2: Getting Started guide
- [ ] Task 3: Starter templates (0/5)
- [ ] Task 4: Landing page content
- [ ] Task 5: Certificate template

### Time Log
- 00:00 - Started Task 1 (Lessons)
- 00:15 - Completed Lessons 1-3
- 00:30 - Completed Lessons 4-6
- ...

### Notes
- [Any issues or decisions needed]
- [Questions for when you return]
```

---

## ✅ Success Criteria

This autonomous work session is successful if:

- [ ] All 10 lessons have complete starter/solution code
- [ ] All instruction files are clear and actionable
- [ ] Getting Started guide is beginner-friendly
- [ ] 5 starter templates are ready to use
- [ ] Certificate template looks professional
- [ ] All code examples compile and run
- [ ] Documentation is well-organized
- [ ] No placeholder/TODO items left

---

## 🚀 What Happens Next (When You Return)

When you get back, we'll have everything ready to:

1. **Review**: Go through all lesson content
2. **Iterate**: Make any changes you want
3. **Build**: Start building tutorial.jounce.dev
4. **Test**: Run through all 10 lessons
5. **Launch**: Deploy and share!

---

## 📝 Notes & Decisions

### Assumptions I'm Making
- Tutorial is in English (will note where i18n needed)
- Web-based (browser, no installation)
- Target audience: Developers with basic JS knowledge
- Lessons are linear (must complete in order)
- Free and open (no login required, optional for saving)

### Questions for When You Return
- Certificate design preferences (colors, fonts)?
- Should we add video transcripts for accessibility?
- Any specific Jounce features to emphasize?
- Preferred code style (tabs vs spaces, etc.)?

---

## 🎯 Final Checklist (Before I Start)

- [x] Read and understand the plan
- [x] Have all file paths ready
- [x] Know the lesson structure
- [x] Understand success criteria
- [ ] Start timer (120 minutes)
- [ ] BEGIN WORK!

---

**Created**: November 1, 2025
**Duration**: 120 minutes
**Owner**: Claude (Autonomous Agent)
**Status**: Ready to start when you grant permission

---

## 🤖 Autonomous Agent Instructions

**When user says "START" or grants permission:**

1. Create WORK_LOG.md and start timer
2. Work through tasks 1-5 in order
3. Update WORK_LOG.md every 15 minutes
4. If I finish early, add bonus content:
   - Troubleshooting guide
   - Migration guide from React
   - FAQ section
5. Create COMPLETION_SUMMARY.md when done
6. Stop exactly at 120 minutes (no overtime)

**Constraints:**
- No breaking changes to existing code
- No architectural decisions (stick to content creation)
- No external API calls or installations
- Focus on documentation and examples
- If stuck, make a note and move on

**Communication:**
- Update WORK_LOG.md regularly
- Flag any blockers or questions
- Provide time estimates for remaining work
- Summarize what was completed

---

**Ready to start! Just say "START" and I'll begin the 2-hour autonomous work session!** 🚀
