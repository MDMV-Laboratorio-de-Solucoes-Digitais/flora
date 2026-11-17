# Tasks: 041 — Rust dependency debloat

- [x] T001 R1 research (3 subagents): dead crates / RbacService / role-RLS.
- [x] T002 Decision 80/20: H1 dead crates (high confidence, low effort) chosen; RbacService/RLS deferred (need scope decisions).
- [x] T003 Baseline gates (cargo check/test, lock 4898 lines) captured.
- [x] T004 Remove 5 workspace deps + 2 members from Cargo.toml; 4 deps from flora-core.
- [x] T005 Delete flora-tasks/ and flora-files/ directories.
- [x] T006 Remove 6 cargo-vet exemptions; keep dotenvy.
- [x] T007 Regenerate Cargo.lock (4898→4529, −369 lines, 34 packages, zero additions).
- [x] T008 Gates green: fmt/clippy/deny/test/vet; test count identical to baseline.
- [x] T009 Fresh-context author-blind audit.
- [ ] T010 Validate + release + research + ROADMAP + commit.
