# 028 — Minimal, low-noise advisory coverage for Flora: concrete job plan and go/no-go

**Date:** 2026-11-16
**Scope:** read-only research + read-only Flora cross-check. **No workflow, config, `deny.toml`, `dependabot.yml`, or source file was created or changed.** Working tree was clean before and after (verified with `git status --porcelain`). This is a decision document for a later implementation task.
**Question:** which combination of GitHub Dependabot alerts, `actions/dependency-review-action`, a scheduled OSV-Scanner sweep and/or `rustsec/audit-check` gives Flora a real advisory signal without becoming a flaky blocking gate? Cover actions permissions, SHA pinning vs Dependabot alert coverage, `license-check: false`, failure severity, schedule cadence, cost.
**Method:** primary sources only for load-bearing claims — GitHub Docs and action repositories, OSV-Scanner docs and the reusable-workflow source at the pinned tag, RustSec. Sources were consulted in **EN, ES, DE, FR, JA and zh-CN**; pt-BR was re-attempted (§9). English canonical text governs every operative claim. Complements `2026-11-16-027-advisory-sbom-alternatives.md`, which framed the options; this document decides and specifies them [S27].

**Resumo (pt-BR):** o Flora já tem alertas do Dependabot ativos (83 alertas, 4 deles no `Cargo.lock` raiz — todos `rustls`, com versão corrigida disponível). O repositório é **público** e **não tem branch protection** em `dev`: nenhum check é obrigatório hoje. Decisão: manter os alertas como sinal principal, adicionar `dependency-review-action` em PR com `license-check: false` e `fail-on-severity: moderate` (vermelho visível, mas não-bloqueante), e **um** varredor agendado — OSV-Scanner semanal informativo com SARIF, limitado a `-L Cargo.lock`. Não adicionar `rustsec/audit-check` (mesmos dados do RustSec, cria issues), não tornar nada required check, não habilitar `license-check`. Custo: **R$ 0 / US$ 0** em repositório público.

---

## 1. Decision (go/no-go)

| # | Action | Verdict | Why |
|---|---|---|---|
| 1 | Keep **Dependabot alerts** as-is | **GO — no change** | Already enabled and already producing 4 actionable Rust alerts on the canonical `Cargo.lock`. Hosted, no local DB, $0. This is the load-bearing signal. |
| 2 | Add `actions/dependency-review-action@v5.0.0` on `pull_request` | **GO — non-blocking** | Catches *newly introduced* vulnerable deps at PR time; free on public repos; needs only `contents: read`. With no required checks on `dev`, a failed check is a visible red X, not a gate. |
| 3 | Add **one** scheduled sweep: OSV-Scanner reusable workflow, weekly, SARIF, `fail-on-vuln: false` | **GO — informational** | Detects advisories published *after* the last dependency change. SARIF dedupes in the Security tab. Offline DB path exists if the git-based RustSec fetch ever becomes the blocker again [S27]. |
| 4 | Add `rustsec/audit-check` as well | **NO-GO** | Same upstream RustSec data as OSV; scheduled mode opens one issue per advisory **including informational ones**. Duplicates triage burden without a second opinion [S4][S10]. |
| 5 | Make any of these a **required status check** | **NO-GO (for now)** | No branch protection exists today; adding it would also gate `docker-staging.yml` deploys. Revisit only with a named owner and a deliberate decision (§7). |
| 6 | `license-check: true` on dependency review | **NO-GO** | ClearlyDefined license data conflicts with `cargo deny check licenses` (crate LICENSE / SPDX allow-list). Keep licenses in cargo-deny [S2][S27]. |
| 7 | Enable Dependabot **security updates / version updates** broadly | **NO-GO now** | Currently disabled; would add PR noise across the non-canonical `apps/web` and `apps/frontend` trees. Optional narrow exception for `github-actions` if SHA pinning is adopted (§5). |
| 8 | Run `cargo deny check advisories` as a third gate | **NO-GO** | Same data, third failure mode, and the local git DB is the unresolved problem from 027 [S27]. |

**Bottom line:** three signals, two small workflow files, one existing toggle — and **zero blocking gates**.

---

## 2. Flora state (read-only cross-check, 2026-11-16)

Verified this session via `gh api` and local file reads. This **updates** several facts in 027.

