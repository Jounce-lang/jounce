# Jounce Development Roadmap

**Current Version**: v0.8.2 "Runtime Safety Nets"
**Target Version**: v1.0.0 "Language Lock & Stable Release"
**Last Updated**: November 5, 2025
**Versioning System**: See [VERSIONING.md](../guides/VERSIONING.md)

---

## 📍 Current Status (v0.8.2)

**Production Ready!** ✅
- **Tests**: 580/580 passing (100%)
- **Bugs**: 0 critical issues
- **Safety**: 3-layer defense-in-depth protection complete
- **Packages**: 35/100 complete (35%)
- **Status**: Ready for public launch

### What's Complete

**Core Language** (100%):
- ✅ Lexer, Parser, Type Checker, Code Generator
- ✅ Multi-file projects with `./` and `../` imports
- ✅ Fine-grained reactivity (signals, computed, effects)
- ✅ Full JSX support (components, lambdas, returns)
- ✅ Component system with props and return types
- ✅ Style system with themes and scoping

**Developer Experience** (100%):
- ✅ 457 CSS utility classes (Tailwind-inspired)
- ✅ Enhanced error messages (20+ error codes)
- ✅ 4 production-ready starter templates
- ✅ Compilation cache (102x faster builds)
- ✅ HMR dev server with live reload

**Runtime Safety** (100%) - NEW in v0.8.2!:
- ✅ **Phase 1**: Type checker prevents dangerous code compilation
- ✅ **Phase 2**: Static analyzer warns about problematic patterns
- ✅ **Phase 3**: Runtime catches issues in dev mode
- ✅ Frozen signal objects (Object.freeze protection)
- ✅ Dev-mode side effect detection in computed()
- ✅ 9 critical gotcha fixes implemented

**Ecosystem** (35%):
- ✅ 35 packages published (jounce-router, jounce-db, jounce-auth, etc.)
- ✅ Package manager with dependency resolution
- ✅ 850+ tests across all packages
- ✅ Multi-package example app (task-dashboard)

### What's Next

**Immediate** (v0.9.0 - November 2025):
- Interactive tutorial system (tutorial.jounce.dev)
- Video course and screencasts
- Documentation overhaul with cookbook
- 20+ copy-paste example apps

**Near-Term** (v0.10-0.11 - December 2025 - January 2026):
- Visual playground with AI assistance
- VSCode extension pro with debugging
- CLI power tools (doctor, init, add, deploy)
- Security annotations

**Long-Term** (v1.0.0 - Q2 2026):
- 50+ packages in ecosystem
- Language specification finalized
- Community building and adoption

---

## 🗺️ Roadmap Structure

This roadmap uses a **Phase → Sprint → Release** hierarchy:

- **Phase**: 3+ month strategic milestone (e.g., "Core Compiler", "Package Ecosystem")
- **Sprint**: 1-2 week development cycle within a phase
- **Release**: Public version following semantic versioning

See [VERSIONING.md](../guides/VERSIONING.md) for complete details on our development workflow.

---

## 📊 Phase Overview

| Phase | Focus | Duration | Status | Release |
|-------|-------|----------|--------|---------|
| Phase 1-10 | Core Language | Oct 2025 | ✅ Complete | v0.1.0-0.3.0 |
| Phase 11 | Module System | Oct 2025 | ✅ Complete | v0.3.1 |
| Phase 12 | Reactivity | Oct 2025 | ✅ Complete | v0.4.0 |
| Phase 13 | Style System | Oct 2025 | ✅ Complete | v0.8.0 |
| Phase 14 | Essential Packages | Oct 2025 | ✅ Complete | v0.8.0 |
| Phase 15 | Developer Onboarding | Nov 2025 | 🚧 In Progress | v0.9.0 |
| Phase 16 | Developer Productivity | Dec 2025 | 📋 Planned | v0.10.0 |
| Phase 17 | Security & Production | Jan 2026 | 📋 Planned | v0.11.0 |
| Phase 18 | Ecosystem Expansion | Feb 2026 | 📋 Planned | v0.12.0 |
| Phase 19 | AI Integration | Mar 2026 | 📋 Planned | v0.13.0 |
| Phase 20 | Language Lock | Apr-Jun 2026 | 📋 Planned | v1.0.0 |

---

## 🎯 Phase 15: Developer Onboarding & Learning (CURRENT)

