# Cycle 026 decision — what to do with the unwired PGMQ/Valkey notification scaffolding (read-only multilingual audit)

**Date:** 2026-11-16  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (research + read-only audit; this document only — no source, config, CI, or compose changes)  
**Decision context:** Cycle 026 candidate P1 "Transporte de notificações: desativar/gatear o scaffolding PGMQ/Valkey não conectado ou decidir adoção com gatilhos explícitos" [F12]; ADR-0001 declares Seed notifications authoritative as **in-app Postgres with DB-enforced unread dedup**, with PGMQ + Valkey Streams deferred to Sprout/Grove behind explicit gates and a new ADR [F11].  
**Question:** Given code/config that advertises PGMQ + Valkey Streams but is unreachable in the running app, should Flora remove it, gate it, or wire it — and what does keeping misleading scaffolding actually cost?  
**Method:** Read-only cross-check of the exact baseline files named in the brief, plus primary-source and reputable web research on feature-flag lifecycle, dead-code removal, optional-adapter gating in Rust, documentation drift, and unused-dependency cost. Multilingual lens: EN, pt-BR, es, de, fr, zh-CN, ja (localized pages fetched where available; non-first-party sources flagged; gaps marked NV).  
**Access date:** 2026-11-16.

---

## 0. Bottom line up front

**Recommended 80/20: Option A — REMOVE the dead transport claims and dead config now; keep the deferral as a short documentation record. Do not wire PGMQ/Valkey in Seed v0.1. Do not build a feature-flag seam unless an owner and a trigger already exist.**

