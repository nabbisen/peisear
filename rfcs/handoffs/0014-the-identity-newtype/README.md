# Handoffs — the identity newtype

**RFC [0014](../../accepted/014-the-identity-newtype.md)**, accepted
2026-10-07, **`DEC-058`**.

| ID | Link | What | Release |
|---|---|---|---|
| PRIV-002 | [PRIV-002](./PRIV-002-identity-becomes-a-type.md) | Identity becomes a type only an authenticated session can construct, so personal-data storage refuses anything else at compile time. **Fixes no defect** — the boundary holds. **§1 first**: whether Rust can actually enforce the construction rule, because tests call storage directly from nine files and have no session. If the honest answer is a one-caller scan rather than a type-system proof, **the decision gets amended rather than read as stronger than what shipped.** | 0.43.0 |
| PRIV-003 | [PRIV-003](./PRIV-003-the-subject-with-no-requester.md) | **`DEC-058` decision 2's own trigger fired.** The snapshot job has a **subject with no requester**, so `PRIV-002` left 11 of 36 functions on `&str` — and three of the six personal-data modules have no `RequesterId` at all. A second sealed type, `SubjectId`, obtainable only `From<&RequesterId>` or from storage's own user enumeration. Closes the one place this release traded a weak guard for none rather than for a strong one. | 0.43.0 |

