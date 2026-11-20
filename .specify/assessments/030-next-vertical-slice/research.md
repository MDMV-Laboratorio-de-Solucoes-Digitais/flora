# Research: 030 — R1 synthesis

- **E2E:** GO conditional with exact edit list. `select_organization` revokes the injected jti (`auth.rs:384,387`) → `retries: 0` and `workers: 1` are mandatory. Stable semantic selectors exist for select-org → channels/chat → tasks; no `data-testid` needed for one smoke. Dead `.spec.ts` files stay dead.
- **Frontend:** functional flows = select-org, channels/ChatLayout, tasks; header search/AI/login decorative; `FileUpload`/`KanbanBoard` dead code. Keep "Assign to me" checked for a unique task assertion.
- **Release/SBOM:** exact 8-manifest version centralization (root uses dotted `package.*` — a `[workspace.package]` header would duplicate-key); draft-first immutable release with CycloneDX 0.5.10. Defer to 031 (no tag to be cut in this cycle).
- **CORS:** `app.cors_origins` exists but is ignored; allowlist feasible; CORS does not protect WS. Defer to a later hardening slice.
- **Load/restore:** execution deferred (no second host / off-host backup target).
- **Audit:** stale clippy prose in README/rust.yml/OBJETIVO and 029 research snapshots; smoke evidence artifact lacks revision pin/secret fingerprint/checked-in transcript.

## Decision drivers
Highest value per effort: add one deterministic browser smoke (the last verification lane) while repairing documentation integrity.