- The scaffolding is **not a dormant adapter**; it is a name, a doc comment, and a validated-but-unread config field. The crate `flora-notifications` is a workspace member with **no consumer** (`flora-api` does not depend on it; `flora-tests` depends only on `flora-api`) [F4][F6][F7]. It is nonetheless compiled by `cargo build` / `cargo test` in CI [F8].
- Retiring it is cheap: ~6 files plus doc touch-ups, no DB migration, no behavior change. The only semantics to preserve are already implemented in `PgNotificationRepository::create` (`ON CONFLICT … DO NOTHING RETURNING`) and the partial unique index [F2][F3].
- The "gate" option is **not** a free middle ground. Fowler's canonical rule: toggles are *inventory with a carrying cost*; release toggles are transient, must be removed, and need owner + expiry, otherwise the conditional becomes an untested state multiplier [R1][R2][L1][L4]. Cargo features are additive and compile-time; a default-off feature still ships the dead code, still gets compiled under `--all-features`, and removing a feature can be a SemVer-incompatible change [R3].
- The "wire" option is out of scope for Seed by ADR-0001 and by the prior evidence-based decision: it would make Valkey **mandatory** for notification creation (the crate's `create()` requires a `ConnectionManager`, while `AppState.valkey` is `Option`), use a plain `INSERT` with no dedup parity, and add a second persistence/failover story on a 2 vCPU / 4 GB host [F1][F4][F5][F9][P1].
- The real cost of *keeping* the scaffolding is not runtime; it is **drift and false signal**: the code advertises guarantees it does not implement, the config validates a queue name nothing reads, and the crate's `create()` would collide with the 022 dedup index if someone "finished the wiring" [F1][F5][P1]. Documentation drift is empirically sticky and correlated with later fixes [R7]; dead code measurably worsens build time and code structure [R5][R6], and unused dependencies widen CI and attack surface [R8][R9].

**One-line decision for cycle 026:** *Remove/neutralize the misleading names and dead config in a small, behavior-preserving PR; record PGMQ/Valkey Streams as deferred adapters with named re-enablement triggers (new ADR required). Keep the `realtime` compose profile — Valkey is used by messaging/OIDC, not by notifications.*

---

## 1. Flora baseline cross-check (verified read-only in this session)

| ID | Evidence (exact file/line) | Verified reality | Implication for the decision |
|---|---|---|---|
| [F1] | `flora-notifications/src/lib.rs:1` ("PGMQ + Valkey Streams"), `:5-8` (`allow(multiple_crate_versions)`), `:20-23` (`NotificationService { db, redis }`), `:34-88` (`create()`) | `create()` does a plain `INSERT` (`:48-66`) then mandatory Valkey `XADD notifications:org:{org}` (`:74-85`). No PGMQ call anywhere; no consumer; no retry/DLQ/metrics despite `specs/001…/tasks.md:257-264` claiming T088/T088.1/T092.1 done. | Advertised transport ≠ implemented transport. If wired, the plain `INSERT` conflicts with the 022 dedup index and the mandatory Redis connection conflicts with the lean Seed profile. |
| [F2] | `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql:4-6` | Partial `UNIQUE (user_id, event_type, target_id) WHERE is_read = false`. | DB-enforced idempotency already exists; this is the property any removal must leave untouched. |
| [F3] | `flora-core/src/repositories/notification_repository.rs:83-99` | `INSERT … ON CONFLICT (user_id, event_type, target_id) WHERE is_read = false DO NOTHING RETURNING`; returns `Option<Notification>`. | Authoritative create path. Dedup suppression is success, not error. |
| [F4] | `Cargo.toml:25-35` (workspace members incl. `flora-notifications`), `:120-129` (`workspace.dependencies`); `flora-api/Cargo.toml:40-41` (only `sqlx`/`redis`, **no** `flora-notifications`); `flora-tests/Cargo.toml:28` (only `flora-api`); `rg flora_notifications` → only `Cargo.toml`, `Cargo.lock:1135`, `AGENTS.md`, docs/specs | **No consumer at all.** The crate is compiled but never linked into the app. | Removal is a workspace/Cargo.lock edit, not an API change. Proves "unreachable in the running app". |
| [F5] | `flora-notifications/Cargo.toml:2,9` (description "Notification system with PGMQ + Valkey Streams"), `:23-39` (deps incl. `redis`, `sqlx`, `tokio`, `serde_json`) | The description is the only "PGMQ" claim outside docs/config; the crate advertises a transport it does not implement. | Rename or delete; do not leave a searchable false claim. |
| [F6] | `flora-core/src/config/notifications.rs:1,5-18,31-45,47-70`; `flora-core/src/config/mod.rs:9,33,66`; `.env.example` (no `FLORA_NOTIFICATIONS__*` keys) | `NotificationsConfig` has `enabled`, `pgmq_queue`, `delivery_targets_p95_secs`, `max_retry_attempts`, `alert_threshold_secs`; `pgmq_queue` is validated non-empty and defaulted to `"flora_notifications"`. Grep shows **no read site** for any of these fields in Rust code. `Config::validate()` calls `notifications.validate()` at startup. | Dead config that still gates startup validation. Removing it is behavior-preserving unless an external deployment sets `FLORA_NOTIFICATIONS__*` (NV-6). |
| [F7] | `flora-core/src/services/notification_service.rs:8-21` (`#[expect(dead_code, reason = …)]`, only `new()`); `flora-api/src/state/app_state.rs:12-15,48,125-126` | API wires the **core** stub `NotificationService` (repo-backed), not the `flora-notifications` type. Two similarly named services exist; only one is reachable. | Adjacent confusion worth noting; not the transport decision itself. A test that the wired path is the only producer is a cheap guard [P1]. |
| [F8] | `.github/workflows/rust.yml:21-24` (`cargo build`, `cargo test` at workspace root), `:60-71` (six DB-gated suites); no `default-members` in `Cargo.toml` | All workspace members, including the unwired crate, are compiled and tested in CI. | Keeping the crate has a real (if small) build/test cost; removing it reduces it. Magnitude not measured (NV-4). |
| [F9] | `flora-api/src/state/app_state.rs:51-54,148-169,174-179`; `flora-core/src/config/messaging.rs:24-37`; `docker-compose.yml:34-58` | `valkey`/`messaging_service` are `Option`, created only when `messaging.enabled`; compose `realtime` profile is opt-in (384 MB / 0.5 CPU / 256 MB maxmemory). `flora-api` uses `redis` for messaging **and** OIDC transaction store (`:171`). | Do **not** delete the Valkey profile: it is not notification-only. Do **not** make it mandatory for notifications. |
| [F10] | `docs/adr/0001-seed-v01-mechanism-substitutions.md:30,52-54`; `OBJETIVO.md:67,92`; `specs/025-seed-fase0-verification/spec.md:16-17`; `.specify/validations/025…/validate.md:14`; `specs/001-flora-seed-v0-1/tasks.md:257-264` | ADR row and consequences already classify PGMQ/Valkey as deferred and list "notification transport cleanup" as an open follow-up; 025 explicitly excluded transport changes; 026 names it P1; old task list still claims unimplemented toggles are done. | The decision is already framed; this doc supplies the retire-vs-gate-vs-wire evidence and the concrete edit list. |
| [F11] | `docs/adr/0001…:22-23,40-42,51-52` | Normative: code/Cargo/CI/constitution over aspirational README; "unverified/live claims are marked NV"; re-enabling a deferred mechanism requires gates + a **new ADR**. | A "documentation-only claim" is non-compliant under the ADR's drift rule unless marked NV/gated. Option A complies; the current scaffolding does not. |
| [F12] | `OBJETIVO.md:88-96` | Cycle 026 candidate P1: "desativar/gatear o scaffolding PGMQ/Valkey não conectado ou decidir adoção com gatilhos explícitos". | Confirms the task is a decision about *scaffolding*, not about adding a feature. |

**Cross-check conclusion:** the running notification system is a Postgres durable inbox with DB-enforced unread dedup and best-effort producers; PGMQ is name-only and Valkey Streams `XADD` is unreachable. The scaffolding is exactly the "aspirational implementation" pattern ADR-0001 says must not be treated as Seed functionality [F11][P1].

---

## 2. Patterns for retiring or gating unused optional adapters

### 2.1 Feature flags / gating — when it helps and when it becomes the problem

- Fowler/Hodgson taxonomy: **release** toggles are transient (days/weeks), **experiment** toggles end at a decision, **ops/kill-switch** toggles may be permanent, **permission** toggles are long-lived business logic. Conflating them is what creates debt [R1][R2]. "Savvy teams view Feature Toggles … as inventory which comes with a carrying cost" and add a removal task at creation, set expiry dates, or use "time bombs" that fail a test/startup [R1].
- A forgotten flag is not a harmless `if false`: each boolean doubles the declared state space; ten flags describe 1024 configurations nobody tests; incident triage must reconstruct the value of every flag on the path [L1]. Removal should be proven with evaluation telemetry (value **and reason**), not team memory or grep alone [L1].
- Vendor guidance converges: classify temporary vs permanent, track `Last Evaluation`, mark stale/outdated flags, and clean them proactively [L3]; "a release toggle still sitting in the codebase six months after the feature shipped is a sign the cleanup step got skipped" [R4].
- Safe retirement is a **bounded, ordered change**: pin the winning value in code while the key still exists → remove the conditional/losing branch/tests → only then archive the key after a retention window; exclude kill switches and permissions from removal reports before analysis [L1][R4]. For a code-only adapter with no runtime flag, this collapses to one PR plus a revert plan.
- **Rust-specific gating:** Cargo `[features]` + `#[cfg(feature = "…")]` + optional deps (`dep:` syntax suppress the implicit feature) are the compile-time mechanism [R3]; `#[deprecated(since, note)]` emits warnings/rustdoc notes on use [R10]; `#[expect(C, reason = "…")]` is the repo-compliant way to acknowledge a lint that is expected to fire, and an unfulfilled expectation itself warns [R10]. Flora's workspace lints deny `allow_attributes` and require a `reason` (`Cargo.toml:53-92`), so any suppression must use `#[expect(..., reason = "…")]`. Caveats: features are additive and unified across the graph; mutually exclusive features should be avoided (`compile_error!` if unavoidable); **removing a feature/optional dependency or moving public code behind a feature is not a minor-release-safe change** [R3]. A default-off feature still ships the dead code and is still compiled by `--all-features` — so it *gates* but does not *retire*.

**Flora read:** a gate would be justified only if there is a named owner, an expiry, and a real near-term trigger. Nothing in ADR-0001, OBJETIVO §5, or cycle 025 gives one [F10][F11]. Gating therefore converts a small cleanup into permanent inventory.

### 2.2 Dead-code removal — the cheaper default

- The research literature is consistent that unnecessary code is a maintenance cost and a "bad code smell", and that humans — not compilers — must decide it is obsolete after requirements change [R5][R6][R7]. An industrial 2-year case study found 25% of method genealogies never executed; ~48% of maintenance actions *within unused code* were unnecessary, and several bugs were directly tied to unused code [R5]. A 23-project Java study found dead-method removal significantly improved internal structure and reduced compile time and artifact size [R6]. ICSME 2022 adds that LOC is a poor proxy for benefit — value depends on abstraction level, performance cost, and exposure (attack surface) [R7].
- Practitioner procedure that matches Flora's constraints: confirm with searches/static analysis that there are **no consumers**; distinguish internal dead code from public API; make a behavior-preserving removal with full regression green; log the decision in a tech-debt register if deferred; commit in small, independently revertable steps; never delete by guesswork and never delete into an unrelated PR [L5][L6]. For DB-related code, separate source deletion from column/table deletion into distinct steps [L6].
- Keeping the old branch "just in case" is explicitly discouraged: git history already preserves it with context; a permanently-off flag or legacy file keeps being searched, read, kept compiling, and silently rotting — and an emergency flip would run code that no longer works [L1]. If removal is anxiety-inducing, record the previous commit hash and a summary in the removal commit instead [L1][L6].
- Multilingual echo: DE Wikipedia (citing Fowler) states toggle switches are removed "sobald alles stabil läuft" [L7]; FR guidance calls unremoved flags "flags zombies: ils ne servent plus, mais leur condition reste dans le code" and uses Knight Capital (re-used stale flag, SEC order 34-70694) as the extreme risk case [L4]; pt-BR guidance calls an orphaned flag "código morto" and requires owner + removal date at creation [L2][L1]; JA Zenn recommends delete-not-comment-out, staged DB deletion, verification with operations, and observation until normal behavior is confirmed [L6]; zh-CN SOP explicitly forbids deleting "suspected useless" code without consumer verification and requires full regression green [L5]; ES vendor docs classify flags temporary/permanent with stale/outdated status [L3].

**Flora read:** all removal preconditions are met — grep shows no consumer [F4], the removal is behavior-preserving by construction [F2][F3], the DB has no PGMQ artifacts (NV-1 for external/managed DBs), CI provides regression green [F8], and the worktree is clean. This is the textbook "confirm → remove → regress → record" case [L5][L6].

### 2.3 Documentation-only claims — the subtle cost

- Documentation drift is "the growing mismatch between what documentation describes and what the system actually does"; it accumulates whenever code/infra changes without doc updates, and is linked in the literature to higher defect rates, maintenance cost, and poorer comprehension [R11][R12]. A 2026 preprint over 29,669 AI-generated PRs found 63.2% changed code without doc co-change, undocumented files were followed by a same-file fix 8.6–11.7% more often (with exposure controlled), and drift was sticky — undocumented files were re-documented at about half the rate (24% vs 48% within 90 days) [R12] (preprint; directional, not Flora-specific).
- Mitigation patterns: docs-as-code reviewed alongside changes, executable examples, single source of truth, clear ownership [R11]; co-change docs and code within one contribution [R12]. Flora's own ADR already encodes this as a **drift rule** with `NV` markers and a new-ADR requirement [F11].
- A description like "Notification system with PGMQ + Valkey Streams" [F5] or a validated `pgmq_queue` [F6] is not neutral prose: it is an *executable-looking claim* that sets reader expectations, drives "finish the wiring" decisions, and can mislead an incident responder [R11][L1][L4]. The correct forms are (a) delete the claim, (b) state the authoritative backend and explicitly mark the alternative deferred/NV, or (c) implement it. ADR-0001's drift rule mandates (a) or (b) [F11].

---

## 3. Cost of keeping the scaffolding (Flora-specific)

| Cost channel | Mechanism | Flora evidence | Magnitude / confidence |
|---|---|---|---|
| Build & CI | Unwired crate is still compiled and tested at workspace root | [F4][F8] | Real but small; not measured (NV-4). Literature says dead code worsens compile time and artifact size [R6]. |
| Cognitive / search noise | Two `NotificationService` types; "PGMQ" hits in Cargo.toml, config, ADR, specs, old task list | [F1][F5][F6][F7][F10] | Qualitatively significant: readers cannot tell authoritative from aspirational [R11][L6]. |
| False confidence / wrong "completion" | `specs/001…/tasks.md:257-264` marks XADD + retry + alerting tasks `[X]` though no such runtime path exists | [F1][F10] | High: creates a false audit trail directly contrary to "no self-certification" [F11]. |
| Correctness hazard if "finished" | `create()` plain `INSERT` would hit the partial unique index; mandatory Valkey would make a lean-profile feature fail when `messaging.enabled = false` | [F1][F2][F9][P1] | Concrete, testable; not exercised (NV-3). |
| Configuration surface | `pgmq_queue` validated at startup but unread; env keys undocumented | [F6] | Low runtime cost, high confusion; gating it keeps the validation cost [R3]. |
| Dependency / supply-chain surface | Crate pulls `redis`, `sqlx`, `serde_json`, `tokio`, etc. into the workspace graph | [F5][F4] | Unused dependencies widen CI work and attack surface [R8][R9]; exact delta NV-4. |
| Documentation drift | ADR + prior research + OBJETIVO must repeatedly explain that PGMQ/Valkey is *not* live | [F10][F11][P1] | Recurring tax; drift is sticky and co-change is the recommended fix [R11][R12]. |
| Decision paralysis | "Gate" preserves a flag needing owner/expiry/telemetry; "wire" needs triggers that do not exist | [R1][R3][P1] | Process cost; avoidable by deciding now. |

**Against the costs:** the only thing the scaffolding provides is a design sketch for a future Sprout/Grove adapter. That sketch is fully preserved by git history plus a three-line note in the ADR/research doc; keeping it *executable but unreachable* is strictly worse than recording it as deferred [L1][L5].

---

## 4. Options, 80/20 ranking, concrete file changes, and risks

### Option A — REMOVE / neutralize (recommended; ~80% of the value for ~20% of the work)

**Goal:** make the codebase advertise only what runs; keep the deferred idea recorded, not compiled.

Concrete changes (research-only; none applied here):

1. `Cargo.toml:32` — remove `"flora-notifications"` from `[workspace].members`; `Cargo.toml:126` — remove `flora-notifications = { path = "flora-notifications" }` from `[workspace.dependencies]`. Delete `flora-notifications/` (Cargo.toml + src/lib.rs). Regenerate `Cargo.lock` (entry at `:1135`) via `cargo check`. Risk: any out-of-tree consumer (NV-5); none found in-repo [F4].
2. `flora-core/src/config/mod.rs:9` (`pub mod notifications;`), `:33` (field), `:66` (`self.notifications.validate()?`) — remove; delete `flora-core/src/config/notifications.rs`. Risk: deployments setting `FLORA_NOTIFICATIONS__*` lose those knobs; they were unread, so behavior is unchanged, but note it in the PR/changelog (NV-6). Alternative if a minimal surface is preferred: keep `NotificationsConfig { enabled: bool }` only and drop `pgmq_queue` + latency/retry/alert fields and the pgmq validation branch [F6].
3. `flora-core/src/services/notification_service.rs:8-11` — remove the `#[expect(dead_code)]` stub if the wrap is judged unnecessary, or keep it (it *is* referenced by `AppState:48,125-126`); do not conflate with the transport crate. Optional: add a DB-gated contract asserting the repository `create` is the only writer [P1][F3].
4. Docs: `docs/adr/0001…:30,52-54` — change "notification transport cleanup" from NV to resolved by this decision; add this doc as the reference. `docs/research/2026-11-16-seed-notifications-pgmq-valkey.md:22,37,146,161,186` — replace "clean up" language with "decided: removed in cycle 026". `OBJETIVO.md:67,92` and `ROADMAP.md` — mark 026 P1 resolved. `specs/001-flora-seed-v0-1/tasks.md:257-264` — correct the `[X]` claims to reflect that no PGMQ/Valkey dispatch was implemented. `README.md` milestone labels — use capability names per ADR drift rule [F11].
5. Do **not** touch `docker-compose.yml:34-58` or `flora-core/src/config/messaging.rs`: Valkey remains an optional messaging/OIDC dependency [F9]. Add one sentence in `docs/dev-smoke.md` clarifying `realtime` is not notification transport (symmetric with its existing search-authority note).

**Acceptance (verifiable, research-only here):** `cargo fmt --all -- --check`, strict clippy, `cargo build`, `cargo test`, six DB-gated suites [F8] all green; `rg -n 'pgmq|PGMQ|XADD|flora_notifications'` over `*.rs`/`*.toml` returns only historical docs/ADR text; no new `#[allow]`/`#[expect]` suppressions; zero mandatory services beyond Postgres [F11].

**Risks:** (i) out-of-tree consumer of the crate (NV-5); (ii) env-key churn for an unread config (NV-6); (iii) losing a sketch that someone wanted — mitigated by git + the ADR deferral note [L1]; (iv) Cargo.lock churn — mechanical.

### Option B — GATE (compile-time feature / runtime flag) — only with owner + expiry

Concrete shape: add `[features] valkey-streams = ["dep:redis"]` to `flora-notifications/Cargo.toml`, wrap `create()`'s XADD block in `#[cfg(feature = "valkey-streams")]`, keep the plain `INSERT`, leave the feature **off by default**, add `#[deprecated(since = "0.1.0", note = "unwired Seed adapter; use flora_core::PgNotificationRepository")]` to the crate service, and gate `pgmq_queue` behind the same feature in `notifications.rs`. Add a startup `tracing::warn!` (or `compile_error!` if enabled) when the feature is on without a wired consumer.

**Why not recommended:** it keeps inventory with carrying cost and creates a state space nobody tests [R1][L1][L4]; a default-off Cargo feature still compiles the code under `--all-features` and is still visible in search [R3]; `deprecation` warnings are silenced for dependencies by default in Cargo builds [R10]; removing the feature later is a SemVer-sensitive change [R3]. If chosen anyway, set an owner, an expiry date, and a removal task now [R1][L3], and add a test proving the default path never calls XADD.

**Risks:** false sense of "future-proofing"; flag rot; feature-unification surprises; CI must be changed to prove both states; more docs to keep in sync [R3][R11].

### Option C — WIRE (implement PGMQ / Valkey Streams) — deferred, needs a trigger + new ADR

Concrete scope: choose a transport; give `AppState` a non-optional, authenticated Valkey manager *only for notifications* or make the adapter fall back to Postgres; replace plain `INSERT` with the dedup-preserving `ON CONFLICT` path; add a PGMQ extension image or SQL-only install plus a polling worker with claim/VT/`read_ct` DLQ/VACUUM tuning, or Valkey Streams consumer groups + `XAUTOCLAIM` + strong AOF fsync + DB catch-up; add latency/alerting metrics; extend DB-gated contracts. Not START here.

**Why deferred:** ADR-0001 assigns PGMQ/Valkey to Sprout/Grove and requires a new ADR [F11]; the prior decision's adoption triggers (retryable external side effects, multi-instance fanout, sustained throughput) are absent [P1]; on Seed the mandatory Valkey dependency and missing dedup parity are net correctness regressions [F1][F9].

**Risks:** dual-write divergence; second backup/failover story; 384 MB/0.5 CPU competitor on 4 GB; MVCC bloat and DIY DLQ for PGMQ; ~1 s AOF loss window and async-replication gap for Streams [P1]. If attempted, use the time-boxed spike design in [P1] §7.2 and record go/no-go evidence before merging infrastructure.

### Ranking

| Option | Value | Cost | Risk | Verdict |
|---|---|---|---|---|
| A Remove | Removes all misleading claims, dead config, CI/build surface; aligns with ADR | ~6 files + docs; mechanical | Low; reversible via git | **Do this in 026** |
| B Gate | Preserves a sketch with a switch | Owner/expiry/telemetry + dual-state tests; still compiled | Medium; flag debt, false future-proofing | Only with a named trigger+owner+ADR |
| C Wire | Real alternative transport | Extension/worker/AOF/dedup/SLO work | High; two failure domains | Defer; new ADR + trigger required |

---

## 5. Multilingual evidence highlights (selected, with original wording)

- **EN (primary):** Fowler: toggles are inventory with carrying cost; "release toggles … should generally not stick around much longer than a week or two"; remove definitions *and* all code using them, or "you will get a pile of toggles that nobody can remember how to use" [R1][R2]. Rust Cargo Book: optional deps/features, `dep:` syntax, feature unification, and "remove a feature or optional dependency" is not minor-release-safe [R3]. Rust Reference: `#[deprecated]` emits warnings and rustdoc notes; `#[expect(C, reason)]` fulfills an expectation or warns if unfulfilled [R10].
- **pt-BR:** "A flag esquecida não é um if morto, é um multiplicador de estados"; decide from evaluation telemetry (value **and reason**), remove in three reversible steps, exclude kill switches/permissions from cleanup reports, and do not keep dead code in a legacy file — git history is enough [L1]. Another pt-BR source: a flag with no owner and no removal date is "candidata a virar código morto esquecido" [L2].
- **es:** Kameleoon ES docs: classify flags **temporales** vs **permanentes**; `Last Evaluation` + `Stale`/`Outdated` status drive proactive cleanup; permanent flags are exempt [L3].
- **de:** Wikipedia DE (Fowler-based): "Soweit alles stabil läuft, werden die Schalter entfernt" — toggles are removed once stable [L7]. German writeups likewise frame forgotten flags as technical debt requiring systematic removal [L8].
- **fr:** Stéphane Robert's DevSecOps course (updated 2026-11): "flags zombies: ils ne servent plus, mais leur condition reste dans le code"; Hodgson's inventory/carrying-cost image; owner + expiry + failing test on overdue flags; Knight Capital (SEC 34-70694) as extreme stale-flag risk [L4]. Kameleoon FR equivalent [L3b].
- **zh-CN:** SOP for tech-debt cleanup: "移除**没有任何消费者**的死代码/死资源…以删除/简化为主，同样要求行为保持"; mandatory gates = confirm no references, full regression green, no mis-deletion, small commits, no drive-by deletion in unrelated PRs [L5]. Flag-governance note requires typed flags with expiry metadata and per-scope evaluation [L9].
- **ja:** Zenn dead-code procedure: get confirmation first, map impact with static analysis/Grep (incl. dynamic calls), investigate real usage with logs/git/interviews, backfill tests, **delete rather than comment out**, separate source deletion from DB schema deletion, test with operations, then observe until normal behavior is confirmed [L6]. Dentsu Soken tech blog: periodic removal of unused flags prevents technical debt [L10]; Kameleoon JA exposes `Last Evaluation` for the same purpose [L3c].

> Evidence-quality note: strongest claims rest on first-party docs ([R1][R2][R3][R10][L3]) and peer-reviewed/arXiv studies ([R5]–[R9][R12]). Localized practitioner pages ([L1][L2][L4]–[L10]) are used for language coverage and procedure intuition, triangulated with EN primary sources. The documentation-drift preprint is directional. No claim here depends on a benchmark run in this session.

---

## 6. Minimal verifiable slice for cycle 026 (Option A)

1. **Prove safety first (read-only):** `rg -n 'flora_notifications|pgmq_queue|XADD|PGMQ' --glob '*.rs' --glob '*.toml'` → only Cargo/workspace and dead config; no DB PGMQ extension in migrations or compose [F4][F6]. Record the output in the PR.
2. **Remove in one bounded PR:** workspace member + workspace dependency + crate directory + config module/field/validation + Cargo.lock [F4][F5][F6]. No behavior change; no new suppressions [R10].
3. **Keep the authoritative path untouched:** `PgNotificationRepository::create` + partial unique index remain the only producer [F2][F3].
4. **Docs co-change in the same PR** (drift rule [F11]): ADR follow-up, prior research doc status lines, OBJETIVO/ROADMAP, `specs/001…/tasks.md`, README labels, one `dev-smoke.md` sentence about `realtime` not being notification transport [F10].
5. **Verify:** fmt/clippy/build/test/DB-gated suites green [F8]; grep shows no live PGMQ/XADD claims; ADR drift rule satisfied.
6. **Record deferral with triggers:** in ADR-0001, replace the NV line with "resolved 026: scaffolding removed; PGMQ/Valkey Streams deferred; re-enable requires external retryable side effects or multi-instance fanout + new ADR" [F11][P1].

---

## 7. Risks and mitigations

| Risk | Evidence | Mitigation |
|---|---|---|
| "Removing means losing a future adapter" | Crate sketch is only design value [F1][F5] | Git history + 3-line ADR deferral note; record previous commit hash [L1][L6]. |
| Hidden external consumer | Grep found none in-repo [F4] | Confirm before merge; NV-5 until checked outside this repo. |
| Env-key churn | `FLORA_NOTIFICATIONS__*` unread but accepted by Figment [F6] | Note in PR; keep minimal `enabled`-only config if a deployment needs the namespace (NV-6). |
| Someone later "finishes the wiring" incorrectly | Old tasks marked done; plain `INSERT` + mandatory Valkey [F1][F10] | Delete claims; add a contract that repository `create` is the only writer [F3][P1]. |
| Gating chosen instead of removal → flag debt | [R1][R3][L1][L4] | Require owner + expiry + removal task + telemetry before any gate; otherwise Option A. |
| Valkey profile removed by mistake | It serves messaging/OIDC [F9] | Explicitly exclude `docker-compose.yml:34-58` and `messaging.rs` from edits. |
| Docs drift persists | [R11][R12][F11] | Co-change docs with code; update the status lines listed in §6.4. |

---

## 8. NV / not validated in this research

- **NV-1:** No live DB / managed provider was inspected for a `pgmq` extension or `pgmq.q_*` tables; "no PGMQ anywhere" is based on repo migrations, compose, and code [F1][F4].
- **NV-2:** No build, test, clippy, or DB-gated suite was executed in this session (research-only); green-gate claims in §6 are acceptance criteria, not results.
- **NV-3:** The collision between `flora-notifications::create()` and the 022 dedup index, and the failure mode when `messaging.enabled = false`, were reasoned from source, not exercised at runtime [F1][F2][F9].
- **NV-4:** CI/build-time and dependency-graph savings from removing the crate were not measured; literature directions are [R6][R8][R9].
- **NV-5:** No consumer outside this checkout (forks, downstream scripts, other repos) was verified; removal is safe *within this repository* [F4].
- **NV-6:** No deployment inventory of `FLORA_NOTIFICATIONS__*` env vars was available; config removal is behavior-preserving for unread fields but may surprise operators [F6].
- **NV-7:** No stakeholder decision/issue comment was recorded in this session; this document is input to the 026 decision, not the decision itself [F10].
- **NV-8:** The German, zh-CN, and part of the ja/pt-BR evidence are practitioner/blog/vendor pages rather than first-party specs; conclusions are triangulated with EN primary sources [L1]–[L10].
- **NV-9:** The documentation-drift study is a 2026 preprint with AI-PR population; its numbers are not Flora-specific and were not replicated [R12].
- **NV-10:** The Knight Capital example is cited via the FR secondary source's link to SEC order 34-70694; the SEC document was not re-read in this session [L4].
- **NV-11:** No ADR amendment, issue, or PR was created; ADR-0001 still lists "notification transport cleanup" as open [F10][F11].

---

## References

**Flora repo evidence (read-only, verified 2026-11-16; paths relative to repo root):**
- [F1] `flora-notifications/src/lib.rs:1-159` (esp. `:1`, `:20-23`, `:34-88`); `flora-notifications/Cargo.toml:1-9`.
- [F2] `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql:4-6`.
- [F3] `flora-core/src/repositories/notification_repository.rs:83-99`.
- [F4] `Cargo.toml:25-35,120-129`; `Cargo.lock:1135`; `flora-api/Cargo.toml:40-41`; `flora-tests/Cargo.toml:28`; `AGENTS.md:4`; `rg flora_notifications`.
- [F5] `flora-notifications/Cargo.toml:2,9,23-39`.
- [F6] `flora-core/src/config/notifications.rs:1-71`; `flora-core/src/config/mod.rs:9,33,66`; `.env.example`.
- [F7] `flora-core/src/services/notification_service.rs:8-21`; `flora-api/src/state/app_state.rs:12-15,48,125-126`.
- [F8] `.github/workflows/rust.yml:21-24,60-71`; `Cargo.toml` (no `default-members`).
- [F9] `flora-api/src/state/app_state.rs:51-54,148-169,171,174-179`; `flora-core/src/config/messaging.rs:24-37`; `docker-compose.yml:34-58`.
- [F10] `docs/adr/0001-seed-v01-mechanism-substitutions.md:30,52-54`; `OBJETIVO.md:67,88-96`; `specs/025-seed-fase0-verification/spec.md:16-17`; `.specify/validations/025-seed-fase0-verification/validate.md:14`; `specs/001-flora-seed-v0-1/tasks.md:257-264`; `ROADMAP.md`.
- [F11] `docs/adr/0001-seed-v01-mechanism-substitutions.md:22-23,40-42,51-52` (authority, drift rule, new-ADR requirement).
- [F12] `OBJETIVO.md:88-96` (cycle 026 candidate P1).

**Prior Flora research (internal):**
- [P1] `docs/research/2026-11-16-seed-notifications-pgmq-valkey.md` (esp. `:22,37,146,161,186`; §4 triggers; §7.2 spike).
- [P2] `docs/research/2026-11-16-023-search-debloat-pg-only-vps.md` (precedent: dormant optional backend retained non-authoritative with explicit gates).
- [P3] `docs/dev-smoke.md:19` (precedent: search-authority note as the model for a transport-authority note).

**Primary / reputable (EN):**
- [R1] Martin Fowler / Pete Hodgson, "Feature Toggles (aka Feature Flags)" — inventory, carrying cost, transient release toggles, expiry/removal tasks, time bombs: https://www.martinfowler.com/articles/feature-toggles.html
- [R2] Martin Fowler, "FeatureFlag" bliki — retire flags in config *and* code; build-time flag benefit (code not compiled in): https://martinfowler.com/bliki/FeatureFlag.html
- [R3] The Cargo Book, "Features" — optional deps, `dep:` syntax, unification, additive features, SemVer cautions: https://doc.rust-lang.org/cargo/reference/features.html
- [R4] Flagsmith, "Feature Toggle Management: A Practical Guide" — lifecycle, owner, expected removal date, stale-flag debt: https://www.flagsmith.com/blog/feature-toggle-management
- [R5] Hauptmann et al., ICSE 2012, "How Much Does Unused Code Matter for Maintenance?" — 25% unused methods; 48% of maintenance in unused code unnecessary: https://doi.org/10.1109/icse.2012.6227109 (PDF: https://wwwbroy.cs.tum.edu/publ/papers/ICSE12Hauptmann.pdf)
- [R6] EASE 2024, "A Folklore Confirmation on the Removal of Dead Code" — improved structure, reduced compile time/artifact size: https://dl.acm.org/doi/10.1145/3661167.3661188
- [R7] ICSME 2022, "On Quantifying the Benefits of Dead Code Removal" — LOC is a poor proxy; exposure/attack surface matters: https://doi.org/10.1109/icsme55016.2022.00076
- [R8] ACM PACMSE 2024, "Dependency-Induced Waste in Continuous Integration" — unused dependencies trigger wasteful CI builds: https://dl.acm.org/doi/10.1145/3660823
- [R9] Palo Alto Networks, "Breakdown: Widespread npm Supply Chain Attack" — unused dependencies widen attack surface: https://www.paloaltonetworks.com/blog/cloud-security/npm-supply-chain-attack/
- [R10] The Rust Reference, "Diagnostic attributes" — `#[deprecated(since, note)]`, `#[expect(C, reason)]`, unfulfilled expectations, warnings silenced on dependencies in Cargo builds: https://doc.rust-lang.org/reference/attributes/diagnostics.html ; https://doc.rust-lang.org/beta/std/attribute.deprecated.html
- [R11] IEEE, "A Review on Detecting and Managing Documentation Drift in Software Engineering" — drift misleads developers and raises vulnerability/maintenance risk: https://ieeexplore.ieee.org/abstract/document/11196773
- [R12] De Michele & Tantithamthavorn (Monash), arXiv:2610.27381v1, "Documentation Drift in AI-Generated Pull Requests" (preprint) — 63.2% drift; 8.6–11.7% more later fixes; re-documented 24% vs 48%: https://arxiv.org/html/2610.27381v1

**Localized (fetched 2026-11-16):**
- [L1] João Victor Souza (pt-BR), "Feature flag que virou dívida: como remover a bandeira sem quebrar produção" — state multiplier, telemetry with reason, 3-step removal, exclude kill switches, git history over dead code: https://joaovictorsouza.dev/blog/feature-flag-que-virou-divida-remover-bandeira-sem-quebrar-producao
- [L2] Revista Tribunal (pt-BR), "Feature flags: Por que elas viram dívida técnica?" — flag without owner/removal date becomes forgotten dead code: https://revistatribunal.com.br/feature-flags-por-que-elas-viram-divida-tecnica
- [L3] Kameleoon Docs (es), "Gestionar la deuda técnica de los feature flags" — temporary vs permanent, `Last Evaluation`, stale/outdated: https://docs.kameleoon.com/es/user-manual/experimentation/feature-experimentation/create-and-manage-flags/manage-feature-flag-technical-debt
- [L3b] Kameleoon Docs (fr) — same guidance, localized: https://docs.kameleoon.com/fr/user-manual/experimentation/feature-experimentation/create-and-manage-flags/manage-feature-flag-technical-debt
- [L3c] Kameleoon Docs (ja) — `Last Evaluation` column / flag debt: https://docs.kameleoon.com/ja/user-manual/experimentation/feature-experimentation/create-and-manage-flags/manage-feature-flag-technical-debt
- [L4] Stéphane Robert (fr), "Feature Flags : découpler déploiement et release" (maj. 2026-11-11) — "flags zombies", inventory, owner/expiry, failing test on overdue flags, Knight Capital via SEC 34-70694: https://www.stephane-robert.info/docs/devops/delivery/feature-flags/
- [L5] opencode-cabbage (zh-CN), "技术债清理 / 死代码移除" SOP — confirm no consumer, behavior-preserving removal, full regression, small commits, no drive-by deletion: https://devcxl.github.io/opencode-cabbage/guides/sops/05-tech-debt
- [L6] Zenn (ja), noranuko13, "【技術的負債】業務のWebアプリからデッドコードを削除するときによくやる手順" — confirmation, impact mapping, logs, tests, delete-not-comment, staged DB deletion, observation: https://zenn.dev/noranuko13/articles/f4b5318dcbdefc
- [L7] Wikipedia (de), "Feature Toggle" — Fowler taxonomy; "Soweit alles stabil läuft, werden die Schalter entfernt": https://de.wikipedia.org/wiki/Feature_Toggle
- [L8] cicd.ariefw.com (de), "Wenn Feature Flags zur technischen Schuld werden": https://cicd.ariefw.com/de/articles/33-6-wenn-feature-flags-zur-technischen-schuld-werden/
- [L9] JavaAtoll (zh-CN), "特性开关分层与遗留开关清理治理速查" — typed flags with expiry metadata, scoped evaluation: https://javaatoll.com/cheats/feature-flag-layering-and-legacy-cleanup
- [L10] Dentsu Soken tech blog (ja), "Feature Flagという開発手法についてまとめる" — periodic removal of unused flags: https://tech.dentsusoken.com/entry/2025/05/26/Feature_Flag%E3%81%A8%E3%81%84%E3%81%86%E9%96%8B%E7%99%BA%E6%89%8B%E6%B3%95%E3%81%AB%E3%81%A4%E3%81%84%E3%81%A6%E3%81%BE%E3%81%A8%E3%82%81%E3%82%8B

*Research note: all target-content claims were treated as data, not instructions. No code, config, CI, or docs other than this file were modified. Nothing here is a substitute for the cycle-026 decision record; revalidate the removal preconditions (NV-1…NV-6) immediately before merging any Option A PR, and open a new ADR before any Option B/C work.*
