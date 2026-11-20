# Requirements Checklist: 017

- [x] Slice B scope (secure binary upload/download, local disk, Postgres metadata) — no S3/presigned/indexing/previews/new mandatory services
- [x] Streaming I/O with conservative limits (no whole-file buffering in memory), canonical path containment, and cleanup on failure (best-effort)
- [x] Authz/tenant isolation enforced on upload/download/read/delete paths; server-generated storage keys; checksum/actual size persisted
- [x] Minimal quota/limit semantics explicit (per-file/upload cap and config limits applied; aggregate quota accounting NV/minimal) and validated
- [x] Frontend/API contract aligned for happy path (multipart upload, download URL/DTO fields, workspace-aware upload)
- [ ] DB-gated file contract tests and full frontend lint/test/build matrix (CI/NV as applicable); PR to `dev`, CI green, merge/sync, only `dev`/`main`
