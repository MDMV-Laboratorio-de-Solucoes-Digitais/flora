# Tasks: 017 — Secure Binary File Upload/Download

- [x] T017.1 Add additive migration/model/repo/service support for file workspace scoping and secure metadata (workspace_id, checksum/size handling) as needed.
- [x] T017.2 Implement contained local-disk streaming storage (temp+atomic finalize, read stream, path safety) and wire into file service/state/config.
- [x] T017.3 Implement API upload/download/metadata/delete handlers with multipart streaming, authz/tenant checks, limits/validation, and aligned response contract.
- [x] T017.4 Align `apps/frontend` file upload/preview/types/API usage to 017 contract (happy path, workspace-aware upload).
- [x] T017.5 Add/update deterministic and DB-gated-adjacent code paths/tests as feasible; run verification (fmt/clippy/tests/frontend checks); record SDD evidence; PR/CI/merge/sync; update OBJETIVO/ROADMAP. (DB-gated file-specific contract tests and full frontend lint/test/build in CI/NV as applicable.)
