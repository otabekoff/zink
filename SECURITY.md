# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Zink, please report it responsibly.

**Do NOT open a public issue for security vulnerabilities.**

Instead, please send an email to **[INSERT SECURITY EMAIL]** with:

1. A description of the vulnerability
2. Steps to reproduce the issue
3. The potential impact
4. Any suggested fixes (optional)

### What to expect

- **Acknowledgment** within 48 hours
- **Assessment** within 1 week
- **Fix or mitigation** as soon as feasible, depending on severity
- **Credit** in the release notes (unless you prefer to remain anonymous)

### Scope

The following are in scope:

- **lang**: The Rust interpreter — any crash, infinite loop, or unexpected behavior from user input
- **playground**: The browser playground — any XSS, injection, or sandbox escape
- **docs**: The documentation site — any configuration that exposes sensitive data

The following are **out of scope**:

- Issues in third-party dependencies (report those upstream)
- Social engineering attacks
- Denial of service via intentional resource exhaustion in the interpreter (Zink is not designed to be a sandboxed execution environment)

## Security Considerations

Zink is a scripting language interpreter. Please note:

- **No file system access** — Zink programs cannot read or write files
- **No network access** — Zink programs cannot make network requests
- **No system calls** — Zink programs run in a controlled interpreter environment
- **Browser playground** — The IDE runs entirely client-side with no server execution

Thank you for helping keep Zink safe for everyone.
