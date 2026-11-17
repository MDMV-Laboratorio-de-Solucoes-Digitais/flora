# Decision: 041 — Rust dependency debloat

**Verdict:** go (scoped)

## Chosen slice
Remove 4 dead dependencies (dotenv, argon2, bcrypt, rust-i18n) + 2 orphan workspace
members (flora-tasks, flora-files) — all with **zero call sites** verified by grep across
every `*.rs` and `Cargo.toml`, no `build.rs`, no feature-gate, no reexport. Delta: 34
packages / 369 Cargo.lock lines, zero additions. Pure deletion, behavior-preserving.

## Deferred (with reasons)
- **RbacService wiring** (c041-02): `revoke_role` unimplemented, `PermissionService`
  not in AppState, needs DTO/assign/revoke semantics + last-Owner protection — scope
  decisions for the SE. Not a mechanical debloat.
- **RLS / least-privilege role** (c041-03): app connects as superuser `postgres`, so
  RLS is inert; `FORCE` without policies would zero rows. Needs role + policies +
  DB-gated tests — SE authorization. Stays `[NV]`.

## Non-goals / non-claims
No code behavior change, no backend/migration change, no ADR/constitution edit, no
required-check/protection change, no deploy/tag/release/secrets. No performance claim
beyond graph/config size.

## NV
Runtime footprint deltas unmeasured; Forgejo/remote CI [NV]. ADR-0003 makes merge a
machine decision under 4 green required checks.
