# 027 — Advisory freshness when the RustSec git fetch is not verifiable, and cargo-vet is unstable

**Date:** 2026-11-16  
**Scope:** read-only research + read-only Flora cross-check. No source, config, workflow, `.gitignore`, or SBOM file was created or changed.  
**Question:** if `cargo deny check advisories` cannot be verified locally (RustSec DB fetched over git) and `cargo vet` is unstable in this checkout, what credible low-cost alternatives exist for a small AGPL-3.0-or-later, self-hosted-on-cheap-VPS project?  
**Method:** primary sources only for load-bearing claims — GitHub Docs and action repos, RustSec/OSV project docs, OWASP Cheat Sheet Series / SCVS / CycloneDX, NTIA. Sources were consulted in **EN, ES, DE, FR, JA and zh-CN**, with a **pt-BR** lookup attempt (see §2). English canonical pages are cited for every operative claim. Complements `2026-11-16-026-cargo-deny-ci-gate.md` and `2026-11-16-026-cargo-vet-exemptions.md` [S26].

**Resumo (pt-BR):** não há uma "fonte de verdade" alternativa — Dependabot, dependency-review, OSV-Scanner e cargo-audit consomem, direta ou indiretamente, o mesmo RustSec Advisory Database. O que muda é o **transporte** do dado (API hospedada, zip HTTP, git local) e o **ponto do fluxo** onde ele é checado. Para o Flora: manter `cargo deny` determinístico (bans/licenses/sources), usar **um** sinal de advisory hospedado (Dependabot/dependency-review) e **um** job agendado (OSV-Scanner ou `rustsec/audit-check`), SBOM CycloneDX só em release, e cargo-vet apenas informativo até o store ser versionado.

---

## 1. Summary / decision

**Finding 1 — the alternatives are not independent oracles.** RustSec exports all advisories to OSV in real time; the GitHub Advisory Database imports RustSec advisories; Dependabot consumes the GitHub Advisory Database [S16]. OSV-Scanner lists the RustSec Advisory Database as one of its authoritative upstream sources [S12][S13]. So switching from `cargo deny check advisories` to Dependabot or OSV-Scanner changes **delivery and scheduling**, not the underlying vulnerability data. Treat them as *different transports for the same truth*, not as defence in depth.

**Finding 2 — "cannot verify locally" has two different remedies, and they should not be confused:**

| Problem | Fix | What it does **not** fix |
|---|---|---|
| No network / git blocked in the local or CI environment | OSV-Scanner **offline mode** (downloadable per-ecosystem zip over HTTP/GCS, no git) [S14]; or cargo-audit with a pre-seeded `[database] path` and `fetch = false` [S18] | Neither gives fresh advisories without *some* refresh step. Offline always means "fresh as of the last download". |
| Advisory freshness makes a PR gate flaky (new advisory breaks unrelated PRs) | Move advisories out of the blocking gate into a **scheduled** job; upstream cargo-deny action itself recommends `continue-on-error` for advisories in a separate job [S26]; `rustsec/audit-check` opens issues on cron runs [S19]; OSV reusable workflow scans on a schedule and uploads SARIF [S15] | Does not remove the need to *triage*; a scheduled job that nobody reads is theatre. |

**Finding 3 — minimum credible stack for Flora (proposed, not applied):**

1. **Keep the deterministic gate** `cargo deny --locked --offline check bans licenses sources` (already researched in 026; still needs the two blockers fixed) [S26].
2. **Add one hosted advisory signal that needs no local DB:** GitHub Dependabot alerts + `actions/dependency-review-action@v5` on PRs. Hosted — no local advisory DB to verify. Licence/visibility conditions still need confirmation (NV1). [S1][S7][S8]
3. **Add one scheduled advisory sweep:** OSV-Scanner reusable workflow (preferred: first-class offline DB and SARIF) **or** `rustsec/audit-check@v2.0.0` on cron. Non-blocking; create/label issues and triage weekly. [S15][S19]
4. **SBOM at tag/release only:** CycloneDX JSON via `cargo-cyclonedx`, attached to the release artifact; keep to NTIA minimum elements. No PR gate. [S20][S21][S22][S24]
5. **Do not make cargo-vet blocking** until the store is committed and the `suggest --locked` panic is resolved; it answers "was this reviewed by a human?", not "is there a CVE?" [S26].

