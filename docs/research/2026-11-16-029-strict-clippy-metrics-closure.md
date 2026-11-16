> **Superseded by PR #238/#239 (cycle 029):** this is a pre-implementation snapshot; check committed files for current state.

# Strict Clippy (`--all-targets`) closure and the `/metrics` stub — 2026-11-16-029

- **Type:** research only. No source, manifest, config, CI, or test file was changed. This document is the only artifact.
- **Scope:** one nested repo (`projetos/Arquivados/flora`). Read-only cross-check of `flora-api/src/routes/health.rs`, `Cargo.toml`, `flora-api/Cargo.toml`, `.github/workflows/rust.yml`, adjacent lint-suppression sites, ADR-0001, and cycle-026 observability research.
- **Questions:**
  - **A.** What is the least invasive, non-reverting fix for test-only `expect_used` violations in `flora-api/src/routes/health.rs` that block `cargo clippy --workspace --all-targets -- -D warnings`, and which option best matches the workspace lint policy?
  - **B.** With no OTel exporter and a cheap VPS, should Flora keep the static `/metrics` stub or implement minimal real metrics now?
- **Method:** primary sources only (Clippy lint docs, Rust Reference on `#[expect]`, Prometheus exposition-format spec, `tower-http` docs, `docs.rs` for the Rust Prometheus client). Local evidence is read-only. A live clippy run was **not** executed (see NV-1).

---

## 0) TL;DR

1. **Topic A — exact fix:** put a named `#[expect(clippy::expect_used, reason = "...")]` on the `#[cfg(test)] mod tests` in `health.rs`, immediately after `#[cfg(test)]`. This is a 4-line additive edit, reverses nothing, and is the same suppression pattern already used in `flora-api/src/extractors/auth.rs:259-265` for test-only `unwrap_used`/`panic`. A test helper returning `Result` is possible but broadens the change into test bodies (more churn, and the workspace already accepts narrow test-module expectation). `unwrap_or_else` is the **wrong** tool here: it keeps the fallible call and typically trips `clippy::panic` because the fallback panics. (§1–§3)
2. **Topic B — keep the static stub. Do not add real metrics now.** The stub is valid Prometheus text exposition, correctly labelled `text/plain; version=0.0.4; charset=utf-8`, and ADR-0001 makes "static `/metrics`; no mandatory collector" the *authoritative Seed mechanism*. Adding `prometheus = "0.14"` would introduce a new dependency into a repo whose Seed verification gate already has open NVs for `cargo-vet` exemptions and advisory/SBOM coverage — a poor trade for a 2 vCPU/4 GB box with no scraper. (§4–§6)
3. **Scope reality:** the two blockers are *not* equally urgent. CI currently runs `cargo clippy --workspace --lib --bins -- -D warnings` (production targets only), so the test-only `expect`s are **not** exercised by CI at all; `--all-targets` is a local/per-cycle gate. Closing Topic A makes that strict local gate pass (subject to NV-2/NV-9) and lets the stale comment at `rust.yml:23-25` be removed. (§1.2)

**Decision in one line:** *add one module-level `#[expect(clippy::expect_used, reason = ...)]` to `health.rs` tests; keep `/metrics` static until a real scraper/backend exists.*

---

## 1) Flora evidence (read-only)

### 1.1 The violating calls

`flora-api/src/routes/health.rs` contains exactly four `.expect(...)` calls, all inside `#[cfg(test)] mod tests` (lines 122-168):

| Line | Call | Context |
|---|---|---|
| 141 | `axum::body::to_bytes(...).await.expect("body bytes")` | `test_llms_txt_returns_ok_markdown_manifest` |
| 142 | `std::str::from_utf8(&body_bytes).expect("utf-8 body")` | same |
| 163 | `axum::body::to_bytes(...).await.expect("body bytes")` | `test_metrics_returns_ok_prometheus_text` |
| 164 | `std::str::from_utf8(&body_bytes).expect("utf-8 body")` | same |

