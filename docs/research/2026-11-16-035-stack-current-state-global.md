## Cargo workspace test scope (verified)

Access date for all URLs below: 2026-11-16.

**Root package + default selection.** Cargo docs: "If the `[workspace]` section is added to a `Cargo.toml` that already defines a `[package]`, the package is the *root package* of the workspace." `cargo test` docs: "By default, when no package selection options are given, the packages selected depend on the selected manifest file (based on the current working directory if `--manifest-path` is not given). If the manifest is the root of a workspace then the workspaces default members are selected, otherwise only the package defined by the manifest will be selected." [W][T]

**`workspace_default_members` (`workspace.default-members`).** [T]: "The default members of a workspace can be set explicitly with the `workspace.default-members` key in the root manifest. If this is not set, a virtual workspace will include all workspace members (equivalent to passing `--workspace`), and a non-virtual workspace will include only the root crate itself." [W] adds: "When unspecified, the root package will be used ... When a root package is present, you can only operate on it using `--package` and `--workspace` flags." Flora is a **non-virtual** workspace (root `Cargo.toml` has both `[package] name = "flora"` and `[workspace]`) with no `default-members` key, so bare `cargo test` selects only the root placeholder package — not the 7 members.

**`--workspace`.** [T]: "Test all members in the workspace." It is a *package-selection* flag; `--all` is its deprecated alias. [T]

**`--all-targets` is not clippy-only.** [T]: "Test all targets. This is equivalent to specifying `--lib --bins --tests --benches --examples`." Clippy docs confirm `--fix` "implies `--all-targets`" and "All the usual workspace options should work with Clippy" [C]. Thus `--all-targets` changes *which targets*, not *which packages*: Flora's `cargo clippy --workspace --all-targets` already covers the 7 crates, while its plain `cargo test` does not.

**Prioritization.** Given 7 member crates and ~104 tests reachable only via `cargo test --workspace`, changing `.github/workflows/rust.yml`'s test step from `cargo test` to `cargo test --workspace` is a one-line fix that restores CI coverage of those tests with no dependency or architecture change. **Inference (not doc-backed):** calling this the *highest-leverage* single action rests on the repo's own test count and on clippy already using `--workspace`; primary sources [W][T] prove the selection mechanics but do not rank remediation priorities.

[W] https://doc.rust-lang.org/cargo/reference/workspaces.html
[T] https://doc.rust-lang.org/cargo/commands/cargo-test.html
[C] https://doc.rust-lang.org/clippy/usage.html

---

# Flora stack — current state, global (research 2026-11-16)

Method: 12+ web searches (en, plus official zh-CN/pt/ja cross-checks); only primary sources used for version/state claims — official release blogs, docs, GitHub repos, crates.io/npm, Rust/PostgreSQL/Node/Vite/Svelte/Tauri/Docker docs. All sources below accessed **2026-11-16**.

## Executive summary

Every pinned version in Flora is **real**; none is a phantom. However the repo is **not uniformly current**: (1) the Rust CI/Docker pin is exactly **1.98.0**, a release with a known trait-object vtable miscompilation (UB) fixed in **1.98.1**, while current stable is **1.100.0**; (2) the frontend is a **full SvelteKit major behind** (2.63.0 vs 3.2.0), and its `svelte.config.js` + `adapter-node` 5.x setup is incompatible with Kit 3; (3) Vitest (4 vs 5) and Playwright (1.60 vs 1.65) lag by a major and five minors. Axum, SQLx, PostgreSQL 18 major, TypeScript 6, Tailwind 4 minor and Vite 8 major are current. Security-relevant patch levels matter more than minor churn: PostgreSQL 18.7 (2026-11-12) fixes an RLS leak and GIN corruption; Docker Engine 29.9.3 fixes multiple CVEs. Archived-repo status argues against wholesale upgrades; the one change with a correctness rationale is bumping Rust off 1.98.0.

## Version table