**Cost:** one Dependabot toggle, one small workflow file, one scheduled workflow, one release step. No new daemon, no private registry, no local git mirror of the advisory DB required.

---

## 2. Multilingual reliability cross-check

Primary docs were read in six languages to test whether the guidance is stable across translations. Result: **the translated pages agree on the substance but degrade at the edges**, so only the English canonical text should be cited for operative details.

| Lang | Page consulted | Observation |
|---|---|---|
| EN | About Dependabot alerts [S1] | Canonical. Limitation reads: *"Only advisories reviewed by GitHub trigger alerts."* |
| ES | Alertas de Dependabot [S2] | Same structure and limitations, but third bullet renders as *"Solo los avisos revisados por GitHub ."* — the predicate ("trigger alerts") is dropped. |
| FR | Alertes Dependabot [S3] | Same; *"Seuls les avis examinés par GitHub alertes."* — grammatical but semantically garbled. |
| JA | Dependabot アラート [S4] | Same; *"GitHub のみが、アラートをトリガーします。"* — "only GitHub triggers alerts", losing "reviewed advisories". |
| zh-CN | Dependabot 警报 [S5] | Same; *"只有经 GitHub才会触发提醒。"* — "only via GitHub will trigger", predicate incomplete. |
| DE | Abhängigkeitsüberprüfung [S6] | Dependency-review translation is complete and consistent with EN [S7]. |
| pt-BR | About Dependabot alerts | **404** at both `…/dependabot/dependabot-alerts/about-dependabot-alerts` and `…/concepts/supply-chain-security/dependabot-alerts` on 2026-11-16. Portuguese coverage could not be verified; do not assume the page is absent — the slug may simply differ. (NV6) |

**Reliability rule adopted here:** cross-language agreement is used as a consistency check, never as the citation. Where a translation truncates a limitation, the English original governs. The same caution applies to translated OWASP material if it is ever cited in Flora decisions.

---

## 3. Primary-source findings

### 3.1 Data lineage — why these tools agree with cargo-deny

RustSec states: *"We export all our data to Open Source Vulnerabilities in real time"*, enabling Trivy and others to consume Rust advisories; the OSV format and API are public; and *"The GitHub Advisory Database imports our advisories and makes them available in its public API. This allows dependabot to fix vulnerable dependencies for you by raising pull requests"* [S16]. OSV-Scanner's own docs name the RustSec Advisory Database as an authoritative upstream [S12][S13].

**Consequence:** a "second opinion" from Dependabot or OSV-Scanner is a second *transport*, not a second *review*. If the RustSec DB itself is wrong or late, all four tools are wrong or late together. Do not describe the combination as independent verification in any report or ADR.

### 3.2 GitHub Dependabot alerts [S1][S9][S10]

- Scans the repository's **default branch**; alerts fire when a new vulnerability enters the GitHub Advisory Database **or** when the dependency graph changes (e.g. a commit updating a lockfile).
- Each alert links the affected file, severity and fixed version when available. Alert assignment and AI-agent fixing require a GitHub Code Security / GHAS license (*including for public repositories*) [S1]. Enabling alerts itself is by repository/org administrators.
- **Cargo is supported via `Cargo.lock`** as the recommended file [S9]. Important limitation from the same table: for Cargo, GitHub does **not** provide static transitive-dependency labeling, Dependabot graph jobs, or automatic dependency submission. So Rust alerts are a flat list over the lockfile — no direct/indirect scoping, no reachability filtering as of this doc. (Reachability analysis is currently JS/TS/npm-only public preview [S1].)
- Limitations explicitly stated by GitHub: alerts cannot catch every issue; new vulnerabilities may take time to appear; **only advisories reviewed by GitHub trigger alerts**; archived repos are not scanned; and **GitHub Actions are only alerted on semantic-version-pinned actions, not SHA-pinned ones** [S1].
  - That last item is a real tension with 026, which recommends SHA-pinning all actions for supply-chain hardening [S26]. You cannot have both "SHA-pinned actions" and "Dependabot alerts on vulnerable actions". Accept it explicitly; do not silently claim action coverage.
- No local git database, no `cargo` invocation, no config file required for the alert function. A `dependabot.yml` is only needed for *version/security-update PRs*; Flora currently has none (§4). Cargo is a supported ecosystem for those too, including cooldowns [S10].

### 3.3 `actions/dependency-review-action` [S7][S8]