**Target Release**: v0.9.0 "Super Easy Start"
**Timeline**: November 2025 (4 weeks)
**Status**: 🚧 Starting Week 1

### Goal

**Make Jounce the easiest language to learn in 2025**. Developers should go from zero to productive in under 1 hour.

### Philosophy

**"Learn by Building"** - Every tutorial is a working app you deploy at the end.

### Sprints

#### Sprint 15.1: Interactive Tutorial System (Week 1) 📋 CURRENT
**Duration**: Nov 1-7, 2025
**Status**: 🚧 Starting

**Deliverables**:
- [ ] **Interactive Web Tutorial** (tutorial.jounce.dev)
  - In-browser code editor with live preview
  - 10 progressive lessons (5 mins each)
  - Instant feedback on code
  - No installation required
  - Auto-saves progress

- [ ] **Lesson Plan**:
  1. Hello World (2 mins)
  2. Variables & Signals (5 mins)
  3. JSX Basics (5 mins)
  4. Event Handlers (5 mins)
  5. Reactive State (5 mins)
  6. Components (5 mins)
  7. Props & Composition (5 mins)
  8. Styling (5 mins)
  9. Forms & Validation (5 mins)
  10. Deploy Your App (5 mins)

- [ ] **Completion Badge**: "Jounce Certified Developer"

**Success Metric**: User completes all 10 lessons in under 1 hour

#### Sprint 15.2: Video Course & Screencasts (Week 2)
**Duration**: Nov 8-14, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **YouTube Channel**: "Jounce in 100 Seconds"
  - 10 short videos (1-3 mins each)
  - Professional editing with captions
  - Code examples in description

- [ ] **Video Series**:
  1. What is Jounce? (100 seconds)
  2. Install & First App (3 mins)
  3. Reactivity Explained (3 mins)
  4. Component Patterns (3 mins)
  5. Styling & Themes (3 mins)
  6. State Management (3 mins)
  7. Forms & Validation (3 mins)
  8. Package System (3 mins)
  9. Deployment (3 mins)
  10. Common Mistakes (3 mins)

- [ ] **Live Coding Series** (30 mins each):
  - Build a Todo App
  - Build a Blog
  - Build a Dashboard
  - Build an E-Commerce Store

**Success Metric**: 10,000+ YouTube views in first month

#### Sprint 15.3: Documentation Overhaul (Week 3)
**Duration**: Nov 15-21, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **Getting Started Guide** (rewrite)
  - 5-minute quick start
  - Installation troubleshooting
  - Common setup issues
  - IDE configuration

- [ ] **Tutorial Hub** (docs.jounce.dev/tutorials)
  - Beginner tutorials (5)
  - Intermediate tutorials (5)
  - Advanced tutorials (5)
  - All with working code repos

- [ ] **Cookbook** (recipes for common patterns)
  - Authentication patterns
  - API integration patterns
  - State management patterns
  - Performance optimization patterns
  - Testing patterns
  - Deployment patterns

- [ ] **Migration Guides**:
  - From React to Jounce
  - From Vue to Jounce
  - From Svelte to Jounce
  - From Next.js to Jounce

- [ ] **Troubleshooting Guide**:
  - Common errors and fixes
  - Performance issues
  - Debugging techniques

**Success Metric**: Average time-to-first-app under 10 minutes

#### Sprint 15.4: Example App Library (Week 4)
**Duration**: Nov 22-28, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **Example Repository** (examples.jounce.dev)
  - 20+ copy-paste examples
  - All deployable with one command
  - Progressive complexity

- [ ] **Starter Categories**:
  - **Basics** (5 examples): Counter, Todo, Form, List, Timer
  - **UI Patterns** (5 examples): Modal, Tabs, Accordion, Dropdown, Tooltip
  - **Real Apps** (5 examples): Blog, Dashboard, Chat, Shop, Admin Panel
  - **Advanced** (5 examples): SSR, Auth, Database, API, WebSockets

- [ ] **One-Click Deploy**:
  - Every example has "Deploy to Vercel" button
  - Every example has "Deploy to Fly.io" button
  - Pre-configured for production

- [ ] **Searchable Gallery**:
  - Filter by complexity (Beginner/Intermediate/Advanced)
  - Filter by package (router, db, auth, etc.)
  - Filter by pattern (forms, auth, API, etc.)
  - Live preview of each example

