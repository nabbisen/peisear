# Security

Documentation for people running peisear or auditing how it handles
credentials and data.

- [Hardening notes](hardening.md) — defense-in-depth measures built
  in by default (parameterized queries, argon2id, JWT in HttpOnly
  cookies, CSRF posture, access control scoped at the query level)
  and the decisions left to the operator.

For **reporting a vulnerability**, please see
[.github/SECURITY.md](../../.github/SECURITY.md) in the repository
root — that document explains the disclosure process and a contact
channel.
