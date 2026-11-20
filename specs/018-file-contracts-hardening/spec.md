# Feature Specification: 018 — File Slice Hardening + DB-gated Contracts

**Feature Branch**: `feat/018-file-contracts-hardening`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/018-next-vertical-slice/decision.md` (verdict: go, slice A1)

## Goal
Harden the 017 binary file slice (authz/tenant/workspace invariants, multipart/validation/cleanup semantics) and add deterministic DB-gated contract/integration tests on disposable PostgreSQL 18 with isolated temp storage, keeping the implementation Seed/VPS-safe and Postgres-first.

## Scope
- **Authorization/tenancy:**
  - Validate `workspace_id` exists and belongs to the authenticated organization for `POST /api/v1/files/upload` and legacy `POST /api/v1/files`.
  - Enforce consistent access control for `GET /api/v1/files/{id}`, download endpoints, and `DELETE /api/v1/files/{id}` (tenant isolation, non-leaky 404/403 behavior, deleted-file handling, owner/delete policy as implemented).
- **Multipart/contract robustness:**
  - Tolerate `workspace_id`/`file` field order in multipart uploads where feasible, require exactly one file, and return deterministic validation errors for missing/invalid/duplicate fields.
  - Preserve FE-compatible upload/download response contract for happy paths; make legacy metadata-only endpoint behavior explicit and tested.
- **Quota/integrity/cleanup (minimal):**
  - Retain streaming size caps, SHA-256 checksum, and actual-size persistence.
  - Implement minimal aggregate quota guard/check for upload/create paths where feasible (DB-backed, Seed-appropriate), or explicitly codify and test per-file/config ceiling semantics if aggregate enforcement is deferred in this slice.
  - Improve failure/delete/purge consistency minimally (best-effort cleanup, clear ordering, no avoidable dangling artifacts) and cover chosen semantics in tests.
- **Storage safety parity:** ensure storage operations used by the file path apply consistent containment/sanitization guarantees.
- **DB-gated tests:** replace stubs in `flora-tests/src/contract/files.rs` and/or `flora-tests/src/integration/files.rs` with env-gated tests (disposable PG18, migrations, seeded fixtures, temp storage, multipart helpers) for happy paths, isolation/authz, validation/limits, and delete/quota/cleanup cases per implemented behavior.

## Non-goals
- Search/notifications wiring, outbox/workers, push delivery, or crate convergence.
- S3/presigned/resumable/chunked uploads, CDN, AV/`infer`-grade sniffing, previews/transcoding, content indexing, multi-node storage.
- Full high-concurrency quota accounting/load/OOM/proxy/restore benchmarks, browser e2e matrix.
- Frontend redesign (beyond compatibility if strictly required) or new mandatory services.

## NV
- Aggregate quota under concurrent uploads may remain minimal/approximate if full transactional reservation is not implemented; semantics must be explicit and tested.
- Real 2vCPU/4GB/proxy/runtime limit behavior not validated.
- Workspace-level membership/ACL model beyond org membership + workspace∈org may be constrained by existing product model; enforce consistently and document.