Repo-wide grep for `.expect(` finds only three files: `health.rs` (4, unsuppressed), `flora-api/src/oidc.rs` (3, also unsuppressed and also test-only — see NV-2), and `flora-core/src/utils/jwt.rs` (2, both already suppressed with `#[allow(...)]`). The single `.unwrap()` in `flora-core/src/utils/validation.rs:24` is production code, is pre-existing, and is out of scope for this research.

### 1.2 The lint policy and the CI gap

`Cargo.toml:40-90` (workspace lints, inherited by **all** seven members via `[lints] workspace = true`) sets:

- `clippy::expect_used = "deny"` (line 79) and `clippy::unwrap_used = "deny"` (line 78);
- `clippy::allow_attributes = "deny"` (line 61) — forces `#[expect]` over outer `#[allow]`;
- `clippy::allow_attributes_without_reason = "deny"` (line 66) — forces `reason = "..."`;
- `clippy::panic = "deny"` (line 80).

`.github/workflows/rust.yml:23-29` deliberately scopes strict clippy to production targets:

```yaml
# Strict clippy is applied to production targets (`--lib --bins`); test-only
# targets are still linted locally per cycle, but a parallel agent's test
# code currently trips `expect_used` under `--all-targets` (tracked NV).
...
- name: Clippy (strict, production targets)
  run: cargo clippy --workspace --lib --bins -- -D warnings
```

So the motivating command for Topic A is the local/per-cycle `cargo clippy --workspace --all-targets -- -D warnings`, not CI's own step. CI's actual state was not observed (NV-1); what is certain from the source is that its clippy step excludes tests. Closing the NV also lets the stale comment at lines 23-25 be removed (a separate, trivial follow-up — not done here).

### 1.3 Established codebase precedent

The workspace's own convention is a reason-bearing expectation, applied at the narrowest useful scope:

- `flora-api/src/extractors/auth.rs:259-265` — the closest analogue (test module, `unwrap_used` + `panic`):
  ```rust
  #[cfg(test)]
  #[expect(
      clippy::panic,
      clippy::unwrap_used,
      reason = "Test setup and assertions"
  )]
  mod tests {
  ```
- `flora-core/src/utils/jwt.rs:7-10` — crate-level inner attribute with a reason; lines 77-81 and 121-126 — per-function/per-statement `#[allow(clippy::allow_attributes, clippy::expect_used, reason = ...)]` (the `allow_attributes` exception is needed because those are outer `#[allow]`s, which `allow_attributes = "deny"` would otherwise reject).
- Other test modules use `#[expect(dead_code, reason = "...")]` (e.g. `flora-tests/src/contract/files.rs:130,135`).

Clippy's own docs confirm why `#[expect]` is preferred: it "suppress[es] the lint emission, but emit[s] a warning, if the expectation is unfulfilled" — so the suppression self-expires when the test stops using `expect` [S2]. The Rust Reference adds that `#[expect(C)]` "indicates that lint `C` is expected to be emitted" and emits `unfulfilled_lint_expectations` otherwise [S3].

---

## 2) Topic A — ranking the three candidate fixes

| Option | Invasiveness | Policy fit | Verdict |
|---|---|---|---|
| **A1. Module-level `#[expect(clippy::expect_used, reason = ...)]` on `mod tests`** | 4 additive lines; zero test-body edits; zero reversions | **Best.** Matches `auth.rs:259-265` exactly; satisfies `allow_attributes = "deny"` and `allow_attributes_without_reason = "deny"`; self-expiring per RFC 2383 / Clippy docs | **Choose this** |
| **A2. Test helper returning `Result`** (e.g. `fn body_string(res: Response) -> Result<String, ...>`) + `?` in `#[tokio::test] -> Result<(), ...>` | Rewrites both test bodies and their signatures; introduces an error type or `Box<dyn Error>`; larger diff in another agent's file | Technically purest (no suppression), but creates avoidable churn in code we were told not to revert, and the workspace already codifies narrow test-module expectation as acceptable | **Fallback / future cleanup**, not now |
| **A3. `unwrap_or_else`** | Similar churn to A2 | **Poor fit.** `to_bytes(...).await.unwrap_or_else(\|e\| panic!("..."))` still panics, so it dodges `expect_used` only to collide with the workspace's `clippy::panic = "deny"` (Cargo.toml:80). Replacing with a default value (e.g. `unwrap_or_default()`) would silently turn a test failure into a false negative | **Reject** |