| Item | Verified state | Evidence |
|---|---|---|
| Visibility / host | **PUBLIC** on GitHub (`MDMV-Laboratorio-de-Solucoes-Digitais/flora`); default branch **`dev`**; Issues enabled | `gh repo view --json visibility,isPrivate,nameWithOwner,hasIssuesEnabled`; `gh api … --jq .default_branch` |
| Dependabot alerts | **ALREADY ENABLED**, 83 open alerts | `gh api repos/…/dependabot/alerts?per_page=100` |
| Alert breakdown | npm: 27 `apps/frontend/package-lock.json`, 15 `apps/web/package.json`, 2 `apps/frontend/package.json`; rust: **4 root `Cargo.lock`**, 17 `apps/web/tauri/src-tauri/Cargo.lock`, 14 `apps/web/tauri/Cargo.lock`, 4 `apps/web/tauri/macro_bundle_updater_plugin/Cargo.toml` | same API, grouped by `.dependency.manifest_path` |
| The 4 canonical Rust alerts | `rustls` × 4 — `RUSTSEC-2026-0285` (medium, fix 0.23.45), `RUSTSEC-2026-0346` (medium, fix 0.23.46), `RUSTSEC-2026-0348` (low, 0.23.46), `RUSTSEC-2026-0349` (low, 0.23.46); all created 2026-11-02 / 2026-11-15 | same API, `security_advisory.references` |
| Dependabot security updates | **disabled**; GHAS/Code Security not enabled (alert assignment & AI fix unavailable) | `gh api … --jq '.security_and_analysis'` |
| Branch protection on `dev` | **NONE** — `required_status_checks: null`, `enforce_admins: false` | `gh api repos/…/branches/dev/protection` |
| Workflows | `ci.yml` (Bun frontend), `rust.yml` (fmt, strict clippy `--lib --bins`, `cargo deny check bans licenses sources`, build, test, 6 DB-gated suites), **`docker-staging.yml` (new since 027)** — push to `dev` on `Dockerfile`/`Cargo.toml`/`Cargo.lock`/`flora-*/**` → GHCR → Coolify webhook | local `.github/workflows/*` |
| Action pinning | All actions **tag-pinned** (`@v4`, `@v3`, `@v6`, `@v2`, `@cargo-deny`) — **none SHA-pinned** | local workflow reads |
| `deny.toml` | `[advisories] version = 2, ignore = []`; allow-list **now includes `"AGPL-3.0-or-later"`** (027 said it was missing — blocker resolved); `bans.multiple-versions = "warn"`, `sources.unknown-git = "warn"` | local read |
| `Cargo.lock` | 4,788 lines, 451 packages, 0 git sources (was 4,807 / 452 in 027) | `wc -l`, `grep -c` |
| `dependabot.yml` | absent | `test -f` |
| Working tree | clean | `git status --porcelain` |

**Consequences:**

1. **027's NV1 is resolved** (public repo ⇒ dependency-review and Dependabot are available without GHAS) and **NV2 is resolved affirmatively** for the canonical lockfile: Cargo alerts do fire on the root `Cargo.lock`, with fixed versions available.
2. **027's NV1 "flaky blocker" concern is currently moot**: there is no required check. Any new check is advisory-only until someone deliberately adds branch protection.
3. **Noise is already real**: 79 of 83 alerts point at non-canonical manifests (`apps/web` Tauri, `apps/frontend`). Do not add an issue-creating scanner on top of that.
4. `docker-staging.yml` means `dev` is a deploy trigger. A future required advisory check would become a staging-deploy gate — one more reason to keep these checks non-blocking.

---

## 3. Why these three and not more

All four tools consume the same underlying truth: RustSec exports to OSV in real time; the GitHub Advisory Database imports RustSec; Dependabot consumes GHADB [S8]. OSV-Scanner lists RustSec among its authoritative sources [S6]. **This is one oracle with different transports, not defence in depth** [S27]. The three chosen signals differ in *when* they fire, not in *what* they know:

| Signal | Fires when | Where the result lands | Blocking today? |
|---|---|---|---|
| Dependabot alerts (existing) | new GHADB advisory, or dependency graph changes on default branch | Security → Dependabot tab, email/digest | No |
| dependency-review-action | a PR adds/updates a dependency with a known advisory | PR check + job log/summary | **No** (no required checks) |
| OSV-Scanner weekly | a new advisory appears in OSV after the last code change | SARIF in Security → Code scanning | **No** (`fail-on-vuln: false`) |

