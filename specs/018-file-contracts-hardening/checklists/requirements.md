# Requirements Checklist: 018

- [x] Workspace∈org enforced for file upload/metadata-create; tenant-safe errors and consistent access checks for read/download/delete
- [x] Multipart/input contract hardened (single file, deterministic required-field/UUID validation, workspace accepted via query or form field; FE aligned)
- [x] Quota/integrity/cleanup semantics explicit and minimally safe (streaming cap, checksum/size, containment preserved; aggregate/concurrent quota reservation NV)
- [x] DB-gated contract tests on disposable PG18 + temp storage added for core happy/error/isolation/authz/delete/legacy cases
- [x] No new mandatory services; Postgres-first/Seed-safe; local verification evidence recorded
- [ ] PR to `dev`, CI green (no blind merge), merge/sync `github/dev`+`origin/dev`, only `dev`/`main`, planning updated