| Component | Pinned in repo | Current stable (Nov 2026) | Status | Source |
|---|---|---|---|---|
| Rust toolchain | `1.98.0` exact (CI toolchain, `rust:1.98-slim`); `rust-version = 1.98` | **1.100.0** (2026-11-12) | Lagging + defective patch: 1.98.0 UB fixed in 1.98.1 | S1–S3 |
| Rust edition | `edition = "2024"`, `resolver = "3"` | 2024 (stable since 1.85, 2025-02-20) | Current; no newer edition | S4–S5 |
| Axum | `0.8.9` (lock), `0.8` workspace | **0.8.9** (2026-04-14) | Current; 0.9 unreleased on `main` | S6–S7 |
| tokio | `1.43` spec / **1.53.1** lock | **1.54.1** (2026-10-21) | Slight lag; 1.53.x LTS to 2027-09; MSRV 1.85 | S8–S9 |
| sqlx | `0.9.0` | **0.9.0** (2026-05-06) | Current; no patch released; MSRV 1.94 | S10–S11 |
| PostgreSQL | `postgres:18-alpine`, `postgres:18` CI | **18.7** (2026-11-12); 18 GA 2025-09-25 | Current major; patch ≥18.7 advisable | S12–S14 |
| Svelte 5 + runes | `^5.56.1` | **5.57.3** (2026-10-26) | Minor lag; runes stable | S15–S16 |
| SvelteKit | `^2.63.0` | **3.2.0** (2026-11-10); last 2.x = 2.70.4 | **Full major lag**; Kit 3 breaking | S17–S19 |
| adapter-node | `^5.5.4` | **6.0.2** (Kit 3 line); last 5.x = 5.5.8 | Incompatible with Kit 3 | S20–S21 |
| Vite | `^8.0.16` | **8.3.7** (2026-11-09) | Current major, minor lag | S22–S24 |
| Bun | `bun-version: latest` (CI, floating) | **1.4.4** (2026-10-22) | Current but unpinned | S25–S26 |
| Tailwind CSS 4 | `^4.3.0` | **4.3.5** (2026-11-06) | Current minor, patch lag | S27–S28 |
| Vitest | `^4.1.8` | **5.1.0** (2026-11-02) | Major lag; 4.1 gets security backports | S29–S30 |
| Playwright | `^1.60.0` | **1.65.1** (2026-11-13) | Five minors behind | S31–S32 |
| TypeScript | `^6.0.3` | **6.0.x** (6.0.3, 2026-04-16) | Current; TS 7 native imminent | S33–S34 |
| Tauri 2 | Deferred via ADR-0002; no `src-tauri/` | **2.12.3 / 2.12.5** (Oct 2026); 3.0 alpha | N/A by explicit decision | S35–S36 |
| Docker Engine / Compose | Floating images (`postgres:18-alpine`, `valkey:8`, `meili:v1.12`) | Engine **29.9.3**, Compose **5.7.1** | No local pin; host patching matters | S37–S38 |

## Per-component notes