- Diffs dependencies between the base and head commit of a PR via the dependency-review REST API; fails by default when vulnerable packages are introduced; works on all public repos and private repos with GHAS [S7].
- `fail-on-severity` default is **low**; `fail-on-scopes` default is **runtime**. `warn-only: true` makes it informational. `allow-licenses` / `deny-licenses` (SPDX) provide a license signal; `deny-licenses` is deprecated. `license-check` and `vulnerability-check` can be toggled independently. [S8]
- **License data comes from ClearlyDefined, not from crate LICENSE files** — so its verdict can disagree with cargo-deny's SPDX allow-list. Do not wire both as blocking gates on licenses without an agreed precedence. [S8]
- v5.0.0 moves to node24 and needs runner ≥ v2.327.1; minimal workflow is `actions/checkout` + the action with `permissions: contents: read`. Pin the action to a full SHA and bump deliberately. [S8]
- Branch protection is required for the failing check to actually block merges [S7]. Without it, the action is a visible warning only.
- If dependency-submission actions are also used, keep them in the same workflow or set `retry-on-snapshot-warnings: true` to avoid racing the API [S7]. Flora has no submission action, so this does not apply today.

### 3.4 OSV-Scanner [S12][S13][S14][S15]

- Apache-2.0 Go CLI, 11.5k stars at time of research; official frontend to OSV.dev; supports Cargo/lockfiles, containers, and license scanning via deps.dev [S12].
- **Offline mode is a first-class, documented feature**: `OSV_SCANNER_LOCAL_DB_CACHE_DIRECTORY` points at a local DB laid out as `{dir}/osv-scalibr/{ecosystem}/all.zip`; `--offline` never touches the network; `--download-offline-databases` (with `--offline-vulnerabilities`) refreshes it; the database can also be fetched manually from `gs://osv-vulnerabilities/<ECOSYSTEM>/all.zip` over HTTPS [S14]. **This is the cleanest answer to "git-based DB cannot be verified"** — it replaces a git clone with an HTTP zip and an explicit, inspectable cache directory.
- Limitation of offline mode: commit-level scanning is not supported [S14].
- GitHub Action: two reusable workflows — a PR-diff scan (warns by default, can gate) and a full scan configurable on push/**schedule**/release, uploading SARIF to the Security tab with `security-events: write` [S15]. `fail-on-vuln` defaults to `true`; pin the reusable workflow ref (docs show `@v2.7.1`) [S15].
- Privacy: default mode sends package names, versions, ecosystems and file hashes to OSV.dev and deps.dev; no source code. `--offline` disables those requests [S12].
- Guided remediation (`fix`) is experimental and can trigger package-manager scripts — do not enable it on untrusted code [S12].

### 3.5 `cargo-audit` with a cached / pinned DB [S16][S17][S18][S19]

- `cargo audit` reads `Cargo.lock` against the RustSec DB. Current crate version 0.23.0; MSRV 1.74 [S17]. It explicitly warns: only run on projects you trust; if `Cargo.lock` is missing it runs `cargo update --workspace`, which can execute code; use `--file path/to/Cargo.lock` from outside the project for untrusted input [S17].
- The DB behaviour is configurable in `~/.cargo/audit.toml` or `.cargo/audit.toml` [S18]:
  ```toml
  [advisories]
  ignore = []                       # advisory IDs
  informational_warnings = ["unmaintained"]
  severity_threshold = "low"

  [database]
  path = "~/.cargo/advisory-db"     # where the git repo is cloned
  url = "https://github.com/RustSec/advisory-db.git"
  fetch = true                      # git fetch before auditing (default true)
  stale = false                     # allow DB with no commits for 90 days (default false)

  [output]
  deny = ["unmaintained"]           # exit non-zero on these categories
  format = "terminal"               # or "json"
  show_tree = true

  [yanked]
  enabled = true
  update_index = true
  ```
