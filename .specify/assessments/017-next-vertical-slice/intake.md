# Intake: 017 — Next vertical slice (post-016)

**Date:** 2026-11-16  
**Baseline:** `dev@d6cac4d74` (016-C merged, PR #219, CI green).  
**Objective:** advance Flora as a viable, performant, self-hosted Macro-like alternative for SMBs on cheap VPS (Rust + PostgreSQL, Svelte 5; Tauri later) via 80/20 debloat and verified vertical slices.

## Inputs
- `.specify/assessments/016-next-vertical-slice/decision.md` — **016 verdict `go` (scoped, slice C)**; **slice B (binary file upload/download hardening) explicitly deferred to next cycle**.
- `OBJETIVO.md` / `ROADMAP.md` — P0 gap: real binary upload/download, metadata/permissions/quotas coherent, local storage default; operability hardening partially addressed in 016-C.
- Existing research: `docs/research/2026-11-16-016-binary-file-upload-hardening.md`, `docs/research/2026-11-16-016-macro-files-ops-parity.md`, `docs/research/2026-11-16-015-{binary-file-upload-download,files-impl-readiness}.md`.
- In-progress 017 research (background): `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md`, `docs/research/2026-11-16-017-macro-files-mvp-parity.md` (if completed).

## Candidates (80/20)
1. **B — Secure binary file upload/download (local disk, metadata-persisted)**: minimal real bytes path (multipart/streaming upload, download/stream), tenant/workspace/authz checks, path safety/limits/checksum/basic quotas, storage wiring, and minimal `apps/frontend` contract alignment.
2. **Realtime/search/notifications hardening**: post-handshake revalidation, broader replay/multi-node, search indexing/events wiring — higher coordination/risk without closing core P0 files gap.
3. **Auth/OIDC/RBAC hardening**: important but broader security/product surface; can be sequenced after or alongside targeted file authz.

## Constraints
- Seed/VPS (~2 vCPU/4 GB): no full-file buffering, conservative limits, no new mandatory services.
- Postgres-first, AGPL-safe, no proprietary deps, no blind merge; CI green required.
- NV: 2vCPU/4GB load/OOM/restore drills, real OIDC/JWKS/browser/multi-client e2e, compose limit enforcement by runtime.
