# Archived: RavensOne Documentation Site

**Archived Date**: November 7, 2025
**Reason**: Outdated branding and content

## Why Archived?

This static HTML documentation website was created during the "RavensOne" era before the project was rebranded to "Jounce". It contains:

- ❌ **Wrong branding**: Uses "RavensOne" instead of "Jounce"
- ❌ **Outdated content**: Doesn't reflect v0.8.3 changes
- ❌ **Old version numbers**: Pre-dates current documentation
- ❌ **Duplicate effort**: Markdown docs in `/docs` are the source of truth

## What Was This?

A static HTML documentation website intended for deployment to:
- Vercel (`https://jounce-docs.vercel.app`)
- Fly.io
- Netlify
- GitHub Pages

It included:
- Landing page with hero section
- Getting Started guide
- Full documentation pages
- Package manager documentation
- Examples page

## Future Documentation Website

If you want to create a modern documentation website in the future, consider:

1. **Use a static site generator** that builds from the markdown files in `/docs`:
   - [VitePress](https://vitepress.dev/) - Vue-based, modern
   - [Docusaurus](https://docusaurus.io/) - React-based, feature-rich
   - [mdBook](https://rust-lang.github.io/mdBook/) - Rust-based, simple

2. **Automatic generation**: Keep `/docs` markdown as source of truth, generate HTML automatically

3. **Single source**: Don't maintain separate HTML files - they get out of sync

## Current Documentation

The current documentation lives in `/docs` as markdown files:

- **README.md** - Quick start guide
- **LEARN_JOUNCE.md** - Tutorials and learning path
- **JOUNCE_SPEC.md** - Complete technical specification
- **docs/guides/** - User guides
- **docs/api/** - API references

All aligned to **v0.8.3 "Enhanced Language Features"** with 580/580 tests passing.

---

**Status**: ⚠️ ARCHIVED - Do not use or deploy