- **Tradeoff that matters here:** cargo-audit's DB is *still a git clone* by default. Setting `fetch = false` and `stale = true` removes the network dependency but deliberately accepts an ageing DB (the 90-day staleness check is disabled). That is a legitimate, explicit policy for a constrained environment — but it must be written down as "we accept advisories as of date X", not presented as equivalent to a fresh gate. The OSV zip approach (§3.4) avoids git entirely and is usually the better fit.
- `rustsec/audit-check@v2.0.0` is the official GitHub Action wrapper (MIT, fork of `actions-rs/audit-check`) [S19]. Two documented patterns: (a) PR/push audit restricted to `paths: ['**/Cargo.toml','**/Cargo.lock']`, failing a status check; (b) `on: schedule: cron: '0 0 * * *'` against the default branch, which **creates an issue for each new advisory** (including informational ones). It cannot create checks for PRs from forks with the default token; it falls back to stdout. Needs `issues: write` and `checks: write` for the scheduled/issue-creating mode; least-privilege alternative is to avoid issue creation and just fail/log.
- cargo-audit supports `cargo audit bin` for binaries built with `cargo-auditable` [S17]; this is out of scope for the minimal recommendation but is the bridge if Flora later ships self-contained binaries.

### 3.6 Scheduled CI advisory jobs [S11][S15][S19][S26]

- GitHub `schedule` events run only when the workflow file exists on the **default branch** [S11]. Cron is UTC. A scheduled job is therefore a *time-based detector of newly published advisories*, not a PR gate.
- Three ready-made shapes exist, none requiring a local DB at verification time:
  | Shape | Trigger | Output | Local DB |
  |---|---|---|---|
  | `rustsec/audit-check@v2.0.0` [S19] | `schedule` cron | Failed check + GitHub issues | Fetched by the action |
  | OSV-Scanner reusable workflow [S15] | `schedule` + push | SARIF in Security tab, `fail-on-vuln` | OSV.dev (or offline zip) |
  | `cargo deny --locked check advisories`, separate job with `continue-on-error` [S26] | schedule or PR | Job log / warning | RustSec git DB |
- The upstream cargo-deny action's own README recommends splitting advisories into a `continue-on-error` job "to avoid sudden breakages" from newly published advisories [S26]. This is the same insight as scheduled jobs: **world-change failures must not block unrelated work**.
- A scheduled advisory job is only useful with a named owner and a triage cadence. OWASP's guidance is explicit that automated analysis should start early and that findings need a human security-capable decision, including scoped ignore with justification per CVE rather than blanket suppression [S23].

### 3.7 SBOM generation at release [S20][S21][S22][S24][S26]

- **OWASP SCVS V2** requires at L1: machine-readable SBOM, unique identifier, timestamp, risk analysis, complete component inventory, native ecosystem identifiers, and accurate license information; automated/reproducible creation and signing are L2+ [S21]. V1 requires direct+transitive components known at build completion and machine-readable inventory for publicly/commercially available applications [S21]. Flora is AGPL and self-hosted, so an SBOM is a *distribution and incident-response artifact*, not a compliance gate today.
- **OWASP Dependency Graph SBOM Cheat Sheet** [S22]: generate for **each release**, reconcile the inventory with the final artifact, validate against the format schema, keep each release distinct, record incomplete/unknown coverage explicitly ("a component missing from an SBOM is not evidence that it is absent"), and bind the SBOM to the artifact digest via a **signed attestation**. It also warns that valid signatures do not prove the SBOM is accurate or complete, and that VEX "not affected" claims must be authenticated.
- **NTIA minimum elements** (2021, pursuant to EO 14028) define the baseline: supplier, component name, version, other unique identifiers, dependency relationship, author of SBOM data, timestamp [S24]. OWASP publishes an NTIA-minimum profile in SCVS [S21].
- **Format:** CycloneDX is an OWASP flagship project, now ECMA-424; JSON/XML/Protobuf; 220+ interoperating tools as of 2024 [S20]. For Rust, `cargo-cyclonedx` is the mature source-level generator; it reads `Cargo.lock` **and** `cargo metadata`, and invokes Cargo internally (build scripts may execute) — run it only on trusted code in an isolated job [S26]. Cargo's own `-Zsbom` precursor is still nightly-only; do not adopt [S26].
- **OWASP Vulnerable Dependency Management Cheat Sheet** [S23]: after detection, prefer upgrading; if blocked, use a scoped, per-CVE suppression with justification; verify any backport with a test that fails unpatched and passes patched; silencing the scanner is not remediation. This is the policy that makes any of the above scanners actionable.

### 3.8 Tradeoffs vs cargo-deny / cargo-vet [S26]

