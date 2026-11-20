# Research: 034 — R1 synthesis

- **Post-033 audit:** #246/#247 on canonical; 023–033 validate/release pairs complete; no tags/releases; `release.yml` tag-only. Only doc drift: `release.md:10` says "five R1 research docs" but 033 produced 3 docs + 3 assessment notes; two 033 doc citations to `ROADMAP.md:54` are stale.
- **Exact plan (GO):** E1–E8 — 4 tiny FE files, one pure Rust `origin_allowed` predicate + 403 guard, deterministic pure/oneshot tests, `.env.example` docs. No new deps, no migration, no `AppState` field.
- **Multilingual security (7 languages, 13 primaries):** allow absent `Origin`; deny `null` and unlisted with 403 before auth/DB; exact byte match against the shared `app.cors_origins`; Origin is a browser-origin boundary, NOT authentication. Top pitfalls: literal drift (ports/slash/case/IPv6), proxy stripping, overclaiming.
- **Live proof (feasible, non-destructive):** the running `:3000` binary is STALE (`ACAO: *`, no `/llms.txt`) and belongs to another agent — do not touch. Use isolated port 3100 + DB `flora_034_t8`; port 3100 free, Postgres 5432 has `flora_023_test`; WS-guard probe is enabled by E6.
- **FASE 7 Documents (035):** GO but large — ~30 files, 1400–2000 LOC; 7 (not 1) `AppState` literals; `error.rs` 3 exhaustive sites; new CI contract suite. Defer.
- **Frontend honesty:** 8 false affordances confirmed; pure-FE cleanup is cheap but cosmetic and closes no tracked governance gap. Defer to 035 (bundle with Documents).

## Decision drivers
Highest verified value per effort: close the long-standing cross-cutting security NV (#2) with deterministic tests and honest live evidence, while the canonical tree is clean and CI can prove it.
