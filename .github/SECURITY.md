# Security Policy

Thank you for helping keep peisear and the people who rely on it
safe. This document describes how to report a vulnerability in
peisear itself.

## Supported versions

peisear is in active development and has not yet reached 1.0. Only
the **most recent minor release** receives security fixes. Older
versions will not be patched; users should upgrade.

| Version | Supported |
|---|---|
| 0.2.x | ✅ |
| < 0.2.x | ❌ |

## Reporting a vulnerability

**Please do not open a public GitHub issue for security problems.**

Instead, contact the maintainers privately via one of the following
channels, in order of preference:

1. **GitHub Security Advisories** — [Report a vulnerability](https://github.com/nabbisen/peisear/security/advisories/new)
   (private, preferred).
2. **Email** — `nabbisen@scqr.net` . PGP key available on request.

When reporting, please include:

- A description of the issue and its potential impact.
- Steps to reproduce, ideally including a minimal proof-of-concept.
- The peisear version (`cargo pkgid` or the git commit).
- Your contact information if you would like credit in the fix
  advisory.

## What to expect

- **Acknowledgement** within **10 business days** of report receipt.
- **Initial assessment** within **20 business days**, including a
  severity judgment (CVSS-style) and an estimated fix timeline.
- **Fix** as quickly as the severity warrants — generally within
  **90 days** for high-severity issues, sooner for criticals.
- **Coordinated disclosure** — we will work with you on a disclosure
  timeline. The default is 180 days or the date of the fix release,
  whichever comes first.
- **Credit** — unless you ask to remain anonymous, your name (and
  optionally a link) will be included in the published advisory.

## What we consider in-scope

- Remote code execution or command injection.
- Authentication or authorization bypass in the peisear codebase.
- SQL injection, XSS, CSRF, and similar web vulnerabilities.
- Cryptographic weaknesses in the auth layer.
- Memory-safety issues in peisear's own code.
- Information disclosure of other users' data.

## What we consider out-of-scope

- Vulnerabilities in third-party dependencies. Report those upstream;
  we will update as fixes become available.
- Issues requiring physical access to the host machine.
- Issues that require an attacker to already have administrative
  access to the deployment.
- Denial-of-service via volumetric attacks. These are an
  infrastructure concern, not a peisear concern.
- Missing security headers on a deployed instance (these are the
  operator's responsibility; see
  [../docs/security/hardening.md](../docs/security/hardening.md)).

## Non-security bugs

If you're not sure whether something is a security issue, report it
privately first. If it turns out to be a regular bug, we'll move the
conversation to a public issue with your permission.

Thank you for being a good citizen of the peisear community.
