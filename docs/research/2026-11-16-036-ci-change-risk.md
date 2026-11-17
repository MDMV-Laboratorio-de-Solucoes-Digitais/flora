# 036 — Risk audit: `cargo test` → `cargo test --workspace` in `.github/workflows/rust.yml:35`

Date: 2026-11-16
Scope: research only, no edits. Target line is currently still `cargo test` (the change is proposed, not applied).
Method: file inspection + `cargo metadata` + local read-only test runs (real cargo `1.99.1` at
`~/.cargo/bin/cargo`; the `cargo` on PATH is the `cargo-queue` wrapper, bypassed for measurement).
No Postgres/Valkey/Meilisearch was running locally during the test runs.

## Verdict: **GO** (ship the one-line change as-is; do not add `--all-targets`)

It fixes a real coverage hole rather than adding risk. All 7 workspace members' tests pass locally
with the proposed command, no ignored test runs unintentionally, and no external service is needed.
The only new cost is CI compile time for test binaries that were previously never linked in this job.

---

## 1. Why the change matters: the current line runs **zero** tests

`Cargo.toml` is **not** a virtual workspace — it has a root `[package] name = "flora"` (line 7)
alongside `[workspace]` (line 17). Cargo therefore uses the root package as the sole default member:

```
workspace_members:       7 members + root "flora" (8 packages)
workspace_default_members: root "flora" ONLY
```

Measured:

```
$ cargo test
  Running unittests src/main.rs (flora-…)
  running 0 tests
  test result: ok. 0 passed; 0 failed; ...
```

`src/main.rs` is the placeholder `fn main() { println!("Hello, world!"); }`. So the `build` job's
Test step has been a no-op for the entire workspace. `--workspace` is the correct fix.

Related observation (out of scope, but worth a follow-up issue): `cargo build` on line 33 has the
same default-member problem. Measured compiler artifacts: only `crates.io-index` and `flora`.
The only CI step that actually touches the real crates today is `cargo clippy --workspace
--all-targets` on line 27.

## 2. Test-binary inventory (all 7 members + root) under `cargo test --workspace`

| Package | Binary / target | Tests | Result |
|---|---|---:|---|
| `flora` (root) | unittests `src/main.rs` | 0 | ok |
| `flora-api` | unittests `src/lib.rs` | 25 | ok |
| `flora-api` | unittests `src/main.rs` | 0 | ok |
| `flora-core` | unittests `src/lib.rs` | 12 | ok |
| `flora-core` | integration `tests/models.rs` | 21 | ok |
| `flora-files` | unittests `src/lib.rs` | 0 | ok |
| `flora-messaging` | unittests `src/lib.rs` | 0 | ok |
| `flora-search` | unittests `src/lib.rs` | 0 | ok |
| `flora-tasks` | unittests `src/lib.rs` | 0 | ok |
| `flora-tests` | unittests `src/lib.rs` | 89 (46 pass, **43 ignored**) | ok |
| 7 crates | doc-tests | 1 (ignored, `flora-api` cors.rs) | ok |

Totals: **104 passed, 0 failed, 43 ignored unit tests + 1 ignored doc test**, runtime ~0.08 s.
All 10 test binaries were discovered and run. Four members (`files`, `messaging`, `search`,
`tasks`) are currently empty test shells — harmless but they confirm `--workspace` adds no hidden
work there.

## 3. The 43 `#[ignore]` tests do **not** run unintentionally

All 43 live in `flora-tests/src/contract/{auth,messaging,files,notifications,search,websocket}.rs`,
each annotated `#[ignore = "requires Postgres; ..."]`. Verified empirically: the `--workspace` run
reports `43 ignored`, and the `db-tests` job (lines 72–83) is what runs them with
`-- --ignored --test-threads=1` against its Postgres 18 service. `--workspace` only broadens
*which packages* are tested; it never flips ignore state. The extra `1 ignored` is the
`/// ```ignore` doc fence in `flora-api/src/middleware/cors.rs:22`, also correctly skipped.

Each DB-backed helper already self-skips when `FLORA_DATABASE__POSTGRES_URL` /
`DATABASE_URL` is unset, so even an accidental `--ignored` run in the `build` job (which has no
DB env) would pass rather than fail — defence in depth.

## 4. External services: not required by the non-ignored tests

- Non-ignored contract tests use `PgPool::connect_lazy("postgresql://localhost/flora_test")`
  (auth.rs, websocket.rs). `connect_lazy` performs **no I/O** until a query is issued; none of
  these tests issue one. All passed with no local Postgres.
- The two websocket guard tests (`ws_allowed_origin_passes_guard_then_fails_auth_without_db`,
  `ws_disallowed_origin_is_403_without_db`) are explicitly DB-free.
- No test hit Redis/Valkey or Meilisearch; FTS is Postgres-only and those paths are `#[ignore]`d.
- The `build` job has no service containers, but none are needed. The `db-tests` job remains the
  sole owner of DB-backed verification.