### 2.1 Exact recommended edit (health.rs)

Insert the `#[expect(...)]` line between the existing `#[cfg(test)]` and `mod tests` at lines 122-123. Nothing else changes:

```rust
#[cfg(test)]
#[expect(
    clippy::expect_used,
    reason = "Test-only body extraction; infallible for these handler responses"
)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    // ... bodies unchanged, including all four .expect(...) calls ...
}
```

Notes that make this safe:

- The expectation **will be fulfilled** today (four `expect_used` emissions occur inside the module), so `unfulfilled_lint_expectations` will not fire [S3].
- `expect` is a rustc built-in, not an `#[allow]`, so `clippy::allow_attributes` is **not** triggered — unlike the outer `#[allow]`s in `jwt.rs` which must exempt themselves [S2]. No `clippy::allow_attributes` entry is needed here.
- Keep the reason factual and tied to the test context; avoid vague wording, because the workspace explicitly requires a reason and reviewers will read it.

### 2.2 "Do not revert other agents' work" constraint

This edit is strictly additive and scoped to the `#[cfg(test)]` module header. It does **not** touch:

- the `/metrics` handler, `PROMETHEUS_METRICS`, the `/llms.txt` manifest, or any production code in `health.rs` (all authored by the parallel agent);
- the four test assertions or their expected strings;
- the CI workflow or workspace lint levels (do **not** "fix" this by weakening `expect_used = "deny"` — that would lower the project's quality bar globally and contradict ADR-0001's verification-gate rule);
- `oidc.rs`, `jwt.rs`, or `validation.rs`.

The parallel agent's `/metrics` tests remain fully intact and continue to assert `flora_api_up 1` and `flora_api_info{service="flora-api"} 1`. (See NV-2 for the one adjacent test-only file that may need the same treatment.)

### 2.3 Verification after applying (not run during research)

```bash
cargo clippy --workspace --all-targets -- -D warnings   # the gate being closed
cargo clippy --workspace --lib --bins -- -D warnings    # must stay green (CI gate)
cargo fmt --all -- --check
cargo test -p flora-api health                          # both tests still pass
```

No `#[allow(...)]` suppression should be introduced; if `allow_attributes` complains, the edit is wrong (use `#[expect]`).

---

## 3) Why `#[expect]` is the policy-consistent choice

The workspace's `allow_attributes = "deny"` + `allow_attributes_without_reason = "deny"` pair is explicitly designed to make `#[expect(..., reason = "...")]` the only sanctioned local exception mechanism [F2][S2][S4]. Clippy's rationale: `#[expect]` "suppress[es] the lint emission, but emit[s] a warning, if the expectation is unfulfilled. This can be useful to be notified when the lint is no longer triggered" [S2]. The Rust Reference formalises the same contract, including the `unfulfilled_lint_expectations` warning and the rule that expectations are evaluated per lint in a list [S3]. A reason parameter is supported on all lint attributes and acts as documentation [S3].

A test helper returning `Result` is not *forbidden* by policy, but it is a larger, riskier edit in code another agent owns. Given the task's explicit non-reverting constraint and an existing in-repo precedent for exactly the panic/unwrap test-module case, `#[expect]` is the lower-risk closure.

---

## 4) Topic B — `/metrics`: keep the static stub

### 4.1 Current state (read-only)

`flora-api/src/routes/health.rs:96-120` returns a constant string with:

