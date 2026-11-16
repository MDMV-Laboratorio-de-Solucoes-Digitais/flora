# Tasks: 018 — File Slice Hardening + DB-gated Contracts

- [x] T018.1 Enforce workspace∈org validation in file upload/metadata routes and keep consistent tenant-safe access checks for read/download/delete (org membership + org/file isolation).
- [x] T018.2 Harden multipart parsing and input validation (single file, deterministic required-field errors, workspace via query/form; FE sends workspace_id as query param for robustness) and make legacy metadata-only behavior explicit/tested.
- [x] T018.3 Apply minimal quota/integrity/cleanup improvements in scope (retain streaming cap/checksum/containment; explicit per-file/config quota semantics; no unsafe storage behavior introduced). Full aggregate/concurrent quota reservation remains NV.
- [x] T018.4 Implement DB-gated file contract tests on disposable PG18 + temp storage (replace stubs) covering upload/download/get happy path, workspace-not-in-org rejection, cross-org isolation, missing workspace/empty file validation, delete owner/non-owner/cross-org, and legacy metadata workspace validation.
- [x] T018.5 Run verification (fmt/clippy/core/api/tests lib), record SDD evidence, open PR to `dev`, ensure CI green, merge/sync, update `OBJETIVO.md`/`ROADMAP.md`, and define 019. (DB-gated file tests executed in CI/env with disposable PG; local execution NV without DB URL.)
