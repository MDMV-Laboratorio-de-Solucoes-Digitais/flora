# Release: 032 — CORS allowlist backend

**Branch/PR:** `feat/032-cors-allowlist` -> `dev` (pending) · **Baseline:** `github/dev@966fb19`

## Delivered
- Exact CORS origin allowlist wired from `AppConfig`, explicit methods/headers, no credentials.
- Deterministic preflight/config tests; `.env.example` Figment array syntax.
- Residual doc drift closed; five R1 research docs recorded.

## Evidence
See `.specify/validations/032-cors-allowlist/validate.md`.

## NV / follow-ups (033)
ADR-0002 #2 (runtime URLs + non-browser proof + WS Origin), cargo-vet blocking + CODEOWNERS (explicit authorization), load/restore drill, OTLP; FE dead search box wiring; FASE 7 Documents.
