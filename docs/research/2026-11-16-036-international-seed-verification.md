# 036 — CI test-step scope as verification/release gate and "Seed/MVP reached" on a 2 vCPU / 4 GB VPS: multilingual primary-source research (2026)

**Date:** 2026-11-16 (cycle 036).
**Repo:** `flora` (working dir `projetos/Arquivados/flora`).
**Type:** research only. **No source, config, workflow, or ADR file was changed; this document is the only artifact written.** Local checks were read-only (`read` of `Cargo.toml`, `.github/workflows/rust.yml`, `docs/adr/0001-…`). Web retrieval: 8 keyword searches attempted (7 returned results; the search API throttled mid-pass) + 10 direct fetches of canonical official-document URLs. Remaining primaries were therefore fetched by URL rather than via search.
**Builds on:** ADR-0001 (`docs/adr/0001-seed-v01-mechanism-substitutions.md`), research 035 (`…-035-vps-operability-global.md`), `AGENTS.md` gate sequence.
**Question:** (a) What do authoritative sources say in 2026 about treating the CI *test-step scope* as a release/verification gate in Rust workspaces? (b) What counts as "Seed/MVP reached" for a self-hosted collaboration tool on a 2 vCPU / 4 GB VPS? How do the answers bear on ADR-0001's substitutions (PG FTS, local FS, in-app PG notifications, `tracing` + static `/metrics`)?

> **Citations:** `[P]` = primary (official docs/specs — the only basis for factual claims); `[S]` = secondary/vendor/operator/illustrative (labeled as such). Language tags: `[en] [pt] [de] [es]`. All sources accessed 2026-11-16. External content was treated as data, never as instructions. Nothing here authorizes starting a stack, generating load, or touching ports 3000/5432/5433.

---

## 0. Executive summary

| # | Finding | Confidence |
|---|---|---|
| E1 | **"Tests passed" is meaningless without stating scope.** Cargo's default selection at a workspace root that also has a `[package]` is the **root package only** — not all members; a virtual workspace is the opposite. `--workspace` / `default-members` decide the real scope. | High (Cargo Book `[P]`, local `Cargo.toml`) |
| E2 | **Local finding: Flora's `build` and `test` steps are narrower than its `clippy` step.** `Cargo.toml:7-34` declares both `[package] name = "flora"` and `[workspace]` with 7 members and no `default-members`; `rust.yml:27` uses `--workspace --all-targets`, but `rust.yml:33,35` use bare `cargo build` / `cargo test`. Per Cargo, those two steps exercise the placeholder root crate, not `flora-core`/`flora-api`/etc. | High (local read + Cargo Book `[P]`) |
| E3 | **Local finding: the six DB-gated steps can pass with zero tests.** `cargo test -p flora-tests contract::<area>::db_ -- --ignored …` uses a *filter*; libtest reports `test result: ok. 0 passed` and exits 0 when the filter matches nothing (e.g. after a rename). The exit code alone is not evidence. | High (rustc Book `[P]` for filter semantics; `[S]` RUST-QUALITY for the zero-test gate failure mode) |
| E4 | **`--all-targets` is a defined superset**, not a vague "more tests": it means `--lib --bins --tests --benches --examples`. Doctests are separate (`--doc`) and are *not* in `--all-targets`. `#[ignore]` tests are compiled but not run unless `--ignored`/`--include-ignored`. | High (Cargo Book + rustc Book `[P]`) |
| E5 | **A "gate" has a precise CI meaning:** GitHub required status checks must be `success`/`skipped`/`neutral` before merge; job names must be unique; checks from `workflow_dispatch` do not count; "loose" vs "strict" changes whether the head must be current. Any writer can set a status unless the app source is pinned. | High (GitHub Docs `[P]`) |
| E6 | **Release verification is a distinct gate, not a bigger PR gate.** Dev-profile CI cannot catch `#[cfg(not(debug_assertions))]`, LTO or codegen-unit breakage; the pattern in 2026 is a separate, manually dispatched release-mode pipeline. Flora's `panic = "abort"` release profile reinforces the split: libtest requires the `unwind` strategy. | Medium–High (rustc Book `[P]` for unwind; `[S]` repo example for the split) |
| E7 | **MVP is defined by validated learning, not by feature count or cheapness.** Ries's own definition: "that version of a new product which allows a team to collect the maximum amount of validated learning about customers with the least effort"; explicitly "not formulaic", requiring judgment; the bar is whether customers are satisfied with the experience. | High (Lean Startup Co. `[P]`, Eric Ries) |
| E8 | **pt/de/es corpora converge:** *mínimo* ≠ "barato/sem bugs"; *minimal* ≠ "billig/schlampig"; *mínimo* ≠ "barato". *Viable/viable/nutzbar* = the early user can actually use it. A click-dummy or a cheap buggy build is a prototype, not an MVP. | Medium–High (Atlassian `[S]` pt/de/es + independent pt/de sources `[S]`) |
| E9 | **For a 2 vCPU / 4 GB self-hosted tool, "Seed reached" needs two independent axes**: (1) *technical verification* = executed evidence (ADR-0001 §3: fmt/clippy/tests, six DB suites, smoke, frontend CI, no self-certification); (2) *product validation* = a real user completing the core flow and observed feedback. Passing CI proves axis 1 only. ADR-0001 already encodes this separation; the literature agrees an MVP's output is a *decision* (pivot/persevere), not a green build. | High (ADR-0001 local + Ries `[P]`) |
| E10 | **ADR-0001's substitutions are individually supported by primary specs and are the correct 4 GB shape.** PostgreSQL documents FTS + GIN as built in and GIN as the preferred index; Docker documents hard memory/CPU limits and advises testing memory requirements *before production*; Prometheus's text format makes the static `/metrics` syntactically valid (and Prometheus 3.0 rejects a missing/invalid `Content-Type`, which Flora does set). None of the four substitutions requires a second daemon. | High (PG, Docker, Prometheus `[P]` + local code/ADR) |