- **Rust / edition.** 1.98.0 (2026-08-20) is real, as is edition 2024 (stabilized in 1.85). 1.98.1 (2026-09-03) fixed a miscompilation that could put a null pointer in a trait-object vtable → UB. Current stable 1.100.0 (never type, allocator API, Cargo unused-dependency reporting) is only four days old. Dependency floors (sqlx 1.94, tokio 1.85, axum 1.80) are all below 1.98, so a bump is mechanically cheap. [S1–S5]
- **Axum.** 0.8.9 (2026-04-14) is latest; `main` carries 22 unreleased changes, five breaking, toward 0.9. Flora is exact-current. MSRV 1.80. [S6–S7]
- **tokio.** Current 1.54.1; LTS lines are 1.51.x (to 2027-03) and 1.53.x (to 2027-09). Flora's lock resolves 1.53.1 — an LTS line, so low urgency. 1.54 raised MSRV to 1.85. [S8–S9]
- **sqlx.** 0.9.0 (release tagged 2026-05-06, announced 05-21) is still current with no 0.9.1; repo moved from `launchbadge` to `transact-rs`. Breaking: `sqlx.toml`, `SqlSafeStr` on `query*()`, `Migrate` trait changes, escaped `PgConnectOptions` options, MSRV 1.94. Flora matches. [S10–S11]
- **PostgreSQL 18.** Native `uuidv7()` (RFC 9562) with optional `shift interval`; parallel GIN builds; FTS now reads config/dictionaries with the cluster's default collation provider (reindex FTS/GIN after upgrade on ICU clusters). 18.7 fixes RLS `ON CONFLICT` leak (CVE-2026-120154), role-cache RLS issue (18.6, CVE-2026-14666) and GIN corruption requiring REINDEX. Note FTS `@@` is not LEAKPROOF, so predicates are not pushed into GIN under RLS — split candidate discovery from visibility if search slows. [S12–S14]
- **Svelte 5 / runes.** Runes (`$state`, `$derived`, `$effect`, `$props`) are the stable compiler contract; Flora force-enables runes mode in `vite.config.ts` with a note to remove it in Svelte 6 (not released). Current 5.57.3 vs pinned 5.56.1. [S15–S16]
- **SvelteKit 3 — highest migration cost.** Kit 3 (2026-10-01) removes `svelte.config.js` (config moves into `sveltekit()` in `vite.config.ts`), replaces `$lib` with `#lib` subpath imports (with file extensions), `$app/stores`→`$app/state`, `$app/environment`→`$app/env`, `csrf.checkOrigin`→`trustedOrigins`, and requires Node 22.17 / TS 6 / Svelte 5.57.1 / Vite 8.0.12 / vite-plugin-svelte 7. Flora fails several of these minimums (5.56.1, Kit 2). adapter-node 6 uses rolldown, serves only build-time static assets, replaces `ORIGIN` with `paths.origin`, content-hash ETags and GET/HEAD-only static responses. Migration: `npx sv migrate sveltekit-3 --tasks all --confirm`. [S17–S21]
- **Vite 8.** Rolldown replaces esbuild+Rollup; Node 20.19+/22.12+; breaking `build.target` raise and `rollupOptions`→`rolldownOptions`; 8.0.16 is real (2026-06-01) but far behind 8.3.x. Official zh-CN announcement cross-checked. [S22–S24]
- **Bun.** CI uses `bun-version: latest` → always current (1.4.4), but floating; `bun check`/`bun fmt` now built in; Playwright 1.65 can run tests under Bun with `--bun`. [S25–S26; S31]
- **Tailwind 4.** 4.3.x current; 4.3.4 fixed a `@tailwindcss/vite` crash under Vite bundled-dev; Tailwind Labs joined Shopify (2026-09-09) — ownership watch item, not a version issue. [S27–S28]
- **Vitest / Playwright.** Vitest 5.1 current; 4.1.12 is last 4.x with backports, and 4.1 first added Vite 8 support (so the pin is not broken, just old). Playwright 1.65.1 is current; 1.62 dropped Node 18/20 from documented support; 1.63 test locks, 1.65 Bun runner. [S29–S32]
- **Tauri 2 / Docker.** Tauri 2.12.x current with 3.0 alphas; ADR-0002 explicitly defers packaging, so no action. Engine 29.9.3/Compose 5.7.1 carry security fixes; Flora pins image tags only, so host/CI runner patch level is the control. [S35–S38]

## Risks and contrarian views

- **Chasing latest is not worth it for a Seed.** This is an archived repo whose verification gate is a browser smoke plus six DB-gated suites. SvelteKit 3 is ~30 breaking changes plus an adapter-node major, Node floor and `$lib` rewrite; its value to a frozen Seed is near zero. Vite 8.0.16→8.3.7, Tailwind 4.3.0→4.3.5 and Playwright patch bumps are churn without new evidence.
- **Counterpoint — one real correctness exception.** Pinning exactly 1.98.0 keeps a known UB miscompilation; moving at least to 1.98.1/1.99.1 (or 1.100.0) is defensible on correctness, not novelty. **Inference:** patched-PG18 and patched-Docker images matter only if actually deployed; the lock files and CI service tags cannot prove the VPS patch level.
- **Floating `bun-version: latest` and floating image tags** are reproducibility/security risks in CI; pinning them is cheaper than any framework major upgrade.
- **Phantom/future audit:** all listed versions verified on crates.io/npm/official repos. The previously phantom `opentelemetry-otlp 0.22` is already removed (Cargo.toml `#026` comment). Not phantom but future: Svelte 6, Tauri 3 stable, TypeScript 7. "Rust 1.98" and "edition 2024" are real; 1.98.0 is real but defective.