**Success Metric**: 1000+ example deployments in first month

### Success Criteria

- ✅ Developer can build & deploy first app in under 10 minutes
- ✅ 10,000+ tutorial completions
- ✅ 10,000+ YouTube views
- ✅ Documentation rated 9/10 or higher
- ✅ 1000+ example app deployments

### Target Release: v0.9.0 "Super Easy Start"

**Release Date**: November 28, 2025
**Type**: MINOR (new features + massive DX improvements)

**Highlights**:
- Interactive tutorial system (tutorial.jounce.dev)
- 10+ video tutorials on YouTube
- Documentation overhaul with cookbook
- 20+ copy-paste example apps
- One-click deployment for all examples

**Tagline**: "The easiest full-stack language to learn in 2025"

---

## 🔮 Phase 16: Developer Productivity & Power Tools

**Target Release**: v0.10.0 "Developer Superpowers"
**Timeline**: December 2025 (4 weeks)
**Status**: 📋 Planned

### Goal

**Make Jounce developers 10x more productive** with world-class tooling and AI assistance.

### Philosophy

**"Ship Faster"** - Every tool saves you time and catches bugs before production.

### Sprints

#### Sprint 16.1: Visual Playground Pro (Week 1)
**Duration**: Dec 1-7, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **Interactive Playground** (play.jounce.dev)
  - Monaco editor with Jounce syntax highlighting
  - Real-time compilation (sub-second)
  - Split-pane live preview
  - Mobile responsive preview mode
  - Share via URL (auto-saves to cloud)
  - Embed in docs and blog posts
  - Fork and remix any example
  - Export to GitHub repo with one click

- [ ] **Power Features**:
  - Multiple file support (components, styles, etc.)
  - NPM package imports
  - TypeScript definitions
  - Auto-completion and IntelliSense
  - Error tooltips with fixes
  - Performance profiling
  - Bundle size analyzer

- [ ] **Templates**:
  - 50+ starter templates
  - Community templates (user-submitted)
  - Template voting and ratings

**Success Metric**: 5,000+ playgrounds created in first month

#### Sprint 16.2: VSCode Extension Pro (Week 2)
**Duration**: Dec 8-14, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **Core Features**:
  - Go-to-definition (cross-file, cross-package)
  - Find all references
  - Hover documentation with examples
  - Auto-imports (add missing imports automatically)
  - Rename symbol (refactor across project)
  - Extract to component/function
  - Organize imports

- [ ] **Smart Features**:
  - Component preview on hover
  - CSS color previews
  - Auto-close JSX tags
  - Auto-format on save
  - Bracket matching for JSX
  - Snippet library (100+ snippets)

- [ ] **Debugging**:
  - Breakpoints in `.jnc` files
  - Step through code
  - Inspect reactive signals
  - Watch expressions
  - Debug console integration

- [ ] **AI Assistant** (Copilot-style):
  - Generate components from comments
  - Suggest fixes for errors
  - Generate tests
  - Explain code blocks

**Success Metric**: 1,000+ VSCode extension installs

#### Sprint 16.3: CLI Power Tools (Week 3)
**Duration**: Dec 15-21, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **`jnc doctor`** - Health check and optimization
  - Checks for common issues
  - Performance audit (bundle size, unused code)
  - Security audit (vulnerable dependencies)
  - Code quality metrics (complexity, duplication)
  - Best practices recommendations
  - One-click fixes for common issues

- [ ] **`jnc upgrade`** - Automatic upgrades
  - Upgrade Jounce to latest version
  - Upgrade packages with compatibility check
  - Auto-migrate breaking changes
  - Rollback on failure

- [ ] **`jnc init`** - Smart project scaffolding
  - Interactive prompts (What are you building?)
  - Choose packages (router, db, auth, etc.)
  - Choose styling (CSS, Tailwind, Jounce utilities)
  - Choose deployment (Vercel, Fly, Docker)
  - Generate complete project structure

- [ ] **`jnc add`** - Add features to existing project
  - `jnc add auth` - Add authentication
  - `jnc add db` - Add database
  - `jnc add api` - Add API routes
  - Auto-configures packages and files

- [ ] **`jnc deploy`** - One-command deployment
  - Auto-detects platform (Vercel, Fly, etc.)
  - Configures environment variables
  - Deploys and returns URL
  - Sets up custom domain