```
# HELP flora_api_up Service process up indicator.
# TYPE flora_api_up gauge
flora_api_up 1
# HELP flora_api_info Static service metadata.
# TYPE flora_api_info gauge
flora_api_info{service="flora-api"} 1
```

and header `text/plain; version=0.0.4; charset=utf-8`. Added in commit `15192c8fa` (`feat(observability): add GET /metrics endpoint to flora-api`) and advertised in `/llms.txt` (line 64). ADR-0001:32 names "tracing + `TraceLayer` per-request spans + static `/metrics`; no mandatory collector" as the authoritative Seed observability mechanism, with "OpenTelemetry exporter/collector" deferred. `Cargo.toml:160-163` records that phantom OTel deps were removed in cycle 026 and that a real OTLP exporter is gated on `otel_enabled` plus a receiver (NV). Cycle-026 research (`2026-11-16-026-otel-honest-wiring.md`) concluded: add `TraceLayer`, remove dead deps, defer OTLP until a collector exists.

### 4.2 The stub is *valid* Prometheus exposition

The Prometheus text format requires `# HELP`/`# TYPE` lines to precede the sample, allows only one `HELP` and one `TYPE` per metric name, and requires `\n` line endings [S5]. The stub satisfies all of these. It also uses the correct `Content-Type`: the spec states the text format's media type is `text/plain` with `version=0.0.4`, and that from Prometheus 3.0 a valid `Content-Type` header is **mandatory** or the scrape fails [S5]. `flora_api_info{service="flora-api"} 1` is a conventional "info" gauge pattern (constant 1 with a label) and does not require the OpenMetrics `info` type to be scrapeable as text.

The limit is honesty, not compliance: `flora_api_up 1` is a constant, so it reports liveness only when the process can answer HTTP at all — which `/health` and `/health/live` already do. It is a discovery/deprecated-style stub, not a measurement.

### 4.3 What "minimal real metrics" would actually cost

The cheapest credible real-metrics path in Rust is the official client library `prometheus` (currently 0.14.0) with `Registry` + `IntCounterVec`/`HistogramVec` + `TextEncoder`, wired in `AppState` and incremented in a middleware [S6][S7][S8]. That is **not** a one-line change:

1. **New dependency** in `flora-api/Cargo.toml` and root `[workspace.dependencies]`, which mutates `Cargo.lock` and re-opens two already-open Seed NVs: `cargo-vet` exemptions (ADR-0001:45, cycle-028 plan) and advisory/SBOM coverage.
2. **Lint surface:** the crate's `TextEncoder`/macro examples rely on `unwrap`/`expect` internally, and our own registration/encoding code would need `Result` handling under `unwrap_used`/`expect_used = "deny"`; `lazy_static` would add yet another dep (the crate also supports a `Registry` without it [S6]).
3. **Cardinality discipline:** a naive `method × path` counter vector on dynamic routes creates unbounded series. Doing it correctly means templated routes or a route-level label, which is real design work.
4. **No consumer:** there is no Prometheus server, dashboard, or alert rule in the Seed profile. ADR-0001 §Consequences keeps heavy observability out of the cheap-VPS target; the cycle-026 note says host/container resource data comes from Coolify Sentinel and process health from `/health*` + the stub.
5. **ADR drift rule:** ADR-0001:40-42 requires an explicit ADR for capability drift. Replacing "static `/metrics`" with a real registry changes the authoritative mechanism and should not be done silently.

`TraceLayer` is the orthogonal, near-zero-cost improvement: `tower-http` already declares the `trace` feature in the workspace (`Cargo.toml:141`), and `TraceLayer::new_for_http()` adds per-request spans (method/path/latency/status) to the existing `tracing` subscriber with **no exporter and no new crate** [S9]. The cycle-026 note rated this "do now (highest value)" but it was not applied in this checkout (the router still lacks `TraceLayer`). That is a separate NV, not part of this closure.

### 4.4 Recommendation and exact plan

