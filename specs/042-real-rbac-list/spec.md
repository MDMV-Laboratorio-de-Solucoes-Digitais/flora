# Feature Specification: 042 — Real RBAC role listing

**Branch**: `feat/042-real-rbac-list` · **Status**: In progress (working tree)
**Assessment**: `.specify/assessments/042-real-rbac-list/decision.md`

## Goal
Turn the placeholder `GET /api/v1/roles` into a real, tenant-safe endpoint — the first
concrete RBAC behavior in the Seed — without expanding into assign/revoke/create
semantics that require owner decisions.

## Scope
- `flora-api/src/routes/rbac.rs`: `list_roles` uses `AuthExtractor.require_org()` as the
  sole tenancy source; membership + `find_by_id` gate via pure `can_list_roles()`
  (`OrgRead | OrgAdmin`, wildcard `*`) before `find_by_organization_id`.
- `RoleResponse` DTO omits `organization_id` and timestamps (no tenant leak).
- 3 pure unit tests in `rbac.rs` (permission gate + DTO omission), gate `cargo test --workspace`.
- Remaining handlers (create/get/assign/revoke) stay explicit placeholders.

## Non-goals / non-claims
- No assign/revoke/create/get-by-id implementation — owner scope decision pending.
- No PermissionService in AppState, no RLS/role change, no new endpoints.
- No ADR/constitution/required-check change, no deploy/tag/release.
- Do NOT claim full RBAC enforcement across routes (only this endpoint is real).

## Verification
Frozen before implementation: fmt, clippy `-D warnings`, deny, `cargo test --workspace`
(3 new rbac tests + baseline counts), fresh `cargo check --workspace` on isolated target,
author-blind audit on a **frozen snapshot** (lesson from cycle 041).

## NV
- DB-gated isolation test not added (helper is module-private; no piso requires it) —
  tenant safety is enforced by code + pure tests; live DB behavior [NV].
- Forgejo/remote CI [NV].