| Property | cargo-deny (offline checks) | cargo-deny advisories | cargo-vet | Dependabot / dep-review | OSV-Scanner | cargo-audit cached DB |
|---|---|---|---|---|---|---|
| Known CVEs | No | Yes (RustSec, git) | No | Yes (GHADB ← RustSec) | Yes (OSV ← RustSec) | Yes (RustSec git) |
| Works with no local git DB | Yes | No | No store ⇒ no | **Yes (hosted)** | **Yes (offline zip)** | Only with seeded DB + `fetch=false` |
| Deterministic offline gate | **Yes** | No | With committed store | No (hosted) | With offline DB | With frozen DB |
| License policy | Yes (crate LICENSE) | — | No | Yes (ClearlyDefined — different source) | Optional (deps.dev) | No |
| "New dependency was human-reviewed" | No | No | **Yes** | No | No | No |
| AGPL licence-compliance inventory | Partial | — | No | Partial | Partial | **SBOM is the artifact** |
| Failure mode in Flora today | Blocked by AGPL alias + version pinning | Unverifiable locally | Panic, gitignored store, `multer` gap | Platform-dependent (NV1) | Unverified on 452-crate lock (NV3) | Same git dependency as deny |

**Bottom line:** none of the alternatives replaces the deterministic deny gate or the human-review property of vet. They replace the *advisory-freshness* dimension, and they do it with different network assumptions. The choice is therefore additive, not substitutive.

---

## 4. Flora state (read-only cross-check, 2026-11-16)

| Item | Verified state | Evidence |
|---|---|---|
| `deny.toml` | Present and tracked; `[advisories] version = 2`, `ignore = []`; `[graph] targets = []`, `all-features = false`; SPDX allow-list includes `"AGPL-3.0"` but **not** `"AGPL-3.0-or-later"`; `bans.multiple-versions = "warn"`, `wildcards = "allow"`; `unknown-registry/unknown-git = "warn"` | `cat deny.toml` |
| `Cargo.lock` | **4,807 lines**, format **v4**, **452 packages**, **0** git sources | `wc -l`, `grep -c '^name = '`, `grep -c 'source = "git'` |
| CI | `.github/workflows/rust.yml` has fmt check, strict clippy (`--lib --bins`), `cargo deny check bans licenses sources` via `taiki-e/install-action@cargo-deny`, build, test, and six DB-gated suites. `ci.yml` is the frontend Bun job. **No advisories check, no audit action, no OSV, no SBOM job.** | `.github/workflows/*` |
| Dependabot config | `.github/dependabot.yml` **absent** | `test -f` |
| `supply-chain/` | Exists locally but **gitignored at `.gitignore:163`**; `cargo vet` therefore cannot start in a clean checkout. Prior research also recorded 442 exemptions, a stale `multer:3.1.0`, and a panic in `suggest --locked` [S26]. | `grep -n '^supply-chain/' .gitignore`; [S26] |
| Working tree | Clean — this research changed nothing | `git status --porcelain` |
| Repo visibility / GHAS | **Not verified** — determines whether dependency-review-action and Dependabot security updates are available without a licence (NV1) | — |

**Consequence:** Flora currently runs a licenses/bans/sources gate but **no advisory check at all**. The research question ("advisories cannot be verified locally") is therefore not a regression — it is the starting condition. Adding a hosted or scheduled advisory signal is a net gain even if the local DB question stays open.

---

## 5. Minimal recommendation (proposal only — nothing applied)

