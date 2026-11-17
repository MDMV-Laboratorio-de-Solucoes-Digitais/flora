# Flora — Security, Licensing & Supply-Chain Research (2026-11-16)

Scope: `/home/luis/development/MDMV/projetos/Arquivados/flora` (AGPL-3.0-or-later, Rust 1.98, 7-member workspace). Method: 20 web searches (en/pt/ko/de) + official docs fetched + local inspection of workflows, `deny.toml`, `supply-chain/*`, `Cargo.toml`, `src/main.rs`, `docs/adr/*`. All URLs accessed **2026-11-16**. P = primary.

## Executive summary
1. `docker-staging.yml:43` uses `if: ${{ secrets.X != '' }}`; GitHub forbids `secrets` in `if:` ⇒ the whole file fails validation, no job runs. Fix = env indirection. (High)
2. `rust.yml:35` runs bare `cargo test` at a root with `[package]` + `[workspace]`, no `default-members` ⇒ only the placeholder root crate (0 tests) is tested. `cargo test --workspace` required. (High)
3. No effective vuln gate: `cargo deny check bans licenses sources` omits `advisories`; OSV `fail-on-vuln: false`; dependency-review is PR-diff-only. (High)
4. `cargo vet` (455 exemptions, `continue-on-error`) and browser smoke are informational — detection, not enforcement. (High)
5. Required checks + branch protection + CODEOWNERS are GitHub-side; no CODEOWNERS/dependabot.yml in checkout. AGPL §13 governs hosted use; CORS/WS/OIDC controls are correct in shape but runtime-unverified. (High)

## Findings by theme