**Now (this closure):** keep `/metrics` as-is. Optionally update the `/llms.txt` line to say "liveness/metadata indicators (static)" if reviewers want the manifest to be maximally honest — cosmetic only, not required.

**If/when a scraper actually exists, staged plan:**

1. Record the trigger in a new ADR (consumer deployed + owner for retention/alerting); update ADR-0001's mechanism table.
2. Add `prometheus = "0.14"` to `[workspace.dependencies]` and `flora-api`; re-run `cargo deny check bans licenses sources` and the `cargo-vet`/advisory gates (Apache-2.0 [S6] is compatible with the current `deny.toml` policy — verify, do not assume).
3. Build a `Metrics` struct holding a private `Registry`, an `IntCounterVec` keyed by `method` + **templated** route, and a `HistogramVec` for latency; register once at startup, carry it in `AppState`.
4. Replace the `metrics()` handler's `&'static str` body with `TextEncoder::encode(&registry.gather(), &mut buf)`; return `Vec<u8>` (or `String`) with the **same** `text/plain; version=0.0.4; charset=utf-8` header [S5][S6].
5. Increment/observe in one middleware after the response status is known; exclude `/metrics` itself to avoid self-scrape noise.
6. Add a unit test asserting `HELP`/`TYPE` uniqueness, trailing newline, and that the counter increases; keep the existing header assertion.
7. Only then consider OTLP/collector, per `2026-11-16-026-otel-honest-wiring.md` §4 — never in the Seed profile.

---

## 5) Risks / NVs

- **NV-1 (live verification gap):** `cargo clippy --version` timed out in this sandbox (toolchain/network resolution), so no live clippy diagnostic set was reproduced. Exact line numbers and the "only health.rs fails under `--all-targets`" claim come from static source inspection. The proposed edit should be accepted only after the §2.3 commands run green.
- **NV-2 (adjacent test-only violation):** `flora-api/src/oidc.rs:165-228` has three unsuppressed `.expect(...)` calls inside `#[cfg(test)] mod tests`, on the same production library target compiled under `--all-targets`. If `cargo clippy --workspace --all-targets -- -D warnings` reports them, apply the identical module-level `#[expect(clippy::expect_used, reason = "Test-only JWKS/token fixtures; valid by construction")]` at `oidc.rs:165-166`. This is not part of the health.rs edit and was not changed here.
- **NV-3 (`#[expect]` self-expiry):** if a future refactor removes all four `.expect` calls from `health.rs` tests but leaves the attribute, rustc's `unfulfilled_lint_expectations` warning will fire (and `-D warnings` will fail). That is the intended signal, not a bug [S3].
- **NV-4 (reason wording):** the exact `reason = "..."` text is a recommendation; any truthful, specific string satisfies `allow_attributes_without_reason`. Keep it short to respect `max_width = 100` (multi-line form shown in §2.1).
- **NV-5 (multilingual coverage gap):** the primary technical sources consulted are English-only. Attempts to fetch translated `#[expect]` reference pages (`/pt-BR/`, `/zh-CN/` Rust Reference) and Prometheus `/es/` exposition-format page returned **404** in this environment. No non-English source was used to infer a lint or format fact; this is reported as a coverage gap, not an inferred translation.
- **NV-6 (metrics value):** no scrape was performed and no scraper is deployed; the claim that the stub is "sufficient for Seed" rests on ADR-0001 and the cycle-026 analysis, not a measured benchmark on the 2 vCPU/4 GB VPS.
- **NV-7 (CI comment drift):** `rust.yml:23-25` still says test code trips `expect_used` under `--all-targets`. After the fix lands, that comment is stale; removing it is a separate, low-risk CI-doc edit not made here.
- **NV-8 (concurrency):** per the task framing, a parallel agent owns the `/metrics` tests in `health.rs`. Writing this file must be coordinated so the additive attribute does not conflict with any in-flight edit to the same module header.
- **NV-9 (production `.unwrap()`:)** `flora-core/src/utils/validation.rs:24` uses `.unwrap()` in **production**, guarded only by an earlier `!email.contains('@')` check, while `unwrap_used = "deny"` is inherited by `flora-core`. Since CI already runs `--lib --bins` with `-D warnings`, a clean CI run would imply this is either already accepted or in fact clippy-invisible (e.g. const-propagated). This is a pre-existing, out-of-scope item for this closure, but it must be checked before claiming a fully green production-target clippy gate.

