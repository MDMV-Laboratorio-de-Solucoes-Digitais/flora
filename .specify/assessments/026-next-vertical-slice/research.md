# Research: 026 — R1 synthesis

- **Verification audit:** ADR-0001's gate is partly prose-only: CI lacks fmt/clippy/cargo-deny; cycle 025 artifacts are unfinalized; constitution still contradicts ADR-0001. Core capability substitutions (PG FTS, local FS, in-app notifs, verified OIDC) are code-verified and green.
- **cargo-deny:** `deny.toml` allows `AGPL-3.0` but crates declare `AGPL-3.0-or-later`; local run (metadata-path, warm cache) gives exit 4 on licenses; the one-line allow-list fix yields exit 0. `bans`/`sources` pass today. Enable in CI after the fix; advisories need RustSec DB network and should be a separate advisory job.
- **cargo-vet:** 430 safe-to-deploy + 12 safe-to-run exemptions; `supply-chain/` is gitignored → no gate on clean checkout; full audit backlog ~8.7M lines (infeasible). Keep only as a cheap new-dependency gate (unignore + 3 imports + fix `multer`) in a later cycle; never treat "green" as reviewed.
- **OTel:** `opentelemetry-otlp 0.22` does not exist; phantom deps absent from Cargo.lock. Remove dead declarations; zero-cost `TraceLayer` optional; real OTLP only behind `otel_enabled` with a receiver.
- **Notification transport:** `flora-notifications` compiled but unused; `pgmq_queue` validated but never read; XADD path would collide with 022 dedup. Retire in a behavior-preserving PR (027), keep `realtime` profile, record PGMQ/Valkey as deferred with triggers.
- **Browser e2e:** Playwright config matches only a demo test; two specs are stale/orphaned; dev-smoke JWT expired 2026-11-16. Thin e2e feasible but needs fresh JWT minting; Tauri explicitly deferred from Seed.

## Decision drivers
Cheapest path to an honest, executable "Seed v0.1 verified": enforce the gates in CI, fix the drift in authoritative docs, and remove latent traps — without new services or heavy infra.