The scheduled sweep is not redundant with Dependabot: Dependabot rescans on advisory publication too, but it is tied to the dependency graph of the default branch and to GHADB's reviewed-advisory scope; OSV-Scanner is an independent transport with offline capability and per-CVE SARIF history. The value is continuity, not a second opinion.

---

## 4. Exact workflow snippets (proposed — not applied)

Both snippets use SHAs resolved from the upstream tags via `gh api repos/<owner>/<repo>/tags` on 2026-11-16. The trailing `# vX` is a same-line version comment (§5), not the ref.

### 4.1 `.github/workflows/dependency-review.yml`

```yaml
name: Dependency Review

on: [pull_request]

permissions:
  contents: read

jobs:
  dependency-review:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@d23441a48e516b6c34aea4fa41551a30e30af803 # v6.1.0
        with:
          persist-credentials: false

      - name: Advisory review of introduced dependencies
        uses: actions/dependency-review-action@a1d282b36b6f3519aa1f3fc636f609c47dddb294 # v5.0.0
        with:
          fail-on-severity: moderate   # default is low — too noisy
          fail-on-scopes: runtime      # default; ignore dev-only deps
          license-check: false         # licenses stay with cargo deny (ClearlyDefined conflict)
          vulnerability-check: true
          warn-only: false             # visible red X; non-blocking until branch protection exists
          comment-summary-in-pr: never # default; keep PR threads quiet
```

Source of every option: [S2]. Minimal install shape and `permissions: contents: read`: [S1][S2]. v5.0.0 requires runner ≥ v2.327.1; `ubuntu-latest` satisfies this [S2]. First-party action; `contents: read` is the only permission needed because PR comments are disabled [S1][S2].

### 4.2 `.github/workflows/osv-scheduled.yml`

```yaml
name: Advisory sweep (OSV-Scanner, informational)

on:
  schedule:
    - cron: "37 14 * * 1"   # Mondays 14:37 UTC; off-peak minute
  workflow_dispatch:        # manual re-trigger if GitHub disables an idle schedule

permissions:
  actions: read             # SARIF upload via codeql-action
  contents: read            # checkout
  security-events: write    # SARIF upload to Security tab

jobs:
  osv-scan:
    uses: google/osv-scanner-action/.github/workflows/osv-scanner-reusable.yml@32d3cd482bdadfb38b186c0ad1b031902d55d932 # v2.7.1
    with:
      scan-args: |-
        -L Cargo.lock       # ONLY the canonical lockfile — do not recurse into apps/web
      fail-on-vuln: false   # informational: SARIF alert, no red failure on 4 known rustls CVEs
      upload-sarif: true
```

Source: reusable workflow inputs, job permissions and internal SHA pinning verified in the file at the pinned tag [S7]; `-L` single-lockfile syntax from OSV-Scanner v2 usage docs [S6]; default `scan-args` is `-r ./`, which **would** scan the whole tree (including the noisy Tauri lockfiles) — overridden deliberately [S7]. `workflow_dispatch` is included because public repos with 60 days of inactivity have scheduled workflows automatically disabled [S3].

### 4.3 Permission budget (least privilege)

| Workflow | `contents` | `actions` | `security-events` | `issues` / `checks` | Why |
|---|---|---|---|---|---|
| dependency-review | `read` | — | — | — | Diffs via dependency-review API; comments disabled [S1][S2] |
| OSV scheduled | `read` | `read` | `write` | — | `actions: read` + `security-events: write` are required for SARIF [S3][S7] |
| `rustsec/audit-check` (rejected) | `read` | — | — | `write` / `write` | Scheduled mode creates issues and checks; high noise [S4] |

Do **not** add `pull-requests: write` (PR comments), `issues: write`, or `checks: write` to the proposed workflows. Do **not** use `pull_request_target` — `pull_request` with a read-only token is sufficient and avoids the untrusted-checkout class GitHub explicitly warns about [S9].

---

## 5. SHA pinning vs Dependabot action alerts — resolve explicitly

This is a genuine trade-off, and Flora sits on the tag-pinned side of it today.

