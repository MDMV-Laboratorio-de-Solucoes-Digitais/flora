# Concept: 018 — File slice hardening + DB-gated contracts (A1, 80/20)

**Verdict target:** **go** (scoped)

## 1) Minimal solution concept
Harden the existing 017 file slice (not a redesign): enforce correct workspace/tenant authorization and input contracts, make quota/cleanup semantics explicit and minimally safe, and add deterministic DB-gated contract/integration tests on disposable PostgreSQL 18 + isolated temp storage. Keep Postgres-first, no new mandatory services, and defer search/notifications wiring and large-scale quota/resumable/storage-engine changes.

## 2) Target behavior (in scope)
1. **Workspace and tenant authz:**
   - Upload (`/api/v1/files/upload`, multipart) and legacy metadata create (`POST /api/v1/files`) must verify `workspace_id` exists and belongs to `org_id`; enforce caller authorization consistent with product policy (minimum: org member; prefer workspace access/member check if cheaply available) and reject cross-org/workspace mismatches (404/403 per anti-enumeration policy, consistent).
   - Get/download/delete must enforce file∈org and consistent access policy (org/workspace/member/owner as applicable) with non-leaky errors; deleted files not downloadable/readable as active.

2. **Multipart/contract robustness:**
   - Accept `workspace_id` and `file` fields in either order (buffer small fields or require resolvable workspace before persisting stream result), require exactly one `file`, and return deterministic 400s for missing/invalid/duplicate fields.
   - Keep FE-compatible responses/URLs; preserve or explicitly constrain legacy metadata-only endpoint behavior (documented and tested).

3. **Quota, integrity, and cleanup (minimal):**
   - Keep per-file streaming size cap and checksum/actual-size persistence; make quota semantics explicit. Implement a minimal aggregate guard where feasible (DB-based usage check, ideally transactional/advisory-locked for upload path) or, if deferred, lock behavior/tests to explicit per-file/config ceilings and record NV.
   - Improve failure consistency: avoid dangling DB/storage artifacts on common failures (best-effort cleanup with clear ordering/logging), align soft-delete/purge expectations (DB purge vs blob cleanup) minimally and test chosen semantics.

4. **Storage/path safety parity:**
   - Ensure storage operations used by service/routes apply consistent canonical containment/sanitization; keep server-generated paths.

5. **DB-gated verification (primary deliverable):**
   - Implement contract/integration tests in `flora-tests` (replace stubs) using disposable Postgres 18 + migrations, seeded org/workspace/membership/user/session, temp local storage, router `oneshot`, and multipart helpers.
   - Cover: happy-path upload/download roundtrip + headers/checksum/metadata, cross-org/workspace isolation, membership/ownership rules, multipart/order/validation errors, size/MIME/empty-file limits, delete/read-after-delete semantics, and selected quota/cleanup cases per implemented behavior.

## 3) Likely files/components
- `flora-api/src/routes/files.rs`
- `flora-core/src/services/file_service.rs`
- `flora-core/src/repositories/file_repository.rs`, `flora-core/src/traits/repository.rs` (helpers for workspace/org resolution and/or usage sum)
- `flora-core/src/storage/local.rs`, `flora-core/src/config/storage.rs` (containment/semantics parity, minimal)
- `flora-core/src/models/file.rs` and migrations only if required (prefer no schema churn; additive only if necessary)
- `flora-tests/src/contract/files.rs`, `flora-tests/src/integration/files.rs` (+ shared test helpers if needed)
- `specs/018-*`, `.specify/assessments/018-next-vertical-slice/*`, `.specify/validations/018-*`, `.specify/releases/018-*`

## 4) Risks / mitigations
1. **Authz behavior changes break implicit clients:** mitigate with consistent, explicit error mapping and targeted compatibility tests (esp. legacy `POST /files`).
2. **Quota locking/transaction complexity:** prefer minimal, correct-enough aggregate check for Seed; if full concurrency-proof accounting is costly, document/test per-file semantics and record NV with follow-up.
3. **Test flakiness/resource use on CI/VPS:** use env-gated disposable PG, temp dirs, small fixtures, and deterministic seeds.
4. **Scope creep into search/notifications:** defer wiring/outbox to 019 unless a trivial, isolated fix is required.

## 5) NV / non-goals
- NV: 2vCPU/4GB load/OOM/proxy-limit/restore drills, browser e2e matrix, full aggregate quota under high concurrency (if not fully implemented), AV/`infer`-grade sniffing, resumable/S3/presigned uploads, content indexing/previews, multi-node storage, outbox/workers/push delivery.
- Non-goals: Macro search/notification pipeline parity, broad crate unification, frontend redesign, new mandatory services.
