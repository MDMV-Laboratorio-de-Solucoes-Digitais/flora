# Validation: 027 — Honest Seed closure

**Branch:** `feat/027-honest-seed-closure` (from `github/dev@cda98041`)

## Evidence executed
- `rg -n 'pgmq|PGMQ|XADD|flora_notifications' --glob '*.rs' --glob '*.toml'`: **zero hits**.
- `Cargo.lock` diff: exactly one `[[package]]` stanza removed (no version changes).
- `cargo fmt --all -- --check`: OK; `cargo clippy --workspace --lib --bins -- -D warnings`: clean.
- `cargo-deny --metadata-path ... check licenses bans sources`: no findings.
- Unit tests: flora-core 8, flora-api 17, flora-tests 44 passed.
- DB-gated suites on disposable PG18: auth 12, messaging 4, websocket 8, search 8, notifications 5, files 6 — all passed.
- PR CI result recorded after merge.

## Contract coverage
- Retired only the unwired adapter: core `NotificationService`, `PgNotificationRepository` + 022 dedup index and the `realtime` Valkey profile remain intact.
- `TraceLayer` adds per-request spans with no exporter; OTLP remains NV.

## NV
cargo-vet store/imports; RustSec advisories/SBOM; browser e2e (fresh JWT); Tauri; 2vCPU/4GB; `--all-targets` clippy.
