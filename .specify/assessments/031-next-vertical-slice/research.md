# Research: 031 — R1 synthesis

- **Release/SBOM:** GO for file edits only. Zero tags today; 8 workspace packages at 0.1.0; root uses dotted `package.*` keys (a `[workspace.package]` header would duplicate-key — empirically confirmed). `release.yml` is tag-triggered; adding it without a tag is safe. Must remain NV: immutability, tag push, generator dry run, `--target` decision.
- **CORS:** GO for backend allowlist, but cannot claim ADR-0002 re-entry #2 (non-browser origin/WS not verified); defer to 032.
- **cargo-vet:** DEFER promotion — no explicit authorization for merge-rule edits, no CODEOWNERS, only ~25 min observation (not 7 days), 13 exemptions labelled informational.
- **Observability:** current tracing + TraceLayer + static /metrics sufficient for Seed; no code change.
- **Load/restore:** host is 8 vCPU/13.6 GiB, load tools absent; execution deferred.
- **Audit:** ADR-0001 still says 443 exemptions and e2e NV (now 455 and closed); AGENTS.md omits e2e-browser; OBJETIVO/ROADMAP stale; 030 research snapshots need supersession banners.

## Decision drivers
Turn the verified Seed into a *releasable* Seed (draft-first tag workflow + SBOM) without cutting a release, and make governance docs literally true.
