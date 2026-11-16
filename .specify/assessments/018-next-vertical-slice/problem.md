# Problem: 018 — File slice has security/consistency gaps and lacks DB-gated contracts

## Problem statement
Cycle 017 delivered real binary upload/download for Seed, but the file slice still has unresolved authorization/consistency gaps and no DB-gated contract coverage. This leaves P0/NV risks around tenant/workspace isolation, quota/cleanup semantics, and regression safety on cheap-VPS deployments.

## Evidence
- **Workspace/authz gaps:** `flora-api/src/routes/files.rs` enforces org membership (`ensure_member`) but does not verify `workspace_id` exists/belongs to `org_id` or caller workspace access on upload/legacy metadata paths; read/download/delete are org-scoped (and delete owner-only) without workspace-level authorization, relying on ID knowledge within org scope.
- **Quota/consistency gaps:** `flora-core/src/services/file_service.rs`/config enforce per-file ceilings (`min(max_upload_bytes, workspace_quota_bytes, org_quota_bytes)`) but not aggregate usage or concurrent reservation; legacy `POST /files` can persist metadata/storage path without bytes/checksum; delete performs best-effort storage removal before/around soft-delete and purge is DB-only (orphan/divergence risk).
- **Storage/validation robustness:** multipart handling assumes/benefits from certain field availability and has limited contract tests; storage containment is strongest in streaming/open helpers but trait methods need consistent guarantees; metadata-only/nullable workspace linkage weakens invariants.
- **No DB-gated file contracts:** `flora-tests/src/contract/files.rs` and `flora-tests/src/integration/files.rs` are placeholders; 017 validation explicitly records DB-gated file contracts (upload/download/authz/containment/quota/deletion) on disposable PG18 as NV.
- **Roadmap/017 NVs:** `OBJETIVO.md`/`ROADMAP.md` and `.specify/assessments/017-next-vertical-slice/decision.md` identify file DB-gated contracts, workspace membership/quota coherence, and storage error semantics as next 80/20 priorities.

## Impact if unresolved
- Potential cross-workspace access or incorrect association within an organization, and weaker tenant guarantees for file operations.
- Quota and storage/DB state can drift (over-allocation, dangling records/blobs, misleading metadata-only records).
- Regressions in a security-sensitive path are likely without deterministic DB-gated verification in CI/dev.

## Desired outcome (018)
A hardened, verifiable file slice: workspace∈org (and membership/policy-consistent) enforcement, robust multipart/validation/error semantics, minimal explicit quota/cleanup behavior, and DB-gated contract/integration tests on disposable PostgreSQL 18 with temp storage covering the core happy/error/security paths.