## Open questions

1. Does the deployed VPS actually run PostgreSQL ≥18.7, and were FTS/GIN indexes REINDEXed per 18.6/18.7 notes?
2. Was the `cargo test` → `cargo test --workspace` CI gap (see section above) closed?
3. Is the SvelteKit 3 migration deferred until after the Seed gate, or abandoned with the archive?
4. Would adapter-node 6's build-time static-asset table break the runtime `/config.js` injection used for origin-boundary config?
5. Is a TypeScript 6→7 (native Go port) migration planned anywhere, given TS 6 deprecations?
6. Are `deny.toml`/`Cargo.lock` fully free of the phantom `opentelemetry-otlp` reference?

## Full sources (all accessed 2026-11-16)

S1 https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/ · S2 https://blog.rust-lang.org/releases/ · S3 https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/ · S4 https://blog.rust-lang.org/2025/02/20/Rust-1.85.0 · S5 https://doc.rust-lang.org/edition-guide/rust-2024/index.html · S6 https://github.com/tokio-rs/axum/blob/main/axum/CHANGELOG.md · S7 https://crates.io/crates/axum · S8 https://github.com/tokio-rs/tokio/releases · S9 https://crates.io/crates/tokio · S10 https://raw.githubusercontent.com/transact-rs/sqlx/main/CHANGELOG.md · S11 https://github.com/transact-rs/sqlx/discussions/4271 · S12 https://www.postgresql.org/about/news/postgresql-18-released-3142 · S13 https://www.postgresql.org/docs/18/functions-uuid.html · S14 https://www.postgresql.org/docs/18/release-18-7.html · S15 https://svelte.dev/blog/runes · S16 https://svelte.dev/docs/svelte/what-are-runes · S17 https://svelte.dev/blog/sveltekit-3-is-here · S18 https://svelte.dev/docs/kit/migrating-to-sveltekit-3 · S19 https://svelte.dev/blog/whats-new-in-svelte-october-2026 · S20 https://github.com/sveltejs/kit/releases/tag/%40sveltejs%2Fadapter-node%406.0.0 · S21 https://github.com/sveltejs/kit/blob/main/packages/adapter-node/CHANGELOG.md · S22 https://vite.dev/blog/announcing-vite8 · S23 https://vite.dev/releases · S24 https://cn.vitejs.dev/blog/announcing-vite8 (official zh-CN) · S25 https://github.com/oven-sh/bun/releases · S26 https://bun.com/blog/bun-v1.4.4 · S27 https://tailwindcss.com/blog/tailwindcss-v4-3 · S28 https://github.com/tailwindlabs/tailwindcss/blob/main/CHANGELOG.md · S29 https://vitest.dev/releases · S30 https://github.com/vitest-dev/vitest/releases · S31 https://playwright.dev/docs/release-notes · S32 https://github.com/microsoft/playwright/releases · S33 https://devblogs.microsoft.com/typescript/announcing-typescript-6-0 · S34 https://github.com/microsoft/TypeScript/releases · S35 https://tauri.app/release/core · S36 https://tauri.app/blog/tauri-2.12 · S37 https://docs.docker.com/engine/release-notes/29 · S38 https://github.com/docker/compose/releases

Supporting official pages: https://www.postgresql.org/docs/18/release-18.html, https://www.postgresql.org/docs/18/textsearch-indexes.html, https://www.postgresql.org/docs/18/gin.html, https://doc.rust-lang.org/releases.html, https://crates.io/crates/sqlx/0.9.0, https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0, https://svelte.dev/docs/kit/adapter-node, https://vitest.dev/blog/vitest-5, https://vite.dev/guide, https://www.docker.com/blog/docker-engine-version-29. Non-primary cross-checks (not used for version claims): Japanese SvelteKit 3 migration write-up https://3chome.net/sveltekit-3 and Portuguese adapter-node VPS guide https://horadecodar.com.br/sveltekit-adapter-node-vps.
