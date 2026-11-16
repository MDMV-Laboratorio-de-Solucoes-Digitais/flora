# Decision: 016 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**C — Seed/VPS operability hardening (Postgres-first, optional Meilisearch/Valkey, 2 vCPU/4 GB).**  
Primary 80/20 for cycle 016: make the Seed runtime **truthful and operable** on a cheap single-node VPS by (1) gating optional services at boot/config with explicit degraded behavior, (2) fixing DB pool/init/config alignment, (3) providing compose profiles/resource/healthcheck guidance for a lean `core` profile, and (4) documenting minimal backup/restore basics — **without** implementing binary file upload/download (slice B) in this cycle.

## Justification (evidence)

- **North-star fit:** `OBJETIVO.md`/`ROADMAP.md` require debloat, Postgres-first optionality, and operability on ~2 vCPU/4 GB. Current gaps directly undermine this: Meilisearch init hard-fails in `flora-api/src/state/app_state.rs` despite optional intent; `flora-core/src/database.rs` hardcodes pool (20) ignoring config; `main.rs` + `AppState` duplicate pool/migrate; messaging config/runtime optionality misaligned; `docker-compose.yml` lacks profiles/limits; `.env.example` has contract drift; backup/restore not operationalized (`docs/research/2026-11-16-seed-vps-profile-audit.md`, `2026-11-16-015-seed-vps-operability-quickwins.md`, `2026-11-16-014-seed-vps-operability-international.md`).
- **80/20 leverage vs B:** B is a valuable P0 gap but carries higher security/contract/schema/FE surface (streaming multipart, path containment, checksums, aggregate quotas/workspace linkage, download authz/headers) per `015-binary-file-upload-download.md`/`015-files-impl-readiness.md`. C is lower-risk, cross-cutting, and de-risks future B implementation/verification on constrained hosts.
- **Multilingual research consensus (EN/pt-BR/es/de/fr/zh-CN/ja; ko/ru triangulated NV):** cheap-VPS self-hosting favors lean core profiles, explicit optional/degraded deps, container resource limits/healthchecks, conservative Postgres pooling/tuning, `pg_dump`+restore drills, and treating Meilisearch/Valkey memory controls as constraints/hints rather than reasons to keep them always-on (`research.md`).

## Scope summary (in)

- Single pool/init path; config-driven DB pool/timeouts.
- Search/messaging optionality aligned (config + boot + minimal degraded handling).
- Compose profiles (`core`/`realtime`/`search`) with 4 GB-oriented limits/healthchecks/localhost binds/persistent volumes and conservative optional-service settings.
- Canonical `FLORA_*__*` env alignment in `.env.example`/docs; truthful unsupported-runtime notes (e.g. SQLite).
- Minimal Postgres + storage backup/restore runbook and profile-aware smoke/docs updates.
- Deterministic tests for config/gating/pool behavior and targeted regressions.

(Full details: `concept.md` — file-by-file checklist, verification, risks/NVs, debloat, non-goals.)

## Key risks (top)

1. **Search/messaging gating breaks call sites or env compatibility** — mitigate with minimal optional/degraded interfaces, additive config compatibility, and targeted tests/audits.
2. **Resource/pool defaults not tuned to real workload** — mitigate with conservative configurable defaults and documented starting points (NV without benchmarks).
3. **Compose limits not enforced as assumed** — mitigate by documenting verification (`docker stats`, runtime mode) and keeping limits as guardrails.

## Non-goals (reaffirmed)

- Slice B (binary upload/download, storage blobs, checksums/quotas/schema/FE file-contract work).
- New mandatory services, multi-node/durable realtime, search indexing features, OIDC/SQLite redesign, S3 rollout, perf benchmarking, deploy/merge/release.

## NVs

- NV: 2 vCPU/4 GB runtime/load/OOM and backup-restore drills not executed here.
- NV: `deploy.resources` enforcement varies by environment.
- NV: exhaustive search/messaging route coupling and full optional-service CI matrix not proven.
- NV: ko/ru research based on triangulation; no locale-primary contradictions found.