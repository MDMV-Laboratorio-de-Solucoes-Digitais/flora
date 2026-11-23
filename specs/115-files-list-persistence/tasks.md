# Tasks: 115 — listagem persistida de arquivos por workspace

**Data**: 2026-11-23 · **Spec**: `specs/115-files-list-persistence/spec.md`

- T-001: Expose workspace-scoped `GET /api/v1/files` endpoint and Rust unit tests in `flora-api/src/routes/files.rs` [FR-001 / SC-001]
- T-002: Add `db_list_files_scoped_to_workspace_and_org` contract test in `flora-tests/src/contract/files.rs` and update lockstep floor `files: 6 -> 7` (78 total) [FR-002 / SC-002]
- T-003: Implement `filesQueryPath`, `FileStore` reload/prepend methods (TDD `FileStore.test.ts`), pt-BR keys, and `/files` page persistence [FR-003 / SC-003]
- T-004: Verify live `GET /api/v1/files` resolution and WCAG 2.2 AA compliance on `/files` in `seed-flow.e2e.ts` [FR-004 / SC-004]
- T-005: Author tenant Comprehension Briefing, governance/append-only docs, test roadmap/evidence, R4/R7 (`0.8.0`), and independent ACR [FR-005 / SC-005]