---

## 6) Sources

### Primary (external)

| ID | Source | Claim used |
|---|---|---|
| S1 | [Clippy lint list — `expect_used` / `unwrap_used` / restriction group](https://rust-lang.github.io/rust-clippy/master/index.html#expect_used) | `expect_used`/`unwrap_used` are restriction lints; workspace denies them |
| S2 | [Clippy `allow_attributes`](https://rust-lang.github.io/rust-clippy/master/index.html#allow_attributes) and [`allow_attributes_without_reason`](https://rust-lang.github.io/rust-clippy/master/index.html#allow_attributes_without_reason) | `#[expect]` preferred over outer `#[allow]`; inner `#![allow]` exempt; reason required; unfulfilled expectations warn |
| S3 | [The Rust Reference — Diagnostic attributes / `#[expect]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html) | `#[expect(C)]` semantics, `unfulfilled_lint_expectations`, `reason` parameter, per-lint evaluation |
| S4 | [RFC 2383 — lint reasons](https://rust-lang.github.io/rfcs/2383-lint-reasons.html) (linked from S2) | rationale for reason-bearing expectations |
| S5 | [Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats/) | text format `text/plain; version=0.0.4`, HELP/TYPE rules, trailing `\n`, mandatory valid Content-Type from Prometheus 3.0 |
| S6 | [docs.rs `prometheus` 0.14.0](https://docs.rs/prometheus/latest/prometheus/) | `Registry`, `TextEncoder`, `IntCounterVec`, `HistogramVec`, Apache-2.0, registration/encode example |
| S7 | [Prometheus — Client libraries](https://prometheus.io/docs/instrumenting/clientlibs/) | Rust client library is the official instrumentation path; implementing the format yourself is allowed but "avoid dependencies" is a valid reason |
| S8 | [tikv/rust-prometheus repository](https://github.com/tikv/rust-prometheus) | maintained official Rust client referenced by S7 |
| S9 | [docs.rs `tower_http::trace::TraceLayer`](https://docs.rs/tower-http/latest/tower_http/trace/struct.TraceLayer.html) | `new_for_http()` exists behind `trace` feature; high-level `tracing` per request; no exporter involved |

### Local (read-only)

- **F1** `flora-api/src/routes/health.rs:64,96-120,122-168` — stub payload, header, and four unsuppressed test `expect`s.
- **F2** `Cargo.toml:40-90,141,160-163` — workspace lint policy, `tower-http` trace feature, OTel comment.
- **F3** `flora-api/Cargo.toml:16-17,27-71` — inherits lints; dependency/dev-dependency surface.
- **F4** `.github/workflows/rust.yml:22-29` — CI clippy scoped to `--lib --bins`; stale NV comment.
- **F5** `flora-api/src/extractors/auth.rs:259-265`; `flora-core/src/utils/jwt.rs:7-10,76-89,106-130` — in-repo suppression precedent.
- **F6** `flora-api/src/oidc.rs:165-228`; `flora-core/src/utils/validation.rs:24` — remaining `expect`/`unwrap` sites (NV-2).
- **F7** `docs/adr/0001-seed-v01-mechanism-substitutions.md:32,40-42,45,47-57` — authoritative observability mechanism, drift rule, open NVs.
- **F8** `docs/research/2026-11-16-026-otel-honest-wiring.md` — cycle-026 conclusion (keep tracing + `/metrics`; add `TraceLayer`; defer OTLP).
- **F9** `git log -- flora-api/src/routes/health.rs` — `/metrics` authored in `15192c8fa`; working tree clean at research time.
