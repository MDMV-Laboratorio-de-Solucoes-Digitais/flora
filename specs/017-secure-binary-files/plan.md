# Plan: 017 — Secure Binary File Upload/Download

## Inputs
- Assessment: `.specify/assessments/017-next-vertical-slice/decision.md`
- Research: `docs/research/2026-11-16-017-secure-binary-upload-download-vps.md`, `docs/research/2026-11-16-017-macro-files-mvp-parity.md`, 015/016 file research
- Audit: subagent `ses_e1da131a5ffeLyW3hly3HK4J1V` findings (routes/storage/wiring/schema/FE gaps)

## Design (minimal, safe)
- **Schema (additive):** add `workspace_id UUID` (nullable or required per compatibility) to `files` with index/FK; update model/repo/service to persist and scope/list correctly (replace broken `find_by_workspace_id` behavior).
- **Storage:** implement contained local-disk byte operations rooted at canonical `storage.local_path` (temp dir + streaming write/read, atomic rename, path sanitization/containment). Evolve or wrap `StorageProvider`/`LocalFileSystem` minimally; avoid full buffering.
- **Service:** stream-aware create/upload flow: validate inputs/limits, compute SHA-256 and actual size during write, persist metadata, return aligned DTO; enforce tenant/authz invariants; minimal quota behavior (per-file max + explicit semantics).
- **API:** multipart upload handler (Axum multipart), streaming download handler with safe headers, keep metadata/delete endpoints consistent; map errors (400/401/403/404/413/415/5xx) clearly.
- **Frontend:** update `FileUpload.svelte` to send multipart with `workspace_id` to `/api/v1/files/upload`; align types/`FilePreview`/download URL and response fields.
- **Config/deps:** ensure axum multipart/tokio-util/sha2/mime/infer availability as needed (workspace deps already include sha2/mime_guess/tokio; verify axum feature).

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib` and targeted file tests
- `cargo test -p flora-api` and DB-gated file contracts in `flora-tests` on disposable Postgres 18 (`--ignored --test-threads=1`) when implemented
- Frontend (`apps/frontend`): `bun run check`, `bun run lint`, `bun run test:unit -- --run` (or CI project), `bun run build` if touched

## Risks/NVs
- Streaming/multipart and dependency features may require Cargo manifest tweaks.
- Workspace scoping/authz and quota semantics must be explicit and tested; do not regress existing flows.
- No perf/load/restore validation on real 2vCPU/4GB.