* **Fact A:** Dependabot alerts are generated for actions using **semantic versioning only**, not for SHA-pinned actions [S1][S11]. GitHub's secure-use reference repeats this verbatim [S9]. If Flora SHA-pins all actions, it loses action-vulnerability alerts.
* **Fact B:** GitHub's current hardening guidance says the strongest measure is a **workflow dependency lockfile** ("supersedes SHA pinning"), and otherwise to pin to a full-length commit SHA verified as coming from the action's own repository, not a fork [S9].
* **Fact C:** the tension is partly mitigateable. Dependabot **version updates** do maintain SHA-pinned actions when a same-line version comment is present (`actions/checkout@<sha> # v6`), and regenerate lockfile entries when a workflow dependency lockfile is onboarded [S9][S12]. `dependabot.yml` with `package-ecosystem: "github-actions"`, `directory: "/"`, `schedule.interval: weekly` is the documented minimal config [S12].

**Decision:** SHA-pin the two new workflows with same-line `# vX` comments (as in §4). Accept that **Dependabot will no longer alert on vulnerable GitHub Actions** once existing tag pins are converted; Cargo and npm alerts are unaffected. Do not claim action coverage in any report or ADR. Keep action freshness via a *later, narrow* `github-actions`-only Dependabot version-update config, not via broad version updates that would spam PRs across `apps/web`.

If the team prefers to keep action alerts, the fallback is to leave `@v5` / `@v6` tag pins on these first-party actions — GitHub explicitly permits tag pinning "only if you trust the creator" [S9] — but that accepts tag-move risk. Pick one and record it; do not mix silently.

---

## 6. Failure severity, `license-check: false`, cadence

**Severity.** `fail-on-severity` defaults to `low`; `moderate` is the chosen threshold — stricter than the default, less noisy than `critical` [S2]. The 4 root-lock alerts are 2 medium + 2 low, so `moderate` keeps the two medium rustls advisories visible in PRs that reintroduce/update them while ignoring low-severity noise. `fail-on-scopes: runtime` is the default and is kept deliberately: dev-only dependencies should not fail a PR. `warn-only: false` gives a real red check; flip to `true` if the red X is perceived as noise — but note that with no branch protection it cannot block merges either way [S1][S2].

The scheduled OSV job deliberately uses `fail-on-vuln: false` (documented default is `true`) [S3][S7]. Rationale: 4 known rustls advisories would make the weekly job permanently red before any fix lands, and a permanently red job gets ignored. SARIF alerts in the Security tab carry the signal instead; the job only turns red if a scan actually fails to produce results (the reusable workflow already fails a scan with no results file [S7]). Flip to `true` only after triage and a named owner exist.

**`license-check: false`.** dependency-review's license data comes from **ClearlyDefined**, not crate LICENSE files, and unrecognised licenses become `LicenseRef-clearlydefined-OTHER`, which is not in Flora's SPDX allow-list [S2]. Flora's `deny.toml` now allows `AGPL-3.0-or-later` from crate metadata; running both as blocking license gates would produce conflicting verdicts with no agreed precedence [S27]. `deny-licenses` is also deprecated [S2]. Keep `license-check: false`; the license gate remains `cargo deny check licenses` in `rust.yml`.

**Cadence.** Weekly, Mondays 14:37 UTC (off-peak minute): GitHub warns that `schedule` can be delayed and even dropped at high load, including the start of every hour, and suggests a non-peak time [S3]. The docs' own example is Monday 12:30 UTC [S3]; the minute is shifted here for that reason. Cron is UTC; workflows run only on the default branch and only if the file is on the default branch [S3]. Daily (the `rustsec/audit-check` example [S4]) is unnecessary for a 451-crate lockfile and multiplies noise.

---

## 7. Blocking-gate risk and the deploy path

Today `dev` has **no required status checks and no admin enforcement** (§2), so §4's failing PR check is informational. That is the intended state.

If branch protection is added later, the dependency-review check becomes a merge gate and, indirectly, a gate on `docker-staging.yml` (push to `dev` triggers GHCR build + Coolify deploy on `Cargo.toml`/`Cargo.lock` changes). Advisory publication does **not** change those paths, so a scheduled sweep cannot itself block a deploy — but a PR bumping a vulnerable crate could be blocked by a newly published advisory between opening and merging. That is a world-change failure of exactly the kind the upstream cargo-deny action warns about [S27]. Required-check status should therefore be a separate, explicit decision with a named triage owner, not a side effect of adding the workflow.

---

## 8. Cost

