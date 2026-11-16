# Validation: 030 — Browser e2e smoke and Seed doc integrity

**Branch:** `feat/030-browser-e2e-smoke` (from `github/dev@0bef919`)

## Evidence executed
- `cargo build -p flora-core --bin flora-mint-session --features dev-mint`: OK.
- `cargo clippy -p flora-core --bin flora-mint-session --features dev-mint -- -D warnings`: clean.
- Plain `cargo build -p flora-core --bin flora-mint-session` fails as intended: target requires `dev-mint`.
- Minter executed with `FLORA_APP__JWT_SECRET` env; emitted storageState decoded:
  `origin=http://127.0.0.1:4173`, `sub=2222…2222`, `jti=018f0000-…-0001`, `organization_id=nil`, `exp=now+3600`, `ver=1`.
- `cargo fmt --all -- --check`: OK; `cargo clippy --workspace --all-targets -D warnings`: clean;
  unit tests flora-core 8 / flora-api 17.
- Frontend `bun run check`: 0 errors, 0 warnings; `bun run lint`: green after Prettier formatting and an ESLint import-style fix.
- `bun run build` was not completed locally (command timed out in this environment); CI job performs the build.
- Six DB-gated PG18 suites remain the merged baseline (023–029); no Rust behavior change in this cycle.

## Contract coverage
- The e2e lane is one-shot by construction (`retries: 0`, `workers: 1`) because `select_organization` revokes
  the injected jti; the setup project guards against secret/claim/session drift with a real API call.
- 029 smoke evidence is now complete and checked in (redacted transcript + provenance).

## NV
Browser smoke executed in CI (runs 43914284405 / 43914293666); not run locally. The e2e job is informational (`continue-on-error`). Release SBOM,
CORS allowlist, load/restore, cargo-vet blocking, OTLP remain deferred.

## Merge evidence
PR #240: all checks green, including the new `e2e-browser` job (4 `build`, 2 `db-tests`, `dependency-review`,
2 `supply-chain-vet`, 2 `e2e-browser`; GitHub runs 43914284405 / 43914293666); merged as `ab3fc007`.
CI iterations fixed a missing clippy component, a repo-root path bug, a webServer cwd, a setup timeout and
an ambiguous "Select" selector.
