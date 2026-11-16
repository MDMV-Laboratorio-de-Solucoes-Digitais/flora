# Release: 024 — OIDC ID-Token Verification

**Date:** 2026-11-16  
**Branch/PR:** `feat/024-oidc-id-token-verification` -> `dev` (pending)  
**Baseline:** `github/dev` after cycle 023

## Delivered
- JWKS-based ID-token verification (`iss`/`aud`/`exp`/`sub`/`nonce` + signature, `kid` selection, symmetric rejection), injected for deterministic tests.
- `nonce` in the OIDC transaction and authorize URL; exact `redirect_uri` allowlist.
- `sub`-based user identity with email fallback; fail-closed on missing claims; legacy unverified decoder removed.
- Five deterministic unit contracts; planning artifacts updated.

## Evidence
- Local: fmt, strict clippy (`flora-api --lib`), 18 unit tests including 5 new OIDC contracts; DB-gated suites recorded in `validate.md`.
- CI: PR result to be recorded after green.

## Seed milestone note
Closes the decisive identity gap identified in the Seed v0.1 R1 research. Remaining Seed follow-ups: FASE 0 CI enforcement, Seed mechanism-reconciliation ADR, honest OTel wiring, notification transport decision, browser e2e.

## NV
Real-OP e2e/JWKS rotation, discovery caching, RBAC stubs, `oidc_subject` backfill, logout/refresh, frontend round-trip, 2vCPU/4GB validation.
