# Problem: 017 — Files are metadata-only; no safe binary path for Seed VPS

## Problem statement
Flora currently records file metadata but does not implement a real, safe binary upload/download path. This blocks a P0 Seed capability (arquivos básicos: upload/download real, metadados/permissões mínimas, quotas coerentes, storage local por padrão) and leaves frontend expectations misaligned with backend contracts.

## Evidence
- `flora-api/src/routes/files.rs`: JSON metadata create/get/delete only; no multipart upload handler and no download/stream endpoint.
- `flora-core/src/services/file_service.rs`: DB record creation and basic validations only; no byte persistence, no checksum computation from actual bytes, quota enforcement acts as per-request/config cap rather than verified aggregate accounting (to confirm/adjust minimally), and no storage backend integration.
- `flora-core/src/storage/local.rs` / `flora-core/src/traits/storage_provider.rs` / `flora-files`: storage primitives exist but are not wired into end-to-end file bytes flow.
- Frontend file components expect multipart upload/download behavior (contract mismatch).
- 016 decision explicitly deferred slice B due to higher security/contract/schema/FE surface, making 017 the next prioritized 80/20 P0 slice.

## Impact if unresolved
- Core collaboration/file utility remains unusable for real SMB workflows.
- Security/operability risks increase if implemented ad hoc later (path traversal, buffering/OOM on 4 GB hosts, weak authz, inconsistent quotas/metadata).
- Continued API/FE drift and inability to validate "happy path" file scenarios in dev/CI.

## Desired outcome (017)
A minimal, verified, Seed-safe binary file slice: authenticated tenant-scoped upload/download of real bytes to local disk with streaming I/O, server-controlled paths, basic validation/limits/checksum, coherent metadata/quota behavior, and aligned frontend/API contract for the happy path.