| Item | Cost on current setup |
|---|---|
| Dependabot alerts | **$0** — already enabled, hosted, no runner [S13] |
| dependency-review-action | **$0 incremental** — public repo; standard GitHub-hosted runners in public repos are free and do not consume included minutes [S13] |
| OSV-Scanner weekly | **$0** — one `ubuntu-latest` job/week (~1–2 min), free on public repos; 5-day SARIF artifact retention negligible; code scanning is available to publicly visible repos [S13] |
| GHAS / Code Security | **not required** for any of the three; only alert assignment / AI fixing needs a licence, including on public repos [S1][S2] |
| Human triage | the only real cost: name an owner and a weekly slot. Without it the scheduled sweep is theatre [S27] |

Change triggers: making the repo **private** would require Code Security/GHAS for dependency-review and would bill runner minutes against the plan's included compute (GitHub Free for organisations: $12/month) [S13]; larger/ARM runners are billed even for public repos [S13]. Note `docker-staging.yml` already uses `ubuntu-24.04-arm` — that is an existing cost, unrelated to this plan.

---

## 9. Multilingual reliability cross-check

The load-bearing "SHA-pinned actions are not alerted" limitation was checked in five translations of the Dependabot alerts page:

| Lang | SHA-pinning limitation | "Only advisories reviewed by GitHub trigger alerts" bullet |
|---|---|---|
| EN [S1] | "alerts are only generated for actions that use semantic versioning, not SHA versioning" | complete |
| ES [S14] | "…las acciones que usan el control de versiones semántico, no el control de versiones SHA" — correct | truncated: "Solo los avisos revisados por GitHub ." |
| DE [S15] | "…nicht für SHA-Versionierung" — correct | truncated: "Nur Empfehlungen, die von GitHub aus." |
| FR [S16] | "…et non le contrôle de version SHA" — correct | garbled: "Seuls les avis examinés par GitHub alertes." |
| JA [S17] | "SHA バージョン管理ではなく、セマンティック バージョン管理を使用するアクションに対してのみ生成されます" — correct | "GitHub のみが、アラートをトリガーします。" — loses "reviewed" |
| zh-CN [S18] | "仅针对使用语义版本控制（而不是 SHA 版本控制）的操作生成警报" — correct | "只有经 GitHub才会触发提醒。" — predicate incomplete |
| pt-BR | 404 at the attempted slugs (same as 027) [S27] | — |

**Rule:** the SHA-pinning limitation is stable across all five translations and is safe to relay in any language; the "reviewed advisories only" caveat is degraded in all five and must be quoted from English. Translations are consistency checks, never citations.

---

## 10. Risks / NVs

- **NV1** — OSV `-L Cargo.lock` was not executed against Flora's v4 lockfile. `-L` is documented for OSV-Scanner v2 [S6] and the reusable workflow passes `scan-args` through verbatim [S7], but end-to-end coverage of all 451 crates and the exact reporter behaviour are unverified. Dry-run with `workflow_dispatch` before relying on it.
- **NV2** — SHAs in §4 were resolved from upstream tags on 2026-11-16 via `gh api …/tags`; verify each belongs to the action's own repository (GitHub's own requirement) before applying [S9]. Tags are mutable; the SHAs are the pins, the comments are documentation only.
- **NV3** — dependency-review severity comes from GHADB/GHSA, not RustSec categorisation. `moderate` was calibrated against the 4 observed root-lock GHSA severities; no PR was run through the action here.
- **NV4** — branch protection could be added by someone else later, silently converting the PR check into a gate with staging-deploy impact (§7).
- **NV5** — public repos with 60 days of no activity have scheduled workflows disabled automatically; `workflow_dispatch` allows manual restart but there is no alert on absence [S3]. Relevant because Flora lives under `Arquivados/`.
- **NV6** — cron can be delayed or dropped under load; this is a weekly detector, not real-time [S3].
- **NV7** — alert noise is already high (79/83 alerts on non-canonical manifests). Dependabot's notification noise can be reduced with the documented weekly digest or auto-triage rules [S1]; neither was configured here.
- **NV8** — `license-check: false` means the PR check carries **no** license signal; do not describe the setup as license coverage. The `AGPL-3.0-or-later` allow-list change in `deny.toml` was observed but not audited for correctness.
- **NV9** — `rustsec/audit-check` was not executed; its issue-creation behaviour and "informational advisories don't affect check status" claim come from its README [S4].
- **NV10** — no cost measurement was made beyond the documented free-for-public-repos rules [S13]; no GHAS billing scenario was sized.
- **NV11** — research only. Nothing was applied; the YAML above is a proposal.

