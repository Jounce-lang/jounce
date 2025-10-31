# Security Policy

## Supported Versions

We take security seriously and actively maintain the following versions:

| Version | Supported          | Status |
| ------- | ------------------ | ------ |
| 0.8.x   | :white_check_mark: | Current stable release |
| 0.7.x   | :x:                | Upgrade to 0.8.x |
| < 0.7   | :x:                | No longer supported |

**Recommendation**: Always use the latest stable release for the best security and features.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them responsibly:

### 🔒 Responsible Disclosure

**Email**: security@jounce.dev

**Subject**: `[SECURITY] Brief description`

**Include:**
1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Affected versions
5. Your suggested fix (if any)

### 📋 What to Expect

**Timeline:**
- **24 hours**: Initial acknowledgment
- **72 hours**: Preliminary assessment
- **7 days**: Detailed response with timeline

**Process:**
1. We'll confirm the issue
2. We'll determine severity and impact
3. We'll develop and test a fix
4. We'll prepare a security advisory
5. We'll coordinate disclosure timing with you
6. We'll release the fix and advisory

### 🏆 Recognition

**We value security researchers!**

- Listed in security advisory (if you want)
- Mentioned in CHANGELOG.md
- Added to Hall of Fame (coming soon)
- Swag for significant findings (coming soon)

## 🛡️ Security Best Practices

### For Users

**When using Jounce:**
1. Always use the latest stable version
2. Review dependencies in `package.json`
3. Enable Content Security Policy (CSP)
4. Validate all user inputs
5. Use HTTPS in production
6. Keep Node.js and Rust updated

**Server-side:**
```jounce
// ✓ Good - Validate inputs
server validateInput(data: String) {
    if (data.length > 1000) {
        throw "Input too large";
    }
    return sanitize(data);
}

// ✗ Bad - No validation
server processData(data: String) {
    return eval(data);  // Never do this!
}
```

**Client-side:**
```jounce
// ✓ Good - Sanitize user content
<div innerHTML={sanitizeHTML(userContent.value)} />

// ✗ Bad - Direct HTML injection
<div innerHTML={userContent.value} />  // XSS risk!
```

### For Contributors

**When contributing:**
1. Never commit secrets (API keys, passwords)
2. Use `.env` files (added to `.gitignore`)
3. Review dependencies for known vulnerabilities
4. Follow secure coding guidelines
5. Add security tests for sensitive features

**Check for secrets:**
```bash
# Before committing
git diff | grep -i "password\|api_key\|secret"
```

## 🔐 Known Security Considerations

### Input Validation

**Jounce compiler validates:**
- Source code syntax
- Type safety
- Memory safety (borrow checker)
- Import paths

**You should validate:**
- User inputs in your app
- API responses
- File uploads
- Database queries

### Server-Side Rendering (SSR)

**SSR security:**
- Sanitize all user-generated content
- Use CSP headers
- Avoid inline scripts
- Validate request origins

**Example secure SSR:**
```javascript
// server.js
app.use((req, res, next) => {
    res.setHeader('Content-Security-Policy',
        "default-src 'self'; script-src 'self'");
    next();
});
```

### Dependency Security

**We use:**
- Rust dependencies (audited via `cargo audit`)
- Node.js dependencies (checked with `npm audit`)
- Automated Dependabot updates

**You should:**
```bash
# Check your app's dependencies
npm audit
cargo audit  # Install: cargo install cargo-audit
```

## 🚨 Security Incident Response

**If a security issue is discovered:**

1. **Immediate:** We'll verify and assess severity
2. **Within 24h:** Private fix developed
3. **Within 48h:** Patch released
4. **Within 72h:** Security advisory published

**Severity levels:**
- **Critical**: Remote code execution, privilege escalation
- **High**: XSS, injection vulnerabilities
- **Medium**: Information disclosure
- **Low**: Minor issues with minimal impact

## 📊 Security Features

**Built-in protections:**
- ✅ Memory safety (Rust compiler)
- ✅ Type safety (static typing)
- ✅ Borrow checking
- ✅ No eval() in generated code
- ✅ Automatic XSS prevention (JSX escaping)
- ✅ CSP-friendly output

**Planned features:**
- [ ] Sandboxed WASM execution
- [ ] Content security scanning
- [ ] Automated security testing
- [ ] Dependency vulnerability alerts

## 🔍 Security Audits

**Status:**
- **Last audit**: Not yet conducted (pre-v1.0)
- **Planned**: Q2 2026 (post-v1.0)

**Want to help?**
- We welcome security audits from the community
- Contact us at security@jounce.dev

## 📚 Resources

**Learn more:**
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Node.js Security Best Practices](https://nodejs.org/en/docs/guides/security/)

**Tools:**
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [npm audit](https://docs.npmjs.com/cli/v8/commands/npm-audit)
- [Dependabot](https://github.com/dependabot)

## 📞 Contact

**Security team**: security@jounce.dev
**General questions**: security@jounce.dev
**PGP Key**: [Available on request]

---

**Thank you for helping keep Jounce secure!** 🔒
