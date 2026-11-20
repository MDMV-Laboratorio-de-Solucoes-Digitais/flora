# ADR-0001 — Seed v0.1 mechanism substitutions and verification gate

**Date:** 2026-11-16  
**Status:** Accepted  
**Scope:** Flora `flora` repository — First Milestone / Seed v0.1

## Context

`README.md:23-47` and `.specify/memory/constitution.md:134-149` describe the first milestone with literal
mechanism names: *Notifications (PGMQ + Valkey)*, *Files (RustFS)*, *Search (Meilisearch)*. Those names conflict
with the constitution's own deployment tiers (`constitution.md:38-43`: Seed = Flora + PostgreSQL; Valkey at
Sprout; PGMQ/RustFS/Meilisearch at Grove) and with the debloat/Postgres-first principles.

The repository itself defines the authority order: `ROADMAP.md:5-6` names code, `Cargo.toml`, workflows and the
constitution as operational truth, and `AGENTS.md` warns that README/old docs are partly aspirational. Cycle
research (`docs/research/2026-11-16-seed-*.md`) confirmed: the Meilisearch index has no mutation write call
sites; PGMQ is name-only; the RustFS/S3 code is a config stub; the `flora-notifications` Valkey XADD path is
unwired. Treating those mechanisms as Seed blockers would be false and would push Flora toward heavy infra.

## Decision

1. **Authority.** The constitution's tier table and the executable evidence (code/Cargo.toml/CI) are normative.
   README's milestone/phase mechanism names are historical and are superseded where they name a Seed mechanism.
2. **Mechanism substitutions for Seed v0.1:**

   | Capability | Seed v0.1 authoritative mechanism | Deferred / profile-gated |
   |---|---|---|
   | Search | PostgreSQL FTS (`search_tsv` + partial GIN, `source: "postgres"`) | Meilisearch (dormant, non-authoritative, Grove; re-enablement gates in `docs/research/2026-11-16-023-*.md`) |
   | Files | Local filesystem, streaming upload/download, SHA-256, contained paths | RustFS/S3-compatible object store (Grove; feature-gated slice defined in `docs/research/2026-11-16-seed-files-rustfs.md`) |
   | Notifications | In-app Postgres table with DB-enforced unread dedup (`20261116000003_*`) | PGMQ + Valkey streams (Sprout/Grove; `docs/research/2026-11-16-seed-notifications-pgmq-valkey.md`) |
   | Identity | OIDC PKCE S256 + verified ID token (JWKS/`iss`/`aud`/`exp`/`sub`/`nonce`), DB sessions with `jti` | Real-OP e2e, JWKS rotation, Keycloak/Zitadel procurement |
   | Observability | `tracing` + `TraceLayer` per-request spans + static `/metrics`; no mandatory collector | OpenTelemetry exporter/collector (phantom deps removed in cycle 026; real OTLP requires a receiver, aligned crate versions and a shutdown path) |

3. **Verification gate.** "Seed v0.1 verified" requires executed evidence, not prose:
   - `cargo fmt --all -- --check`, strict clippy, unit tests;
   - six DB-gated contract suites on disposable PostgreSQL 18 (`auth`, `messaging`, `websocket`, `search`,
     `notifications`, `files`) in `.github/workflows/rust.yml` `db-tests`;
   - integrated dev smoke (`docs/dev-smoke.md`) and canonical frontend CI (`.github/workflows/ci.yml`);
   - per-cycle R4 validation artifacts; **no self-certification**.
4. **Drift rule.** New docs must name capabilities with the authoritative backend, e.g.
   `Search (Postgres FTS; Meilisearch deferred to Grove and gated)`. Per dark-factory governance, drift requires
   an explicit ADR rather than silent divergence; unverified/live claims are marked `NV`.
5. **FASE 0.** AGPL-3.0 is the license. Canonical full text (including the appendix) and `NOTICE` are checked in;
   dependency policy lives in `deny.toml`. Cycle 026 enforced `cargo deny check bans licenses sources` in CI
   (green runs 43875417574 / 43875425127); the 455-exemption `supply-chain/` store was versioned in cycle 028
   and extended in cycle 030 for the dev-mint helper crates
   with an informational CI gate (blocking promotion + CODEOWNERS remain NV).

## Consequences

- The Seed milestone is honest and achievable on a cheap VPS: Flora + PostgreSQL, with profile-gated optional
  components rather than mandatory heavy services.
- Re-enabling Meilisearch/RustFS/PGMQ requires documented gates (hooks/tasks/rebuild/CI, S3 backend, transport
  decision) and a new ADR; it is never implicit.
- Resolved: FASE 0 CI enforcement (cycle 026), notification transport decision — the unwired PGMQ/Valkey crate
  was retired in cycle 027 per `docs/research/2026-11-16-026-notification-transport-decision.md`; canonical Tauri
  deferral is recorded in ADR-0002 (cycle 029).
- Resolved in cycle 030: browser e2e with a freshly minted JWT (one Chromium smoke, green twice in CI).
- Remaining Seed follow-ups (NV): cargo-vet blocking + CODEOWNERS, RustSec advisories/SBOM, OTLP exporter,
  and 2vCPU/4GB load/restore validation.