**Success Metric**: `jnc doctor` used 10,000+ times

#### Sprint 16.4: Developer Dashboard (Week 4)
**Duration**: Dec 22-28, 2025
**Status**: 📋 Planned

**Deliverables**:
- [ ] **Dashboard** (dashboard.jounce.dev)
  - All your Jounce projects in one place
  - Project analytics (bundle size, performance)
  - Dependency health monitoring
  - Security alerts
  - Deploy status and logs
  - Team collaboration (share projects)

- [ ] **Package Explorer**:
  - Browse all 35+ packages
  - Search by category, functionality
  - See usage examples
  - Check compatibility
  - One-click add to project

- [ ] **Community Hub**:
  - Showcase your projects
  - Discover community projects
  - Vote and comment
  - Follow developers
  - Fork and remix

**Success Metric**: 500+ projects published to dashboard

### Success Criteria

- ✅ Playground with 5,000+ creations
- ✅ VSCode extension with 1,000+ installs
- ✅ CLI tools used 10,000+ times
- ✅ Developer dashboard with 500+ projects
- ✅ Development time reduced by 50%

### Target Release: v0.10.0 "Developer Superpowers"

**Release Date**: December 28, 2025
**Type**: MINOR (new features + massive productivity boost)

**Highlights**:
- Visual playground with real-time preview (play.jounce.dev)
- VSCode extension with AI assistance
- CLI power tools (doctor, upgrade, init, add, deploy)
- Developer dashboard for project management
- 10x developer productivity increase

**Tagline**: "Build production apps in hours, not weeks"

---

## 🔒 Phase 17: Security & Production Features

**Target Release**: v0.11.0
**Timeline**: January 2026 (3 weeks)
**Status**: 📋 Planned

### Goal

Add enterprise-grade security and deployment features.

### Sprints

#### Sprint 17.1: Security Annotations (Week 1)
**Deliverables**:
- [ ] @secure annotation
- [ ] @auth(role="admin") annotation
- [ ] @validate(schema=UserSchema) annotation
- [ ] Generate middleware checks
- [ ] Add to RPC layer
- [ ] Document security model

**Example**:
```jounce
@secure
@auth(role = "admin")
fn delete_user(id: i64) {
  // Only admins can delete users
}
```

#### Sprint 17.2: Production Build Optimizations (Week 2)
**Deliverables**:
- [ ] Dead code elimination
- [ ] Tree shaking for packages
- [ ] Minification improvements
- [ ] Code splitting by route
- [ ] 30-50% smaller bundles

#### Sprint 17.3: Deployment Tooling (Week 3)
**Deliverables**:
- [ ] `jnc deploy` command
- [ ] Vercel integration
- [ ] Fly.io integration
- [ ] Docker image generation
- [ ] Environment variable management

### Success Criteria

- ✅ Security annotations working
- ✅ Production builds optimized
- ✅ One-click deployment to major platforms
- ✅ Security documentation complete

### Target Release: v0.11.0

**Release Date**: January 24, 2026
**Type**: MINOR (new features)

---

## 🌐 Phase 18: Ecosystem Expansion (35 → 50 Packages)

**Target Release**: v0.12.0
**Timeline**: February 2026 (4 weeks)
**Status**: 📋 Planned

### Goal

Expand package ecosystem to 50 packages for broad use case coverage.

### Package Categories

#### Networking (5 packages)
11. [x] jounce-websocket ✅
12. [x] jounce-graphql ✅
13. [x] jounce-rpc ✅
14. [ ] jounce-upload - File upload utilities
15. [ ] jounce-cors - CORS middleware

#### Data & Persistence (3 packages)
16. [x] jounce-db ✅
17. [x] jounce-cache ✅
18. [ ] jounce-query - React Query-like data fetching

#### UI & Components (4 packages)
19. [x] jounce-ui ✅
20. [x] jounce-theme ✅
21. [x] jounce-animate ✅
22. [ ] jounce-grid - Data grid with sorting/filtering

#### Deployment & Infrastructure (7 packages)
32. [ ] jounce-deploy - Generic deployment CLI
33. [ ] jounce-vercel - Vercel adapter
34. [ ] jounce-fly - Fly.io adapter
35. [ ] jounce-deno - Deno runtime support
36. [ ] jounce-docker - Docker image builder
37. [ ] jounce-env - Environment management
38. [ ] jounce-config - Configuration loader

