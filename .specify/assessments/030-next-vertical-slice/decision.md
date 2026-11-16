# Decision: 030 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — One Chromium smoke + Seed doc integrity.**

## Scope (in)
- Feature-gated `flora-mint-session` bin in `flora-core` (`dev-mint`, `required-features`), emitting Playwright storageState from a literal `FloraClaims` via `Config::load()`.
- Playwright config rewrite (`retries: 0`, `workers: 1`, `url` webServer gates, setup + chromium projects), `seed.setup.ts` (seed + mint + positive drift guard), one `seed-flow.e2e.ts` smoke; gitignore auth file; `test:e2e` script.
- Non-required `e2e-browser` CI job (Chromium-only, short artifact retention); no `dev-mint` elsewhere.
- Fix stale clippy prose (README, rust.yml comment/step name, OBJETIVO, 029 research banners) and complete smoke evidence artifact.
- PR/CI green on required checks; merge/sync; plan 031.

## NV / deferred (031)
Tag/release + SBOM, CORS allowlist, load/restore, cargo-vet blocking, OTLP. Browser smoke execution in CI may be non-required and recorded as evidence if green.

## Risk controls
`retries: 0`/`workers: 1` mandatory; no prod access; no static tokens; no reverting parallel work.
