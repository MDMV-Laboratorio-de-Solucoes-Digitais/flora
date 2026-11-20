# Requirements Checklist: 016

- [x] Slice C scope only (no slice B binary files, no new mandatory services)
- [x] Postgres mandatory; search/messaging optional/degraded with truthful config and boot behavior (code/config/docs)
- [x] Single DB init/pool path with config-driven conservative defaults
- [x] Compose/env/docs aligned for lean 2vCPU/4GB Seed profile and minimal backup/restore notes
- [ ] Deterministic verification evidence recorded (fmt/clippy/tests/CI/db-tests as applicable)
- [ ] PR to `dev`, CI green, merge/sync, only `dev`/`main`, docs updated