#### Monitoring & Observability (5 packages)
39. [ ] jounce-analytics - Analytics SDK
40. [ ] jounce-sentry - Error tracking integration
41. [ ] jounce-metrics - Performance metrics
42. [ ] jounce-trace - Distributed tracing
43. [ ] jounce-logs - Log aggregation

#### Advanced UI (4 packages)
44. [ ] jounce-dataview - Advanced data table
45. [ ] jounce-chart - Charting library
46. [ ] jounce-calendar - Calendar/date picker
47. [ ] jounce-editor - Rich text editor

#### Utilities (4 packages)
48. [ ] jounce-a11y - Accessibility helpers
49. [ ] jounce-seo - SEO utilities
50. [ ] jounce-color - Color manipulation
51. [x] jounce-markdown ✅

### Success Criteria

- ✅ 50+ packages published
- ✅ Each package has 10+ tests
- ✅ Full documentation for all packages
- ✅ Inter-package compatibility verified
- ✅ Example apps using 10+ packages

### Target Release: v0.12.0

**Release Date**: February 28, 2026
**Type**: MINOR (new features)

---

## 🤖 Phase 19: AI Integration & Automation

**Target Release**: v0.13.0
**Timeline**: March 2026 (4 weeks)
**Status**: 📋 Planned

### Goal

Make Jounce AI-native with LLM integration and AI-powered developer tools.

### AI Packages (6 packages)

#### Package 51: jounce-ai
**Features**:
- Unified LLM SDK (OpenAI, Anthropic, Google AI)
- Streaming responses
- Token counting
- Error handling

#### Package 52: jounce-llm
**Features**:
- Prompt templates
- Response parsing
- Chain-of-thought helpers
- Few-shot examples

#### Package 53: jounce-embed
**Features**:
- Text embeddings
- Vector generation
- Similarity search
- Clustering helpers

#### Package 54: jounce-rag
**Features**:
- Document chunking
- Vector database integration
- Context retrieval
- Answer generation

#### Package 55: jounce-agent
**Features**:
- Tool calling
- Multi-step reasoning
- State management
- Memory/context

#### Package 56: jounce-prompt-kit
**Features**:
- Prompt library
- Version control for prompts
- A/B testing
- Analytics

### AI-Powered Developer Tools

**CLI Commands**:
- `jnc gen component <description>` - Generate component from description
- `jnc gen tests <file>` - Auto-generate test cases
- `jnc explain <code>` - Explain code with AI
- `jnc refactor <file>` - AI-suggested refactorings

### Success Criteria

- ✅ 6 AI packages published
- ✅ AI code generation works
- ✅ Example AI app (chatbot, RAG system)
- ✅ Documentation with AI best practices

### Target Release: v0.13.0

**Release Date**: March 31, 2026
**Type**: MINOR (new features)

---

## 🏆 Phase 20: Language Lock & v1.0 Preparation

**Target Release**: v1.0.0 "Language Lock"
**Timeline**: April-June 2026 (12 weeks)
**Status**: 📋 Planned

### Goal

Finalize language specification, reach 100 packages, prepare for stable 1.0 release.

### Language Finalization

**Documentation**:
- [ ] Complete language specification document
- [ ] Freeze syntax (no breaking changes post-1.0)
- [ ] Document all type system rules
- [ ] Finalize error handling semantics
- [ ] Define concurrency model clearly
- [ ] Macro system design (optional, post-1.0 is fine)

### Package Ecosystem (50 → 100)

**Target**: 100 total packages

**Remaining Categories**:
- Advanced UI components (10 packages)
- Cloud integrations (15 packages)
- Database adapters (10 packages)
- Content management (8 packages)
- Developer tools (7 packages)

### Documentation

**Comprehensive Guides**:
- [ ] Complete API reference (all 100+ packages)
- [ ] Tutorial series (10+ tutorials)
- [ ] Migration guides (from TS, React, Next.js)
- [ ] Performance tuning guide
- [ ] Security best practices guide
- [ ] Video course (optional)

### Community Building