## 5. Profiles and features

- Profiles: `dev`, `ci` (inherits dev), `release` defined in `Cargo.toml:97–112`. CI never passes
  `--profile ci` (the workflow calls plain `cargo build` / `cargo test`), so tests use `dev`.
  No profile change is implied by this one-liner.
- Features: only `flora-core` (`default = []`, `sqlite`, `test`, `dev-mint`) and `flora-api`
  (`default = []`) declare features. No `cfg(feature = ...)` gates appear in any Rust source.
- `flora-mint-session` has `required-features = ["dev-mint"]`. Verified: neither `cargo test
  --workspace` nor `--workspace --all-targets` builds it; it is only clippy-checked explicitly
  in the `e2e-browser` job (line 161). So the test change does not pull in the dev minter.

## 6. Comparison with the clippy command (line 27)

Current: `cargo clippy --workspace --all-targets -- -D warnings`.
Proposed test: `cargo test --workspace`.

After the change the two steps finally cover the same package set (`--workspace`), which removes
an inconsistency. Clippy's `--all-targets` already compile-checks lib/bin/test/example/bench
targets; the test step's job is only to *execute* tests. Keeping the flags different is correct.

## 7. Should `--all-targets` be added to the test command? **No**

- There are **zero** bench and zero example targets in all 8 packages, so it adds no coverage.
- Measured difference: `--workspace --all-targets` produces the identical 104 passed / 43 ignored,
  but **drops the doc-test phase entirely** (no `Doc-tests` sections). `--all-targets` suppresses
  doctests. That is a small coverage regression, not a gain — the ignored cors.rs fence would no
  longer even be enumerated.
- Adding it also invites future confusion by implying benches are tested. The clippy line is the
  right place for `--all-targets`; the test line is not.

Recommended final line (unchanged from the proposal):

```yaml
      - name: Test
        run: cargo test --workspace
```

## 8. Concrete risks and mitigations

| Risk | Likelihood | Impact | Mitigation / status |
|---|---|---|---|
| CI `build` job becomes slower: test targets now need full codegen+link (clippy only produces rmeta; `cargo build` skips tests). ~10 binaries, cold cache. | Medium | Low — seconds to ~1 min | Acceptable. Test execution itself measured <0.1 s. No service startup added. |
| A future un-ignored test needs Postgres/Redis/Meili and breaks `build` (no service containers there). | Medium | Medium | Keep such tests `#[ignore]`d (current convention, 43/43 correct); `db-tests` job owns them. Consider a lint/CI check asserting new `#[ignore]` for DB use. |
| Newly-active tests fail on CI's pinned Rust 1.98.0 vs local 1.99.1. | Low | Medium | Tests use only stable constructs (`uuid::now_v7`, `PgPool::connect_lazy`, tokio). Toolchain pinned in workflow lines 13–19, not via `rust-toolchain.toml`. |
| Environment-dependent tests differ in CI (missing `FLORA_*` env). | Low | Low | Passed locally with no `FLORA_*`/`DATABASE_URL` set; contract helpers self-skip. |
| `print_stdout = "warn"` / strict lints in test code. | Low | Low | Warnings do not fail `cargo test`; clippy `-D warnings` already gates the same code in the prior step. |
| Duplicate coverage with `db-tests` job. | Low | Low | No duplication: ignored tests stay skipped here and run only there. |

## 9. Limits of this audit

- Measurements are from the local toolchain (1.99.1), not CI's pinned 1.98.0, and from a warm
  `target/` cache, so the compile-time delta is estimated rather than measured cold.
- No actual GitHub Actions run was performed; conclusions about job environment are based on
  `.github/workflows/rust.yml`.
- The change is not applied in the working tree; `git diff -- .github/workflows/rust.yml` is empty.
- Other uncommitted files in the repo (`OBJETIVO.md`, `ROADMAP.md`, `docs/research/035-*`) were not
  touched or assessed.