---

## Citations

- **[S1]** GitHub Docs, *Dependabot alerts* (EN) — https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts
- **[S2]** GitHub, `actions/dependency-review-action` README (v5 options, ClearlyDefined, node24, permissions, SHA-pinned releases) — https://github.com/actions/dependency-review-action
- **[S3]** GitHub Docs, *Events that trigger workflows* — `schedule` (default-branch requirement, delay/drop, 60-day inactivity, off-peak advice) — https://docs.github.com/en/actions/writing-workflows/choosing-when-your-workflow-runs/events-that-trigger-workflows#schedule
- **[S4]** RustSec, `audit-check` GitHub Action README (PR path-filter, scheduled issue creation, `issues: write`/`checks: write`, fork limitation) — https://github.com/rustsec/audit-check
- **[S5]** GitHub Docs, *Dependency review* (EN) — availability on public repos, branch protection required to block — https://docs.github.com/en/code-security/supply-chain-security/understanding-your-software-supply-chain/about-dependency-review
- **[S6]** Google, OSV-Scanner *Usage* (v2, `-L` single lockfile, `fix` warning) and *GitHub Action* (reusable workflows, `fail-on-vuln`, `scan-args`) — https://google.github.io/osv-scanner/usage/ ; https://google.github.io/osv-scanner/github-action/
- **[S7]** Google, `google/osv-scanner-action` reusable workflow source at `v2.7.1` (`workflow_call` inputs, job permissions, internal SHA pins, no-results failure) — https://github.com/google/osv-scanner-action/blob/v2.7.1/.github/workflows/osv-scanner-reusable.yml
- **[S8]** RustSec, *A vulnerability database for the Rust ecosystem* (OSV export, GHADB import) — https://rustsec.org/
- **[S9]** GitHub Docs, *Secure use reference* (least privilege, full-SHA pinning, workflow lockfiles "supersede SHA pinning", Dependabot SHA-alert limitation, `pull_request_target` warning) — https://docs.github.com/en/actions/reference/security/secure-use
- **[S10]** RustSec, `cargo-audit` README + `audit.toml.example` (DB git clone, `ignore`, categories) — https://github.com/RustSec/rustsec/blob/main/cargo-audit/README.md ; https://github.com/RustSec/rustsec/blob/main/cargo-audit/audit.toml.example
- **[S11]** GitHub Docs, *Dependabot alerts* — Limitations: "For GitHub Actions, alerts are only generated for actions that use semantic versioning, not SHA versioning" — same URL as [S1]
- **[S12]** GitHub Docs, *Keeping your actions up to date with Dependabot* (`github-actions` ecosystem config, same-line version comments) — https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/auto-update-actions
- **[S13]** GitHub Docs, *About billing for GitHub Actions* (public-repo runners free, Dependabot free, code scanning eligibility, included compute) — https://docs.github.com/en/billing/managing-billing-for-github-actions/about-billing-for-github-actions ; https://docs.github.com/en/code-security/how-tos/find-and-fix-code-vulnerabilities/configure-code-scanning/configure-code-scanning
- **[S14]** GitHub Docs ES, *Alertas de Dependabot* — https://docs.github.com/es/code-security/dependabot/dependabot-alerts/about-dependabot-alerts
- **[S15]** GitHub Docs DE, *Dependabot-Warnungen* — https://docs.github.com/de/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S16]** GitHub Docs FR, *Alertes Dependabot* — https://docs.github.com/fr/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S17]** GitHub Docs JA, *Dependabot アラート* — https://docs.github.com/ja/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S18]** GitHub Docs zh-CN, *Dependabot 警报* — https://docs.github.com/zh-cn/code-security/concepts/supply-chain-security/dependabot-alerts
- **[S25]** Flora local read-only cross-check (2026-11-16): `gh api` visibility/alerts/branch-protection/security-and-analysis; `.github/workflows/{ci,rust,docker-staging}.yml`; `deny.toml`; `Cargo.lock`; `git status --porcelain`
- **[S27]** Flora prior research (read-only): `docs/research/2026-11-16-027-advisory-sbom-alternatives.md`