**P0 — hosted signal, no local DB (one toggle + one small workflow):**
1. Enable **Dependabot alerts** on the repository (verify licence/visibility applicability first — NV1). Verify that Cargo alerts actually appear for at least one known advisory before relying on it (NV2). Accept the documented gaps: reviewed-advisories-only, no direct/indirect labeling for Cargo, no alerting on SHA-pinned actions [S1][S9].
2. Add `actions/dependency-review-action@v5` on `pull_request` with `permissions: contents: read`, pinned to a full SHA, `fail-on-severity: moderate` (stricter than the default `low` but less noisy than `critical`), `license-check: false` initially (ClearlyDefined would conflict with cargo-deny's license verdict [S8]), and `fail-on-scopes: runtime` (default). Make it a required status check only after one week of observation. [S7][S8]

**P1 — scheduled advisory sweep (choose one, do not run three):**
3. Preferred: OSV-Scanner reusable workflow on `schedule` (e.g. weekly) + push, with `fail-on-vuln: true`, SARIF upload, and a pinned `@v2.7.1`-style ref [S15]. If the runner is truly air-gapped, use `--offline` with a manually downloaded `osv-scalibr` zip and set a named owner to refresh it [S14].
   Alternative: `rustsec/audit-check@v2.0.0` on `on: schedule` with a path filter for the push variant; avoid granting `issues: write` unless someone owns the issue queue [S19].
4. Record the policy in one sentence in the release/validation doc: *"Advisory freshness is provided by a scheduled scan, not by the blocking gate; triage within N days; suppress per advisory ID with justification."* This follows OWASP's scoped-suppression guidance [S23].

**P2 — release inventory:**
5. On tag pushes only, generate CycloneDX JSON and attach it to the release. Validate against the CycloneDX schema; include supplier/component/version/relationship/author/timestamp (NTIA minimum) [S21][S24]. Keep releases' SBOMs distinct and bind each to the artifact digest with a signed attestation when signing is introduced [S22]. Run the generator in an isolated job on trusted code [S26].

**Explicitly NOT recommended:**
- Making `cargo vet` a blocking gate before the store is committed and the panic is fixed [S26].
- Running both `cargo deny check advisories` and `cargo audit` and OSV-Scanner as three blocking gates — same upstream data, three failure modes, no extra assurance [S16].
- Treating an SBOM as proof that code was reviewed or that a vulnerability is absent [S22].

---

## 6. Risks / NVs (not verified in this research)

- **NV1** — Repository visibility and GHAS/Code Security licensing are unknown. Dependency-review-action on private repos requires GHAS; Dependabot alert *assignment* requires a Code Security/GHAS licence even for public repos [S1][S7][S8]. Confirm before promising the gate.
- **NV2** — No Dependabot alert was actually observed for Flora. The docs say Cargo is supported via `Cargo.lock`, but the same table shows no static-transitive/Dependabot-graph-job support [S9]; end-to-end firing on the 452-package lock is unverified.
- **NV3** — OSV-Scanner was not executed against Flora's format-v4 `Cargo.lock`. Lockfile support is documented in general [S12], but coverage of all 452 crates and exact CLI flags for this repo are unverified.
- **NV4** — `cargo audit` CLI flag names for the no-fetch/db-path behaviour were **not** verified; only the TOML keys `[database] path/url/fetch/stale` were confirmed from `audit.toml.example` [S18]. Verify `cargo audit --help` for the installed version before writing any command.
- **NV5** — GitHub `schedule` events can be delayed or dropped under platform load; cron is not a real-time guarantee. The docs confirm the default-branch requirement but not delivery latency [S11]. A missed weekly sweep is therefore possible; alert on job absence if the signal becomes load-bearing.
- **NV6** — pt-BR GitHub Docs returned 404 at both attempted URLs (§2). Portuguese-language guidance for the team could not be checked from the canonical source; re-check the slug before citing a translated page in a Portuguese-facing doc.
- **NV7** — The dependency-review action's license data comes from ClearlyDefined and may produce `LicenseRef-clearlydefined-OTHER` values [S8]; this was not reconciled with Flora's `deny.toml` allow-list (including the known `CDLA-Permissive-2.0` / `BSL-1.0` edge cases from 026 [S26]). Keep `license-check: false` until reconciled.
- **NV8** — SBOM tool trust boundary: `cargo-cyclonedx` invokes Cargo and may execute build scripts [S26]. No SBOM was generated here; generator version, output schema validation and artifact binding are unverified.
- **NV9** — No cost measurement was made. Hosted Dependabot is free for public repos; GHAS billing for private repos, runner minutes for scheduled scans, and artifact storage for SBOMs were not sized.
- **NV10** — This is research only. No workflow, config, `deny.toml`, `.gitignore`, `dependabot.yml`, or release step was modified. All YAML snippets above are descriptions of upstream docs, not applied Flora state.

---

## Citations

- **[S1]** GitHub Docs, *Dependabot alerts* (EN): https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts
- **[S2]** GitHub Docs, *Alertas de Dependabot* (ES): https://docs.github.com/es/code-security/dependabot/dependabot-alerts/about-dependabot-alerts
- **[S3]** GitHub Docs, *Alertes Dependabot* (FR): https://docs.github.com/fr/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S4]** GitHub Docs, *Dependabot アラート* (JA): https://docs.github.com/ja/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S5]** GitHub Docs, *Dependabot 警报* (zh-CN): https://docs.github.com/zh-cn/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S6]** GitHub Docs, *Abhängigkeitsüberprüfung* (DE): https://docs.github.com/de/code-security/supply-chain-security/understanding-your-software-supply-chain/about-dependency-review
- **[S7]** GitHub Docs, *Dependency review* (EN): https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/about-dependency-review
- **[S8]** GitHub, `actions/dependency-review-action` README (v5 options, ClearlyDefined, node24 runner): https://github.com/actions/dependency-review-action
- **[S9]** GitHub Docs, *Dependency graph supported package ecosystems* (Cargo row): https://docs.github.com/en/code-security/reference/supply-chain-security/dependency-graph-supported-package-ecosystems
- **[S10]** GitHub Docs, *Dependabot options reference* (Cargo, cooldowns, `dependency-type: indirect/all`): https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file
- **[S11]** GitHub Docs, *Events that trigger workflows* (`schedule`; default-branch requirement): https://docs.github.com/en/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#schedule
- **[S12]** Google, `google/osv-scanner` README (Cargo support, offline mode, data sources, privacy, v2 beta): https://github.com/google/osv-scanner
- **[S13]** Google, *OSV-Scanner documentation* (home): https://google.github.io/osv-scanner/
- **[S14]** Google, *OSV-Scanner — Offline Mode* (local DB layout, `--offline`, `--download-offline-databases`, GCS bucket, limitation): https://google.github.io/osv-scanner/usage/offline-mode/
- **[S15]** Google, *OSV-Scanner — GitHub Action* (PR/scheduled/release reusable workflows, SARIF, `fail-on-vuln`): https://google.github.io/osv-scanner/github-action/
- **[S16]** RustSec, *A vulnerability database for the Rust ecosystem* (OSV export, GitHub Advisory Database import, tooling): https://rustsec.org/
- **[S17]** RustSec, `cargo-audit` README (MSRV, trust warning, `--file`, `--ignore`, target filters, `audit bin`): https://github.com/RustSec/rustsec/blob/main/cargo-audit/README.md ; docs.rs, `cargo-audit 0.23.0`: https://docs.rs/cargo-audit/latest/cargo_audit/
- **[S18]** RustSec, `audit.toml.example` (`[advisories]`, `[database] path/url/fetch/stale`, `[output] deny`, `[yanked]`): https://github.com/RustSec/rustsec/blob/main/cargo-audit/audit.toml.example
- **[S19]** RustSec, `audit-check` GitHub Action README (PR path-filter + scheduled issue creation, permissions, fork limitation): https://github.com/rustsec/audit-check
- **[S20]** OWASP, *CycloneDX* project (ECMA-424) and *OWASP Developer Guide — CycloneDX*: https://owasp.org/projects/cyclonedx ; https://devguide.owasp.org/en/05-implementation/02-dependencies/03-cyclonedx/
- **[S21]** OWASP SCVS, *V2: Software Bill of Materials (SBOM) Requirements* and *V1: Inventory Requirements*; NTIA-minimum profile: https://scvs.owasp.org/scvs/v2-software-bill-of-materials/ ; https://scvs.owasp.org/scvs/v1-inventory/ ; https://scvs.owasp.org/bom-maturity-model/profiles/examples/ntia-minimum-elements/
- **[S22]** OWASP Cheat Sheet Series, *Dependency Graph SBOM* (generate per release, schema validation, signed attestation, limits of signatures/VEX): https://cheatsheetseries.owasp.org/cheatsheets/Dependency_Graph_SBOM_Cheat_Sheet.html
- **[S23]** OWASP Cheat Sheet Series, *Vulnerable Dependency Management* (early automation, per-CVE scoped suppression, patch verification, backporting): https://cheatsheetseries.owasp.org/cheatsheets/Vulnerable_Dependency_Management_Cheat_Sheet.html
- **[S24]** NTIA, *The Minimum Elements For a Software Bill of Materials (SBOM)*, 2021-07-12: https://www.ntia.doc.gov/report/2021/minimum-elements-software-bill-materials-sbom
- **[S25]** Flora local read-only cross-check (2026-11-16): `.github/workflows/{ci,rust}.yml`, `deny.toml`, `Cargo.lock`, `.gitignore:163`, `git status --porcelain`.
- **[S26]** Flora prior research (read-only): `docs/research/2026-11-16-026-cargo-deny-ci-gate.md`, `docs/research/2026-11-16-026-cargo-vet-exemptions.md`.
