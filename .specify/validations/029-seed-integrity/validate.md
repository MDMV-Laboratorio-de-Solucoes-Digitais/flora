# Validation: 029 — Seed integrity and executable evidence

**Branch:** `feat/029-seed-integrity` (from `github/dev@246a18a`)

## Evidence executed

### Strict lint gate
- `cargo clippy --workspace --all-targets -- -D warnings`: **clean** after adding module-level
  `#[expect(clippy::expect_used, reason = "...")]` to `flora-api/src/routes/health.rs` and `flora-api/src/oidc.rs`
  test modules, and removing one now-unfulfilled `too_many_arguments` expectation. No test bodies were edited;
  no parallel work was reverted. CI step promoted from `--lib --bins` to `--all-targets`.
- `cargo fmt --all -- --check`: OK.

### Integrated dev smoke — EXECUTED, rc=0
Run on isolated infrastructure per `docs/research/2026-11-16-029-smoke-alt-port-runbook.md`:
- `docker compose -p flora-smoke-alt` with `ports: !override` → one `127.0.0.1:55432:5432` binding,
  isolated volume `flora-smoke-alt_flora_postgres_data`; PostgreSQL 18 asserted (`server_version_num >= 180000`).
- API started on `127.0.0.1:33000` with nested `FLORA_APP__*` env; readiness gate `/health/ready` returned 200 after 26s.
- `dev-seed.sql` + `scripts/dev-smoke-session.sql` applied; fresh HS256 JWT minted at runtime (`exp = now + 3600`)
  using the same nested secret; no static/expired token.
- `./test-flora.sh` → **Smoke completed. SMOKE_EXIT=0**. Observed: 3 health checks ok, OIDC authorize URL with
  PKCE S256 + nonce, `select-organization` rotation (user `2222…2222` → org `1111…1111`), two workspaces listed,
  task created (`id 01a1e4fd-e432-7d13-8098-96e1a0a8e90a`). Transcript: `/tmp/flora-alt-smoke-transcript.txt`.
- Cleanup: `docker compose -p flora-smoke-alt down -v` removed only our volume/network; our API PID killed.

This closes the last unexecuted item of the ADR-0001 verification gate. No production access, no deploy webhook,
no IdP contact (login only builds the authorize URL).

## Contract coverage
- ADR-0001 gate: fmt/clippy/deny/vet/advisory + six DB suites + frontend CI + integrated smoke rc=0 — all evidenced.
- ADR-0002 records Tauri deferral with four conjunctive re-entry criteria.

## NV
Browser e2e, release SBOM (no tag process), cargo-vet blocking/CODEOWNERS + branch protection, OTLP exporter,
2vCPU/4GB load/restore; permissive CORS `Any` is a Tauri re-entry gate.