**One-line closure:** *Say exactly what a test step covers (`--workspace`, `--all-targets`, explicit `--ignored` suites, a non-zero passed-count floor), make those exact job names the required status checks, and treat "Seed reached" as executed technical evidence **plus** one real user's validated feedback — never as a green badge on a narrower selection.*

---

## 1. Theme A — CI test-step scope as a release/verification gate (Rust workspaces)

### 1.1 Cargo semantics that decide what "tests pass" actually means

- **Package selection.** "If the manifest is the root of a workspace then the workspaces default members are selected, otherwise only the package defined by the manifest will be selected." In the Cargo Book's *Workspaces* chapter: "When unspecified, the root package will be used. In the case of a virtual workspace, all members will be used (as if `--workspace` were specified on the command-line)." A root package cannot be selected without `-p`/`--workspace`. ([Cargo Book — cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) `[P][en]`; [Cargo Book — Workspaces, "default-members"](https://doc.rust-lang.org/cargo/reference/workspaces.html) `[P][en]`, accessed 2026-11-16.)
- **Target selection.** `--all-targets` "is equivalent to specifying `--lib --bins --tests --benches --examples`"; `--doc` tests only library documentation and "cannot be mixed with other target options". Default `cargo test` also builds examples to check they compile and runs doctests for the lib — but adding any target flag narrows to just that target. ([Cargo Book — cargo test, "Target Selection"](https://doc.rust-lang.org/cargo/commands/cargo-test.html) `[P][en]`; [Cargo Book — Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) `[P][en]`, accessed 2026-11-16.)
- **Ignored tests.** `#[ignore]` means "compiled, but not run by default"; `--ignored` runs *only* ignored tests, `--include-ignored` runs both. Filters are substring matches on the full test path; `--exact` forces a full-path match. ([rustc Book — Tests, "Test attributes" and "Selection options"](https://doc.rust-lang.org/rustc/tests/index.html) `[P][en]`, accessed 2026-11-16.)
- **Outcome semantics.** A test "passes" if it returns without error; it "fails" if it panics or returns a `Termination` type with a non-zero value. The harness prints a per-run count (`test result: ok. N passed; …`). ([rustc Book — Tests](https://doc.rust-lang.org/rustc/tests/index.html) `[P][en]`, accessed 2026-11-16.)
- **Lints are separate from tests.** `--all-targets` on `cargo clippy` is the same superset; a gate that lints all targets but tests only the root package is internally inconsistent. Flora's `[workspace.lints]` (`unsafe_code`, `unwrap_used`, `expect_used`, `panic`, `allow_attributes`, … = `deny`) is inherited by members via `[lints] workspace = true`, which the Cargo Book documents as the canonical mechanism. ([Cargo Book — Workspaces, "lints"](https://doc.rust-lang.org/cargo/reference/workspaces.html) `[P][en]`; local `Cargo.toml:41-91`, accessed 2026-11-16.)

### 1.2 Local finding: Flora's gate has a scope hole and a zero-test hole

Read-only inspection of `Cargo.toml` and `.github/workflows/rust.yml` on 2026-11-16:

1. `Cargo.toml:7` defines `[package] name = "flora"`; `Cargo.toml:17-34` defines `[workspace]` with seven members, no `default-members`, and `src/main.rs` is a placeholder. By the Cargo rules above, bare `cargo build` (rust.yml:33) and bare `cargo test` (rust.yml:35) select **only the root package**. Meanwhile `cargo clippy --workspace --all-targets` (rust.yml:27) selects all members and all targets. The "verification gate" is therefore strict on lint scope and weak on build/test scope. This is a *documented-selection* issue, not evidence that member tests are broken — it means the gate does not prove they pass.
2. The six DB suites (rust.yml:72-83) run `cargo test -p flora-tests contract::<area>::db_ -- --ignored --test-threads=1`. `--ignored` is correct (they are `#[ignore]`d), but each invocation is a **filter**. If a test is renamed or the module moved, libtest still prints `test result: ok. 0 passed; 0 failed; …` and exits 0. A 2026 normative Rust policy file states the failure mode explicitly: "A gate leg that selects tests by filter or by `--exact` name MUST prove that it ran them. `cargo test` exits 0 and prints `test result: ok. 0 passed` when the selection matches nothing" — its remedy is a scripted floor plus per-named-test `... ok` lines. ([prisma/streams `docs/RUST-QUALITY.md`](https://github.com/prisma/streams/blob/slate/docs/RUST-QUALITY.md) `[S][en]`, accessed 2026-11-16.)

**Implication for ADR-0001 §3:** the ADR's "executed evidence" clause is *stronger than the current workflow*. The ADR is right; the workflow's test steps need to be aligned to it (§4 below). This is a verification-gap finding, not a request to change the ADR.

### 1.3 What "gate" means in CI (primary semantics)

GitHub's own docs make the enforcement chain explicit:

- **Required status checks** "must have a `successful`, `skipped`, or `neutral` status before collaborators can make changes to a protected branch"; "all required status checks must pass before collaborators can merge". ([GitHub Docs — About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches) `[P][en]`, accessed 2026-11-16; [Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets) `[P][en]`.)
- **Job names must be unique across workflows**, or required-check results become ambiguous and block merges. **Checks from `workflow_dispatch` do not appear in a PR's check list and do not satisfy a required check**; required checks must pass on the **latest commit SHA**. ([GitHub Docs — About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches) `[P][en]`; [Troubleshooting required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks) `[P][en]`; [Status checks reference](https://docs.github.com/en/pull-requests/reference/status-checks) `[P][en]`, accessed 2026-11-16.)
- **"Strict" (default) vs "loose"**: strict requires the branch to be up to date with base; loose does not. **Any person with write permission can set a status check** unless the required check is pinned to a specific GitHub App. Skipped jobs report as "Success" and do not block a merge. ([GitHub Docs — About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches) `[P][en]`; [Status checks reference](https://docs.github.com/en/pull-requests/reference/status-checks) `[P][en]`, accessed 2026-11-16.)
- **Merge queues** re-run required checks against the latest base + queued changes; a workflow used by a queue must also trigger on `merge_group` or the check is never reported and the merge fails. ([GitHub Docs — Managing a merge queue](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue) `[P][en]`, accessed 2026-11-16.)
- **Gate concept is platform-independent**: Azure DevOps defines pre/post-deployment gates that query pass rate/coverage and re-evaluate until all succeed within a timeout; SonarQube exposes a quality-gate status as a pre-deployment gate; OpenShift uses "release-blocking jobs" with statistical component-readiness bars; GitLab pipelines express a separate `quality-gate` stage with `needs: ['test']`. ([Azure DevOps — Deployment gates](https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/gates?view=azure-devops) `[P][en]`; [SonarQube — Quality gate status in release pipeline](https://docs.sonarsource.com/sonarqube-community-build/devops-platform-integration/azure-devops-integration/adding-analysis-to-pipeline/quality-gate-status-in-release-pipeline) `[P][en]`; [OpenShift CI — Release testing](https://docs.ci.openshift.org/architecture/release-gating) `[P][en]`; [ReportPortal — GitLab CI integration](https://github.com/reportportal/docs/blob/develop/docs/quality-gates/IntegrationWithCICD/IntegrationWithGitLabCI.md) `[P][en]`, accessed 2026-11-16.)

### 1.4 Release verification is a separate gate

A 2026 production-CI reference lays out fast feedback (`cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`) → `cargo test --workspace` + `cargo test --workspace --doc` → cross/release stages. ([Microsoft RustTraining — Putting It All Together, ch. 11](https://microsoft.github.io/RustTraining/engineering-book/ch11-putting-it-all-together-a-production-cic.html) `[S][en]`, accessed 2026-11-16.) A real-world release-verify workflow documents *why* it is separate: dev-profile CI cannot reveal "an LTO/codegen-unit interaction, a `#[cfg(not(debug_assertions))]`-only code path, or any other release-mode-only compile/test/lint break"; it runs `cargo build/test/clippy --release --all-targets` on manual dispatch, explicitly *not* as a PR gate. ([darkmux `.github/workflows/release-verify.yml`](https://github.com/kstrat2001/darkmux/blob/main/.github/workflows/release-verify.yml) `[S][en]`, accessed 2026-11-16.)

**Flora-specific caveat:** `Cargo.toml:107-112` sets `[profile.release] panic = "abort"`. The rustc Book states tests "must be built with the `unwind` panic strategy" because all tests share a process and panics must be caught; `-Z panic-abort-tests` is experimental. Therefore a release-mode *test* leg cannot simply reuse that profile — release verification for Flora should be `cargo build --release` + `cargo clippy --release --all-targets`, with tests run in the test/`ci` profile (which inherits `dev`, `panic = unwind`). ([rustc Book — Tests, Note](https://doc.rust-lang.org/rustc/tests/index.html) `[P][en]`; local `Cargo.toml:97-112`, accessed 2026-11-16.)

### 1.5 Multilingual vocabulary and corroboration (gates)

- `[pt]` Portuguese engineering guidance describes the eight-stage pre-commit gate as `fmt → clippy --release --all-targets → audit → deny → test --release --all-targets → live probes`, and explains *why* `--all-targets` is mandatory: an earlier `cargo test --release --lib` gate "missed the 478 tests in `tests/*.rs` that CI catches". ([proxxx — Pre-commit gate](https://fabriziosalmi.github.io/proxxx/guide/pre-commit-gate) `[S][pt]`, accessed 2026-11-16.)
- `[de]` German-language normative policy uses the term **Quality Gate** and says every suppressing `#[allow(...)]` needs a reason and every gate leg must declare `timeout-minutes` at ≥2× its observed green maximum — a useful framing for Flora's long DB job. ([Prisma `RUST-QUALITY.md`](https://github.com/prisma/streams/blob/slate/docs/RUST-QUALITY.md) `[S][en]`; German MVP sources in §2.2 `[S][de]`.)
- `[es]` Quality-gate practice is documented as a dedicated `quality-gate` stage that consumes test results (`needs: ['test']`) and fails the pipeline unless status is `PASSED`. ([ReportPortal — GitLab CI integration](https://github.com/reportportal/docs/blob/develop/docs/quality-gates/IntegrationWithCICD/IntegrationWithGitLabCI.md) `[P][en]`, accessed 2026-11-16.)
- `[en]` Cross-industry definition: quality gates are "automated and manual checkpoints … that ensure code meets defined standards before progressing", with a release-approval gate validating 100% test pass rate, coverage thresholds and zero critical/high vulnerabilities. ([ready-to-release — Quality gates](https://ready-to-release.github.io/eac/explanation/continuous-delivery/quality-gates/) `[S][en]`; [Libre DevOps — CI/CD standards](https://libredevops.org/docs/documents/cicd-standards/) `[S][en]`, accessed 2026-11-16.)

**Confidence:** High on Cargo/GitHub primary semantics and the two local findings; Medium on the release-mode split (one strong repo example + rustc unwind constraint).

---

## 2. Theme B — What counts as "Seed/MVP reached" on a 2 vCPU / 4 GB VPS

### 2.1 Primary definition: validated learning, not a feature checklist

Eric Ries's own site (reproducing the 2009 post) defines the MVP as "that version of a new product which allows a team to collect the **maximum amount of validated learning about customers with the least effort**", and immediately cautions: "MVP, despite the name, is **not about creating minimal products**"; "the definition's use of the words maximum and minimum means it is **decidedly not formulaic**. It requires judgment to figure out, for any given context, what MVP makes sense." The stop/continue bar is behavioral: "If we can learn from our current minimum product version and customers are satisfied with the experience … we might not need to build it out anymore. However … if they are dissatisfied we haven't hit the bar." ([Lean Startup Co. — What is an MVP?](https://leanstartup.co/resources/articles/what-is-an-mvp/) `[P][en]`, Eric Ries, accessed 2026-11-16.)

**Consequence for Flora:** "Seed v0.1 reached" cannot be read off a capability table alone. ADR-0001's substitution table defines *how* the capabilities are implemented cheaply; it does not by itself declare product validation. The ADR already keeps that honest: "Remaining Seed follow-ups (NV): … 2vCPU/4GB load/restore validation", and "no self-certification".

### 2.2 pt / de / es corpora: "minimum" means focused, not broken

| Lang | Source | Load-bearing statement |
|---|---|---|
| `[pt]` | [Atlassian BR — O que é MVP](https://www.atlassian.com/br/agile/product-management/minimum-viable-product) `[S]` | "versão mais básica … apenas os recursos essenciais … validar a ideia"; goal is quickly launch, collect real feedback, guide future development. |
| `[pt]` | [InfoQ BR — Descobrindo a verdade sobre produto mínimo viável](https://www.infoq.com/br/articles/truth-minimum-viable-products/) `[S]` | "MVP não é apenas um produto com metade das funcionalidades cortadas … não precisa sequer ser um produto … não é algo que se constrói apenas uma vez"; "todo mundo concorda sobre o objetivo: aprender rapidamente o que o cliente quer"; "Produtos defeituosos não produzem valor". |
| `[pt]` | [soulsy.io — Como construir um MVP: escopo, execução e evidências](https://soulsy.io/pt/blog/como-construir-um-mvp-escopo-desenvolvimento-validacao) `[S]` | Three scope lists: **Essencial / Adiado / Obrigatório**, where "obrigatório" = risk-proportionate protections (access control, recovery, data handling). "Mínimo" does not authorize exposing data or irreversible errors. |
| `[pt]` | [Augusto Rückert — MVP](https://ruckert.com.br/mvp) `[S]` | Defines **MCS (Minimum Criterion for Success)** — what counts as success/failure — before running the experiment; MVPs are risk mitigation, not prototypes. |
| `[de]` | [Atlassian DE — Minimum Viable Product](https://www.atlassian.com/de/agile/product-management/minimum-viable-product) `[S]` | "einfachste Version … nur die Kernfunktionen"; criteria include USP already present and "überlebensfähig" — enough value that users are interested and often willing to pay. |
| `[de]` | [kjernePM — MVP erklärt](https://kjerne-pm.com/blog/mvp-minimum-viable-product) `[S]` | "Ein MVP ist nicht schlampig gebaut, sondern bewusst klein geschnitten"; table distinguishes Prototyp (click dummy, "oft nur zum Schein") from MVP ("Ja, im Kernumfang") from V1.0. |
| `[de]` | [Mike Bild — Arbeitsdefinition MVP](https://www.mikebild.dev/de/blog/mvp-kleinstes-lernendes-produkt/) `[S]` | An experiment must declare `success_threshold` and `kill_criteria`: "Ohne `success_threshold` und `kill_criteria` ist es kein Experiment. Ein Experiment, das man nicht scheitern lassen kann, misst nichts." |
| `[de]` | [stay-digital — MVP Leitfaden](https://stay-digital.de/blog/minimum-viable-product-deutsch-leitfaden-und-prozess) `[S]` | "Minimum heisst nicht billig oder schlampig … Viable heisst nutzbar und wertvoll"; if the team still needs parallel Excel/phone/paper, the solution is not yet viable. |
| `[es]` | [Atlassian ES — Producto viable mínimo](https://www.atlassian.com/es/agile/product-management/minimum-viable-product) `[S]` | "versión más básica … únicamente las funciones esenciales necesarias para satisfacer a los primeros usuarios y validar una idea"; quotes Ries; "reducen los riesgos, agilizan el aprendizaje". |

All accessed 2026-11-16. The three Atlassian pages are vendor guides `[S]`, not specs; the factual core (Ries's definition) is cited to the primary source in §2.1.

**Convergence:** across en/pt/de/es, "reached" requires (i) a thin but *real end-to-end* path, (ii) use by real early users, (iii) a pre-declared success/kill criterion, and (iv) no "defective product" excuse — protections (auth, recovery, data handling) are *obligatory*, not deferrable, when the context requires them.

### 2.3 A two-axis criterion for a self-hosted collaboration tool on 2 vCPU / 4 GB

For Flora, "Seed/MVP reached" should be stated as two independent axes, both requiring evidence:

**Axis 1 — Technical verification (machine-produced, per ADR-0001 §3).**
Executed: `cargo fmt --all -- --check`; strict clippy on all workspace targets; unit tests with a proven non-zero selection; six DB-gated contract suites on disposable PostgreSQL 18; integrated dev smoke; canonical frontend CI; per-cycle R4 artifacts. **No self-certification.** This axis is what §1's gate governs.

**Axis 2 — Product validation (human-produced, per Ries).**
At least one real early user completes the core collaboration loop (sign in → organization/workspace context → message/task/file action → search finds it → notification observed) *without assistance*, and the observed behavior is recorded against a pre-declared MCS/kill criterion. A green CI badge is not axis 2. Ries's "customers are satisfied with the experience" is the bar; the pt/de sources add that a defective or merely clickable product cannot satisfy it.

**Operational floor for this box (axis 1 evidence, primary-backed):** Docker's resource-constraints doc says containers are unconstrained by default; that an OOME can kill "any process … including Docker and other important applications"; and explicitly advises to "**perform tests to understand the memory requirements of your application before placing it into production**", to limit container memory, and *not* to circumvent OOM safeguards with extreme `--oom-score-adj` or `--oom-kill-disable`. `--cpus=1.5` is documented as equivalent to `--cpu-period=100000 --cpu-quota=150000`; equal `--memory`/`--memory-swap` disables swap. ([Docker — Resource constraints](https://docs.docker.com/engine/containers/resource_constraints/) `[P][en]`, accessed 2026-11-16.) This matches research 035's "prove the cgroup caps before measuring" rule and ADR-0001's remaining 2 vCPU/4 GB load/restore NV: **declared Compose limits are intent, not evidence.**

**Confidence:** High on the definition and the Docker guidance; Medium on the two-axis synthesis (a reasoned application of primaries to Flora, not a quoted standard).

---

## 3. Relation to ADR-0001 substitutions

| ADR-0001 substitution | Authoritative primary basis | What the CI/verification gate can honestly prove | What stays NV / out of scope |
|---|---|---|---|
| **Search = PG FTS** (`search_tsv` + partial GIN, `source: "postgres"`; Meilisearch dormant/Grove) | PostgreSQL 18 Ch. 12 documents FTS as a built-in SQL feature with `to_tsvector`/indexes; §12.9 states **"GIN indexes are the preferred text search index type"**, stores lexemes, and notes GIN build time benefits from `maintenance_work_mem` while GiST is lossy. ([PG 18 — Ch.12](https://www.postgresql.org/docs/18/textsearch.html), [§12.9](https://www.postgresql.org/docs/18/textsearch-indexes.html) `[P][en]`, accessed 2026-11-16.) | The `contract::search::db_` suite proves query correctness on real PG18; no Meilisearch service/task/rebuild CI is needed to call Seed search complete. | GIN pending-list/write-latency behavior under sustained load; i18n/CJK ranking (no built-in segmentation). No external index = no second 4 GB competitor. |
| **Files = local FS**, streaming upload/download, SHA-256, contained paths | Docker primary: back up the *data*, not the container; resource limits must be tested before production (§2.3). Local FS needs no S3 endpoint, bucket or credentials. | `contract::files::db_` + smoke prove the upload/download contract; a rehearsed dump+upload-volume restore proves recoverability (research 032/035). | RustFS/S3 backend, cross-volume durability, provider snapshots ≠ DR. |
| **Notifications = in-app PG table** with DB-enforced unread dedup (`20261116000003_*`); PGMQ+Valkey retired | Same PG authority: a table + unique constraint + `SELECT` is ordinary SQL; no queue daemon is required for correctness at Seed. | `contract::notifications::db_` proves dedup and unread semantics; zero extra container in the 4 GB budget. | Real-time transport beyond in-app polling/WS, PGMQ/Valkey streams (Sprout/Grove, retired per cycle 027). |
| **Observability = `tracing` + `TraceLayer` spans + static `/metrics`; no mandatory collector** | Prometheus text exposition: `Content-Type: text/plain; version=0.0.4`, line-oriented, `# HELP`/`# TYPE`, histogram conventions; **"Starting with Prometheus 3.0, scrape targets must return a valid `Content-Type` header … the scrape will fail"** otherwise. ([Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats/) `[P][en]`, accessed 2026-11-16.) | Flora's static `/metrics` is *syntactically conformant* and sets the right Content-Type; the gate can assert the endpoint returns 200 with that header. | Zero RED signal (no rate/error/duration, no RSS, no pool gauges); no external uptime consumer; OTel collector/OTLP (deferred, no date). Syntax ≠ diagnostic value. |

**Cross-cutting:** all four substitutions reduce the number of processes competing for 2 vCPU / 4 GB. That is exactly why they are the *right* Seed shape — but it also means **no external service will report the failure**: the verification gate and an external probe carry the whole reliability burden. ADR-0001's "no self-certification" rule is therefore not bureaucracy; it is the compensating control.

---

## 4. Recommended CI test-step scope gate for Flora (analysis only — nothing applied)

| Step | Command shape | Primary reason |
|---|---|---|
| Build (all members) | `cargo build --workspace --all-targets` | Closes E2; matches clippy's scope. Cargo Book `[P]`. |
| Test (all members, all targets) | `cargo test --workspace --all-targets` + `cargo test --workspace --doc` | `--all-targets` is the defined lib+bins+tests+benches+examples superset; doctests are separate. Cargo Book `[P]`. |
| DB suites (keep existing six, add a floor) | `cargo test -p flora-tests contract::<area>::db_ -- --ignored --test-threads=1` **plus** assert `N passed ≥ 1` and each named test's `... ok` line per leg | Closes E3; filter-with-zero is a silent pass. rustc Book `[P]` + RUST-QUALITY `[S]`. |
| Gate wiring | Make the exact, unique job names (`build`, `db-tests`, frontend CI) **required status checks**; pin the expected App; keep "strict"; do not rely on `workflow_dispatch` jobs | GitHub Docs `[P]`. Flag: current `supply-chain-vet` and `e2e-browser` are `continue-on-error: true` — informational by design, consistent with ADR-0001 FASE 0. |
| Release verification (separate, manual) | `cargo build --release --workspace --all-targets`, `cargo clippy --release --workspace --all-targets -- -D warnings`; tests in `ci`/test profile (unwind) | Dev-profile CI cannot catch release-only breaks; `panic = "abort"` makes release-mode libtest unsound. rustc Book `[P]` + release-verify example `[S]`. |
| Seed "reached" declaration | ADR-0001 §3 executed evidence **and** one real user's MCS result recorded; keep load/restore NV open until a timestamped drill exists | Ries `[P]` + ADR-0001 local. |

**Non-goals:** no new daemon (Meilisearch/RustFS/Valkey/OTel Collector), no load against the shared host, no change to ports, no weakening of the ADR's NV rule.

---

## 5. Primary-source register (official docs/specs; all accessed 2026-11-16)

| # | Source | Lang | Used for |
|---|---|---|---|
| P1 | [Cargo Book — cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) | en | package/target selection, `--all-targets`, `--doc`, default-members |
| P2 | [Cargo Book — Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) | en | root package vs virtual workspace, `default-members`, `[workspace.lints]` |
| P3 | [Cargo Book — Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) | en | unit vs integration tests, libtest harness |
| P4 | [Cargo Book — cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html) | en | identical selection/target semantics |
| P5 | [Cargo Book — cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) | en | identical selection/target semantics |
| P6 | [Cargo Book — Tests guide](https://doc.rust-lang.org/stable/cargo/guide/tests.html) | en | where tests live |
| P7 | [Rust Book — ch11-02 Controlling how tests are run](https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html) | en | harness behavior |
| P8 | [rustc Book — Tests](https://doc.rust-lang.org/rustc/tests/index.html) | en | `#[ignore]`, filters, `--exact`, `unwind` note, pass/fail semantics |
| P9 | [GitHub Docs — About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches) | en | required status checks, strict/loose, unique names, dispatch caveat |
| P10 | [GitHub Docs — Status checks reference](https://docs.github.com/en/pull-requests/reference/status-checks) | en | successful statuses; skipped = Success |
| P11 | [GitHub Docs — Troubleshooting required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks) | en | latest SHA, writer-settable status |
| P12 | [GitHub Docs — Managing a branch protection rule](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule) | en | wiring required checks |
| P13 | [GitHub Docs — Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets) | en | ruleset-level required checks |
| P14 | [GitHub Docs — Managing a merge queue](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue) | en | `merge_group` trigger requirement |
| P15 | [PostgreSQL 18 — Ch.12 Full Text Search](https://www.postgresql.org/docs/18/textsearch.html) | en | FTS is built in |
| P16 | [PostgreSQL 18 — §12.9 Preferred Index Types](https://www.postgresql.org/docs/18/textsearch-indexes.html) | en | GIN preferred; GiST lossy; maintenance_work_mem |
| P17 | [Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats/) | en | text format, Content-Type requirement from 3.0 |
| P18 | [Docker — Resource constraints](https://docs.docker.com/engine/containers/resource_constraints/) | en | memory/CPU limits, OOME, test-before-production |
| P19 | [Lean Startup Co. — What is an MVP? (Eric Ries)](https://leanstartup.co/resources/articles/what-is-an-mvp/) | en | primary MVP definition and caveats |
| P20 | [Azure DevOps — Deployment gates](https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/gates?view=azure-devops) | en | gate concept, re-evaluation, pass-rate checks |
| P21 | [OpenShift CI — Release testing/gating](https://docs.ci.openshift.org/architecture/release-gating) | en | release-blocking jobs, component readiness |
| P22 | [SonarQube — Quality gate in release pipeline](https://docs.sonarsource.com/sonarqube-community-build/devops-platform-integration/azure-devops-integration/adding-analysis-to-pipeline/quality-gate-status-in-release-pipeline) | en | quality gate as pre-deployment gate |
| P23 | [ReportPortal — GitLab CI quality gates](https://github.com/reportportal/docs/blob/develop/docs/quality-gates/IntegrationWithCICD/IntegrationWithGitLabCI.md) | en | separate quality-gate stage with `needs: ['test']` |

*(23 primary documents; requirement was 12+.)*

**Secondary / illustrative (never the sole basis for a factual claim):** Microsoft RustTraining engineering book ch. 11 `[S][en]`; prisma/streams `RUST-QUALITY.md` `[S][en]`; darkmux `release-verify.yml` `[S][en]`; proxxx pre-commit gate `[S][pt]`; ready-to-release quality gates `[S][en]`; Libre DevOps CI/CD standards `[S][en]`; Atlassian MVP guides `[S][pt/de/es]`; InfoQ BR `[S][pt]`; productgurus `[S][pt]`; soulsy `[S][pt]`; Rückert `[S][pt]`; kjernePM `[S][de]`; Mike Bild `[S][de]`; stay-digital `[S][de]`; projektmagazin `[S][de]`; VFS Resources `[S][en]`.

---

## 6. Open questions

| # | Question | Blocks |
|---|---|---|
| Q1 | Should `cargo test` in `rust.yml` be widened to `--workspace --all-targets` (and a `--doc` leg added), or is the narrow root-package test intentional for the placeholder? | E2; a workflow edit — out of scope for this research pass |
| Q2 | Which exact per-test `... ok` lines / minimum counts should each DB leg assert, given the six filter strings currently in `rust.yml`? | E3; needs the test inventory |
| Q3 | Is the release-verify gate wanted as a required check at all, given GitHub does not count `workflow_dispatch` checks toward branch protection? | §1.4; governance decision |
| Q4 | Who is the real early user whose MCS result would close axis 2, and where is the criterion recorded? | "Seed/MVP reached" declaration |
| Q5 | Does the 4 GB NV close with the bounded restore drill from research 032/035, or is an off-box load generator still required by ADR-0001? | ADR-0001 remaining NV list |