**Outreach**:
- [ ] Launch website (jounce.dev)
- [ ] Create Discord/forum
- [ ] Write blog posts (launch announcement)
- [ ] Submit to Hacker News, Reddit
- [ ] Reach out to influencers
- [ ] Conference talks (optional)

### Success Criteria

- ✅ 100+ packages published
- ✅ Language spec finalized
- ✅ 50+ real-world apps built
- ✅ 1000+ GitHub stars
- ✅ 100+ contributors
- ✅ Zero critical bugs
- ✅ Production deployments

### Target Release: v1.0.0

**Release Date**: June 30, 2026
**Type**: MAJOR (language lock)

**Commitment**:
- API stability guaranteed
- Semantic versioning strictly followed
- Long-term support (LTS)
- No breaking changes in 1.x series

---

## 📅 Release Calendar

### Q4 2025 (October - December)

| Date | Version | Phase | Highlights |
|------|---------|-------|------------|
| Oct 31, 2025 | v0.8.1 | Public Launch | ✅ Community files, templates, CSS utilities |
| Nov 28, 2025 | v0.9.0 | Developer Onboarding | Interactive tutorials, videos, docs overhaul, 20+ examples |
| Dec 28, 2025 | v0.10.0 | Developer Productivity | Playground, VSCode Pro, CLI tools, dashboard |

### Q1 2026 (January - March)

| Date | Version | Phase | Highlights |
|------|---------|-------|------------|
| Jan 24, 2026 | v0.11.0 | Security | Security annotations, deployment tools |
| Feb 28, 2026 | v0.12.0 | Ecosystem | 50+ packages, monitoring tools |
| Mar 31, 2026 | v0.13.0 | AI Integration | AI packages, code generation |

### Q2 2026 (April - June)

| Date | Version | Phase | Highlights |
|------|---------|-------|------------|
| Jun 30, 2026 | v1.0.0 | Language Lock | 100+ packages, stable API, community launch |

---

## 🎯 Success Metrics

### Current (v0.8.1)

- **Tests**: 635/635 (100%)
- **Packages**: 35
- **Contributors**: 1
- **Stars**: TBD
- **Apps**: 25+ examples, 0 production

### Target (v1.0.0)

- **Tests**: 5000+
- **Packages**: 100+
- **Contributors**: 100+
- **Stars**: 10,000+
- **Apps**: 1,000+ production deployments

### Milestones

- [ ] 1,000 GitHub stars
- [ ] 100 contributors
- [ ] 50 production deployments
- [ ] 10,000 monthly downloads
- [ ] 100 packages in registry
- [ ] 1,000 community-built apps

---

## 🔄 Iteration & Flexibility

This roadmap is a **living document** and will evolve based on:

- **Community feedback** - What users need most
- **Technical discoveries** - New opportunities or constraints
- **Market conditions** - Competitive landscape changes
- **Resource availability** - Team capacity and priorities

### Review Schedule

- **Weekly**: Sprint progress review
- **Monthly**: Phase alignment check
- **Quarterly**: Strategic roadmap adjustment

### Feedback Channels

- GitHub Discussions: [Roadmap Feedback](https://github.com/Jounce-lang/Jounce/discussions)
- GitHub Issues: [Feature Requests](https://github.com/Jounce-lang/Jounce/issues/new?template=feature_request.md)
- Discord: Coming soon

---

## 📚 Related Documents

- [VERSIONING.md](../guides/VERSIONING.md) - Sprint-based development workflow
- [CHANGELOG.md](CHANGELOG.md) - Detailed release history
- [CLAUDE.md](CLAUDE.md) - Development guide and current status
- [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) - Release preparation guide
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute

---

## 🚀 Get Involved

Want to help shape Jounce's future?

1. **Try Jounce** - Build something and share your feedback
2. **Report Bugs** - Help us improve quality
3. **Request Features** - Tell us what you need
4. **Contribute Code** - Pick an issue and submit a PR
5. **Spread the Word** - Share Jounce with others

**See [CONTRIBUTING.md](CONTRIBUTING.md) for details.**

---

**Last Updated**: November 1, 2025
**Current Focus**: Phase 15 (Developer Onboarding & Learning)
**Latest Release**: v0.8.1 "Developer Experience & Public Launch"
**Next Release**: v0.9.0 "Super Easy Start" (November 28, 2025)

**🎉 We're making Jounce the easiest language to learn in 2025! Join us on this journey!**