### A. `secrets` in step-level `if:`
GitHub Workflow syntax (P, #1): *"Secrets cannot be directly referenced in `if:` conditionals. Instead, consider setting secrets as job-level environment variables, then referencing the environment variables to conditionally run steps in the job."* Unset secret expressions return `""`. Contexts reference (P, #2): `steps.if` allows `github, needs, strategy, matrix, job, runner, env, vars, steps, inputs` — **no `secrets`**; `jobs.<id>.env` does allow `secrets`; job-level `if` allows only `github, needs, vars, inputs`. Repeated in Using secrets (P, #3). Canonical fix (store a boolean, not the secret):

```yaml
jobs:
  build-and-push:
    env:
      HAS_COOLIFY_WEBHOOK: ${{ secrets.COOLIFY_DEPLOY_WEBHOOK != '' }}
    steps:
      - name: Trigger Coolify Staging Deploy
        if: env.HAS_COOLIFY_WEBHOOK == 'true'
        env:
          COOLIFY_DEPLOY_WEBHOOK: ${{ secrets.COOLIFY_DEPLOY_WEBHOOK }}
          COOLIFY_API_TOKEN: ${{ secrets.COOLIFY_API_TOKEN }}
        run: curl -fsS -X POST "$COOLIFY_DEPLOY_WEBHOOK" -H "Authorization: Bearer $COOLIFY_API_TOKEN"
```

Pass via `env:` (quoted), not interpolated into `run:`. Secure use reference (P, #4): least privilege, `::add-mask::VALUE`, no structured secrets.

### B. "Green CI" requirements
Branch protection (P, #5): require PR, approvals, dismiss stale approvals, Code Owner review, status checks, up-to-date branches, conversation resolution, signed/linear history, no bypass. Caveats: check/job names must be unique across workflows (#6); required checks must have succeeded within 7 days (#7); path-filter-skipped workflows leave checks **Pending**, blocking merge (#1). Rulesets allow multiple simultaneous rules. Code owners (P, #8): *"To protect a repository fully ... define an owner for the CODEOWNERS file itself"* (e.g. `/.github/CODEOWNERS @owner`). Environments (P, #9): required reviewers, prevent self-review, branch restrictions, secrets withheld until approval; `docker-staging.yml` declares no `environment:`. Make checks truthful before requiring them; `continue-on-error: true` always reports success. Proposed required checks: `build`, `db-tests`, `dependency-review`, `CI`, then vet/e2e after promotion.

### C. cargo-deny / cargo-vet / OSV / SBOM (2026)
- **deny**: four checks (`advisories`, `bans`, `licenses`, `sources`); bare `cargo deny check` runs all (P, #10). Since 0.16.0 advisory severity fields removed ⇒ deny-by-default; `version = 2` opts in, but CI never runs `advisories` (P, #11–12; 0.21.0 out 2026-11-16). `deny.toml` uses `warn`/`allow` for duplicates/wildcards/sources vs upstream strict `deny` — staged rollout, not a hard boundary. Metadata trusted in good faith (#13).
- **vet**: Mozilla bootstraps with blanket exemptions; approval falls to `supply-chain/` code owners; FAQ calls exemptions *"backlog"* to shrink via `cargo vet suggest` (P, #14–15). 455 permanent exemptions + no CODEOWNERS = detection only.
- **OSV**: official reusable workflows default `fail-on-vuln: true`; PR-differential stops new vulns, scheduled scan catches new advisories on unchanged deps (P, #16–17). Flora disables failing.
- **SBOM**: CycloneDX 1.5 is ECMA-424, licences use SPDX IDs/expressions (P, #18–19). `release.yml` is strong (checksummed pinned tools, schema validation, immutable release, digest verification) but **tag-only, no tag published** ⇒ no SBOM yet. CRA: duty from 2027-12-11, not public (Recital 77), FOSS in scope only if commercial (P, #20–21); ENISA requires keeping it current.
- **Actions**: only `release.yml` pins SHAs; GitHub says SHA pinning is the only immutable-release method and offers a workflow lockfile (P, #4, #22).

### D. AGPL-3.0 (hosted use)
§13 (P, #23): *"if you modify the Program, your modified version must prominently offer all users interacting with it remotely through a computer network ... an opportunity to receive the Corresponding Source of your version by providing access to the Corresponding Source from a network server at no charge, through some standard or customary means of facilitating copying of software."* §0: *"Mere interaction with a user through a computer network, with no transfer of a copy, is not conveying."* FSF (P, #24): network use ≠ conveying; clause targets programs designed to accept requests/respond over a network; browser JS is conveyed. GNU HOWTO (P): `COPYING`, per-file notices, "Source" link. SPDX v3.29.0 (P, #25): use `AGPL-3.0-or-later`; bare `AGPL-3.0` deprecated. For Flora: a modified hosted service owes a prominent free source offer to all remote users; unmodified operation owes none under §13. pt-BR (phc.adv.br, 2026-10-08) and ko (sktelecom.github.io, 2026-09) context: delimit version/users/source; Apache-2.0/MIT/GPL-3.0 absorbed into the whole become AGPL. Fact-specific — counsel advised.

### E. OIDC / JWKS / WS-Origin / CORS
CORS — OWASP (P, #26): allow only selected trusted domains; never `*` or blind Origin reflection. MDN: no `null`, explicit origins with credentials, `Vary: Origin`. WS — OWASP (P, #27): *"Validate the Origin header on every handshake. Always use an explicit allowlist ... Avoid wildcards or substring matching"*; browser tokens in the **first WSS message, not the URL query**. CWE-1385 (P, #28) adds CSRF token, wss, auth; Origin is browser-only, complements auth. Auth — RFC 9700 BCP 240 (P, #29): public clients MUST use PKCE/S256, enforce `code_verifier`, exact redirects, no implicit grant, sender-constrain tokens; §4.3.2 *"Clients MUST NOT pass access tokens in a URI query parameter"*. OWASP OAuth2 (P, #30) concurs. OIDC Core §10.1.1 (P, #31): refetch `jwks_uri` on unknown `kid`, retain decommissioned keys; cache per issuer with TTL + cooldown. 2026 CVEs: GHSA-jvp4-q659-95mj (Portainer), GHSA-4hpj-wmpw-ghwq (Grav) leaked JWTs via `?token=` (#32).

## Remediation pointers
`rust.yml:35` → `cargo test --workspace`; fix staging `if:` via env indirection + Environment reviewers (**no deploy without authorization**); add `cargo deny check advisories` and OSV PR `fail-on-vuln: true`; shrink exemptions then drop `continue-on-error` + add CODEOWNERS; pin actions; publish first tag so the SBOM exists; add a "Source" link; verify WS token placement and `Vary: Origin` at runtime.

## Contrarian views
Informational gates are valid bootstraps, but 455 permanent exemptions prove only that the list was unchanged — call it detection. "deny advisories are redundant" fails while OSV is non-blocking and dependency-review is diff-only. "Fix staging first" re-enables a real deploy; fix the vacuous test gate first. "AGPL always forces publishing SaaS code" overstates §13's conditional, user-facing offer. "SBOM on release suffices" ignores the CRA life-cycle duty.

## Open questions
1. Branch protection, required checks, Dependabot, `staging` Environment configured in GitHub? (Not observable locally.)
2. Is `COOLIFY_DEPLOY_WEBHOOK` set, and is a deploy authorized? Treat as **no authorization**.
3. Does WS auth accept tokens in query strings? Is `Vary: Origin` emitted?
4. Org-level SHA-pinning policy? 5. Any EU commercial "placing on the market" (CRA)?
6. Do any of the 455 exemptions have known RustSec/OSV advisories? 7. Who retires the two `continue-on-error` gates?

## Full sources (accessed 2026-11-16; P = primary)
1. P https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
2. P https://docs.github.com/en/actions/reference/workflows-and-actions/contexts
3. P https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/use-secrets
4. P https://docs.github.com/en/actions/reference/security/secure-use
5. P https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule
6. P https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
7. P https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/troubleshooting-required-status-checks
8. P https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
9. P https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments
10. P https://embarkstudios.github.io/cargo-deny/checks/
11. P https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html
12. P https://github.com/EmbarkStudios/cargo-deny/blob/main/CHANGELOG.md
13. P https://embarkstudios.github.io/cargo-deny/checks/licenses/README.md
14. P https://mozilla.github.io/cargo-vet/how-it-works.html
15. P https://mozilla.github.io/cargo-vet/faq.html
16. P https://google.github.io/osv-scanner/github-action
17. P https://github.com/google/osv-scanner/blob/main/docs/github-action.md
18. P https://cyclonedx.org/docs/1.5/json
19. P https://cyclonedx.org/guides/sbom/generation
20. P https://eur-lex.europa.eu/eli/reg/2024/2847/oj/eng
21. P https://digital-strategy.ec.europa.eu/en/policies/cra-open-source
22. P https://docs.github.com/en/actions/how-tos/secure-your-work/lock-workflow-dependencies
23. P https://www.gnu.org/licenses/agpl-3.0.en.html
24. P https://www.fsf.org/bulletin/2021/fall/the-fundamentals-of-the-agplv3
25. P https://spdx.org/licenses
26. P https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html
27. P https://cheatsheetseries.owasp.org/cheatsheets/WebSocket_Security_Cheat_Sheet.html
28. P https://cwe.mitre.org/data/definitions/1385.html
29. P https://www.rfc-editor.org/rfc/rfc9700.html
30. P https://cheatsheetseries.owasp.org/cheatsheets/OAuth2_Cheat_Sheet.html
31. P https://openid.net/specs/openid-connect-core-1_0.html
32. P https://github.com/portainer/portainer/security/advisories/GHSA-jvp4-q659-95mj ; https://github.com/getgrav/grav/security/advisories/GHSA-4hpj-wmpw-ghwq ; https://owasp.org/www-community/vulnerabilities/Information_exposure_through_query_strings_in_url
Local: workflows, deny.toml, supply-chain/*, Cargo.toml, src/main.rs, LICENSE. Secondary: phc.adv.br, sktelecom.github.io.

## Next 80/20: bug-fix recommendation

**Verdict: fix only (a) — `.github/workflows/rust.yml:35`: `cargo test` → `cargo test --workspace`. Use `/speckit.bug.assess`.** Confidence: high (primary docs + local counts).

**(a) Vacuous gate confirmed.** Cargo `cargo-test(1)` Package Selection (P, https://doc.rust-lang.org/cargo/commands/cargo-test.html, accessed 2026-11-16): *"If the manifest is the root of a workspace then the workspaces default members are selected, otherwise only the package defined by the manifest will be selected"*; `--workspace` = *"Test all members in the workspace."* Cargo Workspaces (P, https://doc.rust-lang.org/cargo/reference/workspaces.html, accessed 2026-11-16): *"When a root package is present ... When unspecified, the root package will be used. In the case of a virtual workspace, all members will be used (as if --workspace were specified)."* Local: root `Cargo.toml` has `[package] name = "flora"` + `[workspace]` with 7 members, no `default-members`; `src/main.rs` = `Hello, world!`, 0 tests. One-line impact: 149 test attributes (core 35, api 25, tests 89; 43 `#[ignore]` DB-gated, run by `db-tests`) ⇒ **~106 tests actually execute per run, up from 0**. No secrets/deploy; reversible.
**(b) Staging `if:` is real but not first.** Canonical fix = job-level `HAS_COOLIFY_WEBHOOK: ${{ secrets.COOLIFY_DEPLOY_WEBHOOK != '' }}`, then `if: env.HAS_COOLIFY_WEBHOOK == 'true'`. Fixing it re-enables a **real Coolify deploy** on pushes to `dev`; guardrails forbid deploy without authorization. Gain = workflow availability, not verification.
**(c) Compare.** (a) very high gain / 1 line / no side effects. (b) medium / ~5 lines / triggers deploy. Branch protection: high governance value but cannot make a vacuous check meaningful; admin config; must follow (a).
**Recommendation:** (a) via **`/speckit.bug.assess`** (CI defect, matching `.specify/bugs/forgejo-rust-ci-rust-toolchain-action-not-found/`; `/speckit.specify` is for features, `/speckit.assess.intake` only captures ideas). Change **`.github/workflows/rust.yml` line 35 only**; the command writes `.specify/bugs/<slug>/{assessment,fix,test}.md`. Leave `docker-staging.yml` untouched pending authorization; add branch protection after the check is real.
