# Research: 032 — R1 synthesis

- **CORS:** GO with two mandatory corrections (`axum::http`, not `http`; invalid-header fixture must use `\n`, since spaces are valid in HeaderValue). 4-test hard gate; E2+E3 atomic; `AppConfig::validate()` already called at `main.rs:28`.
- **Governance:** promoting the vet check to required, editing branch protection, CODEOWNERS or rulesets requires explicit human authorization (Admin role). Agent may research/draft only.
- **Restore drill:** conditional GO but requires approved execution window; wrong artifact paths in older plans corrected to `$HOME/.local/state/flora/restore-drill`.
- **Prioritization:** CORS is the only unblocked 032 candidate; FASE 7 Documents is the next product candidate (P1, greenfield); FE dead search box is a cheap user-visible gap.
- **Audit drift:** 031 release doc needs supersession banner; ROADMAP ranges 001-019; OBJETIVO "442"; ADR-0002 line refs.

## Decision drivers
Real security gap in a self-hosted multi-tenant app, executable verifier, zero new deps, and honest governance boundaries.
