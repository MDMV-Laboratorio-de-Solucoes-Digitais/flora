# 026 — Minimal deterministic CI supply-chain gate (cargo-deny + cargo-vet)

**Date:** 2026-11-16  
**Scope:** read-only research + read-only Flora cross-check. No source, config, workflow, or gitignore changes were made.  
**Context:** AGPL-3.0-or-later Rust workspace, GitHub Actions CI, cheap-VPS SMB self-hosting. Goal: one small, reproducible gate that fails on the right things and does not fail on noise.

## Summary / decision

**Do not add the gate to `.github/workflows/rust.yml` yet: it would be red on the first run.** Two concrete blockers were verified locally:

1. `deny.toml` allows `"AGPL-3.0"` but all 9 workspace crates declare `AGPL-3.0-or-later`. With cargo-deny ≥ 0.18.4 GNU licenses are matched pedantically, so `cargo deny check licenses` **fails** (`error[rejected]`). Tested with cargo-deny **0.20.2**: adding `"AGPL-3.0-or-later"` to `allow` makes `bans ok, licenses ok, sources ok` (exit 0).
2. `supply-chain/` is **gitignored** (`.gitignore:163`) and is therefore absent from HEAD. `cargo vet` in a clean checkout fails with `ERROR × You must run 'cargo vet init' (store not found ...)`. Even locally, with the ignored directory present, `cargo vet check --locked --frozen` fails: `1 unvetted dependencies: multer:3.1.0 missing ["safe-to-deploy"]`.

**Recommended shape once those two blockers are fixed:** two jobs — (a) deterministic, offline `cargo deny --locked --offline check bans licenses sources` plus `cargo vet --locked --frozen`, and (b) a non-blocking/scheduled `cargo deny --locked check advisories` that fetches the RustSec DB. Split the network-dependent check from the offline ones; pin tool *versions* (not just an action tag); SHA-pin all actions; give jobs `contents: read`.

SBOM: defer. `cargo-cyclonedx` is the only mature source-level option today; Cargo's own SBOM precursor is still nightly-only (Rust Project Goal 2026). For a single-node VPS, an SBOM is a release artifact, not a PR gate.

---

## 1. Primary-source findings

### 1.1 Installing/running cargo-deny reproducibly in GitHub Actions

Two supported paths:

| Path | What it gives you | Caveat |
|---|---|---|
| `EmbarkStudios/cargo-deny-action@v2` | Official wrapper; defaults `command: check`, `arguments: --all-features` | Floating `@v2` can change behavior under you. A real 2026-07 incident: v2.1.0 forwarded the default `log-level: warn` as a positional subcommand and broke every `@v2` job; fixed in v2.1.1. [A1][A5] The action's *default* `--all-features` also conflicts with Flora's `all-features = false`. |
| `cargo install cargo-deny --version X.Y.Z --locked` (directly or via `taiki-e/install-action` with `tool: cargo-deny@X.Y.Z`) | Exact tool version; one install path shared by deny/audit/vet jobs | Pay the compile cost unless cached; pin the version because `deny.toml` schema interpretation changes between releases. |

Primary docs say `cargo install --locked cargo-deny` for installation, and `--locked` as a top-level cargo-deny flag asserts that `Cargo.lock` is unchanged. [A2][A3] GitHub's own hardening guidance is to pin actions to a full-length commit SHA (or use workflow dependency lockfiles), use least-privilege `GITHUB_TOKEN` (`contents: read`), and keep `persist-credentials: false` on checkout. [A4]

The upstream action README itself recommends splitting the matrix: `advisories` in one job with `continue-on-error`, and `bans licenses sources` in another, "to avoid sudden breakages" from newly published advisories. [A1]

**Version pinning is load-bearing.** In cargo-deny ≥ 0.18.4, GNU licenses are handled pedantically: allowing `GPL-3.0` does *not* automatically allow `GPL-3.0-or-later`; you must list the exact identifier. [A6] The local tool is 0.20.2 and exhibits this behavior.

### 1.2 Which checks need the network

- **`advisories`** — fetches `db-urls` (default `https://github.com/RustSec/advisory-db`) into `$CARGO_HOME/advisory-dbs` using the `git` CLI. [A7] This is the only inherently online check. With `--offline` / `check --disable-fetch` no fetch happens, but then `maximum-db-staleness` (default `P90D`) applies. [A7] Yanked checks read local registry indices.
- **`licenses`** — only needs Cargo metadata plus crate LICENSE files from a warm local registry cache; works fully offline once `cargo fetch --locked` has populated it (verified locally, see §2).
- **`bans` / `sources`** — pure graph/config evaluation; no network beyond producing metadata (verified locally offline).
- cargo-deny's `--offline` flag explicitly prevents both Cargo and cargo-deny from accessing the network and skips advisory-DB and `std-replacement-data` fetches; `--frozen` = `--locked` + `--offline`. [A3]

Consequence: a gate that runs `cargo deny check` with defaults in CI is *not* deterministic — it mutates the advisory DB every run and can fail because the world changed, not because Flora changed.

### 1.3 Avoiding flaky/over-strict license failures

Primary-source knobs in `[licenses]` [A6]:

- `unused-allowed-license = "allow"` — default is `"warn"`; Flora currently emits 7 such warnings. Silence them rather than letting them train reviewers to ignore output.
- Exact SPDX identifiers matter. Allow exactly what your manifests declare (`AGPL-3.0-or-later`), not the deprecated alias.
- `confidence-threshold = 0.8` (default) only applies when inferring a license from LICENSE text; keep it explicit. For crates with no machine-readable license, use `[[licenses.clarify]]` with hash-pinned license files, or `[[licenses.exceptions]]` for a one-off crate/version.
- `include-dev` defaults to **false** (dev-dependencies are not license-checked); `include-build` defaults to **true**. State this explicitly in policy so it is not mistaken for full coverage.
- Set `[advisories] yanked = "warn"` explicitly (Flora omits it; default is warn), and consider `unmaintained = "workspace"` / `unsound = "workspace"` (defaults) to avoid transitive-noise failures. [A7]
- Pass `--locked` so a dirty lockfile cannot silently change what is checked. [A3]

Flora's current `bans` settings (`multiple-versions = "warn"`, `wildcards = "allow"`) are deliberately non-fatal; the graph has many duplicate versions (base64 ×2, bitflags ×2, getrandom ×3, etc.). Do not "tighten" these in the same change as adding the gate — that would create a second, unrelated failing signal.

### 1.4 Handling a large cargo-vet exemptions list

Flora has **442** exemption entries — large, but within normal bootstrap range (a 2026 ecosystem survey reports median 131 / mean 206 exemptions per adopter; this is secondary data [A10]). Mozilla's own docs explicitly bless this: `cargo vet init` exempts all existing deps so you can get to green immediately and ratchet down later; "the files in this directory should be added to version control along with `Cargo.lock`". [A8][A9]

Tooling (all primary, from `cargo vet commands`) [A11]:

- `cargo vet suggest` — shows the review backlog with lowest-effort audits; use `suggest = false` on entries you intentionally will not act on.
- `cargo vet certify <crate> <v>` / `cargo vet diff <crate> <v1> <v2>` — full and delta audits; delta audits are what make a large backlog tractable.
- `cargo vet import <org>` / `cargo vet trust` — inherit Mozilla / Google / Bytecode Alliance audit sets instead of auditing everything locally. `cargo vet prune` removes now-unnecessary imports and exemptions.
- `cargo vet regenerate exemptions` — minimize/garbage-collect (adds only what is needed, weakens overbroad `safe-to-deploy` to `safe-to-run` where possible). This is the "most aggressive" fix; the docs advise preferring audits. [A11]
- CI flags: `--locked` (do not fetch new imported audits), `--frozen` (no network at all; requires warm cache), `--no-minimize-exemptions` (stop `check` from silently rewriting exemptions), `--no-all-features` (align with `deny.toml`'s `all-features = false` — cargo-vet defaults to passing `--all-features`). [A11]

Operational stance: an exemption is an IOU, not a verification. Bootstrap big lists are fine; the gate's value is that **new** crates cannot enter silently. A count-only ratchet (`count must not increase in a PR`) is a useful community pattern but is not part of cargo-vet itself [A12].

### 1.5 SBOM options

| Option | Status | Fit for Flora |
|---|---|---|
| `cargo-cyclonedx` (OWASP, Apache-2.0) | Mature; `cargo install cargo-cyclonedx`, then `cargo cyclonedx --format json --describe binaries`; sources from `Cargo.lock` **and** `cargo metadata`; `--license-strict` rejects deprecated `/` separators. It invokes Cargo internally (build scripts may run), so run only on trusted code in an isolated job. [A13][A14] | Best source-level SBOM today. Generate on tag/release, not on every PR. |
| `cargo-auditable` | Embeds dependency list in a linker section; readable later by `cargo audit bin`, Trivy, Grype, OSV-Scanner, Syft. [A14][A15] | Useful when binaries outlive their SBOM, but it replaces the `cargo build` invocation — defer, it perturbs the build. |
| Cargo SBOM precursor (`-Zsbom`) | Nightly-only; RFC #3553 unfinished; Rust Project Goal 2026 "Stabilize Cargo SBOM precursor" explicitly says `cargo metadata` gives inaccurate trees and the work is not done. [A16][A17] | Do not adopt now. Revisit after stabilization. |

RustSec notes advisory data is exported to OSV, so generic scanners (Trivy etc.) can consume Rust advisories without a Rust-specific SBOM. [A15]

---

## 2. Flora cross-check (read-only, exact results)

State of the checkout:

- Working tree **clean**. `deny.toml` is tracked and committed (added in `a58ceec06`, 2026-11-16). `.github/workflows/rust.yml` has only Build / Test / DB-backed contract tests — **there is currently no supply-chain job at all**, so nothing in CI fails or passes today.
- `Cargo.lock`: format v4, **452** packages, all `source = "registry+https://github.com/rust-lang/crates.io-index"`, **0** git sources.
- `supply-chain/` exists locally but is ignored: `.gitignore:163` → `supply-chain/`. `git archive HEAD` confirms the directory is absent from HEAD. Local files: `config.toml` 31.9 KB, `[cargo-vet] version = "0.10"`, **442** `[[exemptions.*]]` entries, **no** imports; `audits.toml` and `imports.lock` are empty headers only.
- Tool versions here: `cargo-deny 0.20.2`, `cargo-vet 0.10.3`, Rust pinned in CI to 1.98.0.

Commands actually run (network-restricted environment; crates.io unreachable):

| Command | Result |
|---|---|
| `cargo metadata --offline --format-version 1` | OK — 452 packages; license metadata available locally |
| `cargo deny --offline --locked --metadata-path … check bans licenses sources` | **exit 4**: `bans ok, licenses FAILED, sources ok` |
| ↳ rejection | `error[rejected]`: `package.license = "AGPL-3.0-or-later"` (root `Cargo.toml:21`) — not explicitly allowed; `licenses FAILED` |
| ↳ warnings | 7 × `license-not-encountered`: `AGPL-3.0`, `BSD-2-Clause`, `CC0-1.0`, `ISC`, `MPL-2.0`, `Unicode-DFS-2016`, `Zlib` (non-fatal) |
| same command with `"AGPL-3.0-or-later"` added to `allow` (simulated in `/tmp`, source untouched) | **exit 0**: `bans ok, licenses ok, sources ok` (same 7 warnings) |
| `cargo vet check --locked --frozen` (local, ignored store present) | **exit 255**: `1 unvetted dependencies: multer:3.1.0 missing ["safe-to-deploy"]` |
| `cargo vet check --locked --frozen --no-all-features` | same failure |
| `cargo vet check --locked --frozen` in clean `git archive HEAD` checkout | **exit 255**: `ERROR × You must run 'cargo vet init' (store not found at …/supply-chain)` |
| `cargo deny check bans sources` (no offline flags) | timed out — the environment cannot reach the network; offline mode above is the reliable local verification path |
| `cargo vet check --locked` (no `--frozen`) | timed out — tries network |

Additional license observations from the offline metadata (452 packages): 235 × `MIT OR Apache-2.0`, 76 × `MIT`, 41 × `Apache-2.0 OR MIT`, 19 × legacy `MIT/Apache-2.0`, 18 × `Unicode-3.0`, 9 × `AGPL-3.0-or-later`, plus `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`, `ISC AND (Apache-2.0 OR ISC)`, `CDLA-Permissive-2.0`, `BSL-1.0`, etc. All OR-expressions are satisfied by an allowed alternative, so the single observed rejection is the AGPL alias. **NV:** `webpki-root-certs`/`webpki-roots` (`CDLA-Permissive-2.0`) and `xxhash-rust` (`BSL-1.0`) are in the lockfile with licenses *not* in the allow list; they were not rejected under the current graph config, but could surface if graph settings change (see §3).

---

## 3. Concrete minimal CI step (apply only after the two blockers are fixed)

Blockers to fix first, in separate commits: (1) add `"AGPL-3.0-or-later"` to `deny.toml` `licenses.allow` and set `unused-allowed-license = "allow"`; (2) remove `supply-chain/` from `.gitignore`, commit `config.toml` / `audits.toml` / `imports.lock`, and close the `multer@3.1.0` gap via `cargo vet certify` or `cargo vet regenerate exemptions`. Then add to `.github/workflows/rust.yml` (reuse the existing 1.98.0 setup block; SHA values are placeholders to be resolved and verified against the upstream repos):

```yaml
  supply-chain:
    name: Supply chain (deterministic)
    runs-on: ubuntu-latest
    timeout-minutes: 15
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@<full-sha>            # v4
        with:
          persist-credentials: false
      # ... existing pinned 1.98.0 toolchain setup ...
      - name: Install pinned supply-chain tools
        run: |
          cargo install cargo-deny --version 0.20.2 --locked
          cargo install cargo-vet --version 0.10.3 --locked
      - name: Warm registry from lockfile
        run: cargo fetch --locked
      - name: cargo-deny (offline; bans/licenses/sources)
        run: cargo deny --locked --offline check bans licenses sources
      - name: cargo-vet (no network; committed store)
        run: cargo vet --locked --frozen --no-all-features --no-minimize-exemptions

  advisories:
    name: RustSec advisories (non-blocking)
    runs-on: ubuntu-latest
    continue-on-error: true                 # new advisories must not break unrelated PRs
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@<full-sha>            # v4
        with:
          persist-credentials: false
      # ... same pinned toolchain + cached cargo-deny install ...
      - name: cargo-deny advisories (fetches RustSec DB)
        run: cargo deny --locked check advisories
```

Notes on the step:

- `--offline` for the deterministic job is what makes bans/licenses/sources reproducible; advisories stay in their own network job, matching the upstream action's own recommended matrix. [A1][A3]
- `--no-all-features` aligns cargo-vet with `deny.toml`'s `all-features = false`. [A11]
- `--no-minimize-exemptions` prevents `check` from silently editing the committed store. [A11]
- If you prefer the official action instead of `cargo install`, pin `EmbarkStudios/cargo-deny-action@<full-sha> # v2.1.1` and set `arguments: --locked` (overriding its `--all-features` default) for the deterministic job. [A1][A4]
- Alternative (simpler first increment): gate only `cargo deny --locked --offline check bans licenses sources` and defer cargo-vet until the store is committed. That still catches new licenses/sources/duplicates.

SBOM increment (later, release-only): `cargo install cargo-cyclonedx --locked` and `cargo cyclonedx --format json --describe binaries` on tag pushes, upload the JSON next to the binary. [A13]

---

## 4. Risks / unknowns / non-verifiable items

1. **Network is unavailable here** (crates.io 403). `licenses` was verified only via `cargo metadata --offline` + cached crate sources and `--metadata-path`; a fresh GitHub runner must run `cargo fetch --locked` first or the offline job will fail for missing sources. Advisories could not be executed at all locally; its result is unknown until CI has network.
2. **cargo-deny 0.20.2 locally vs. whatever CI installs.** The AGPL rejection is version-sensitive (≥0.18.4). Pin the version in CI or the policy semantics can change under you.
3. **`deny.toml [graph] targets = []` is not obviously "all targets".** Docs say target-gated links are ignored if no listed target matches; omitting `targets` means "consider every resolved crate". [A18] Flora's intent should be confirmed — this may be why the `CDLA-Permissive-2.0`/`BSL-1.0` crates did not surface. If graph config changes, new license rejections are likely.
4. **Exemptions are not audits.** 442 entries give green CI but no assurance. Without a burn-down owner, the list grows with every dependency bump.
5. **cargo-vet store governance.** Once committed, `supply-chain/` should be CODEOWNERS-protected; otherwise a PR can self-exempt a crate. [A4][A8]
6. **Action pin drift.** SHA pins need Dependabot (which *does* update SHA-pinned actions, though it will not alert on vulnerable SHA-pinned actions — a known GitHub limitation). [A4]
7. **Dev/build dependency scope.** `licenses.include-dev = false` by default; cargo-vet's default is `--all-features`. The two tools do not see exactly the same graph unless flags are aligned.
8. **Not implemented here.** No workflow, config, gitignore, or lockfile was touched — this is research only. The YAML above is a proposal, not applied state.

---

## Citations

- **[A1]** EmbarkStudios, *cargo-deny GitHub Action* (marketplace README; matrix with `continue-on-error` for advisories, defaults `command: check` / `arguments: --all-features`): https://github.com/marketplace/actions/cargo-deny
- **[A2]** cargo-deny book, *cargo-deny* quickstart (`cargo install --locked cargo-deny`): https://embarkstudios.github.io/cargo-deny/
- **[A3]** cargo-deny book, *Common options* (`--config`, `--all-features`, `--locked`, `--offline`, `--frozen`): https://embarkstudios.github.io/cargo-deny/cli/common.html
- **[A4]** GitHub Docs, *Security hardening for GitHub Actions* (SHA pinning, `contents: read`, `persist-credentials`, CODEOWNERS, Dependabot): https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions
- **[A5]** Incident report (secondary), *cargo-deny-action v2.1.0 broke `@v2`; fixed in v2.1.1*: https://github.com/laurigates/loractl/issues/67
- **[A6]** cargo-deny book, *The `[licenses]` section* (pedantic GNU licenses since 0.18.4, `confidence-threshold`, `clarify`, `exceptions`, `unused-allowed-license`, `include-dev`/`include-build`): https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html
- **[A7]** cargo-deny book, *The `[advisories]` section* (default `db-urls`, git fetch, `yanked`, `unmaintained`, `unsound`, `maximum-db-staleness`, `--offline`/`--disable-fetch`): https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html
- **[A8]** cargo-vet book, *Setup* (`cargo vet init` exempts existing deps; commit `supply-chain/` with `Cargo.lock`): https://mozilla.github.io/cargo-vet/setup.html
- **[A9]** cargo-vet book, *FAQ* (auto-exempt all, then ratchet down with `suggest`): https://mozilla.github.io/cargo-vet/faq.html
- **[A10]** Light Squares (secondary), *A look at cargo-vet in 2026* (median 131 / mean 206 exemptions): https://www.lightsquares.dev/blog/cargo-vet-in-2026
- **[A11]** cargo-vet book, *Commands* (`check`, `suggest`, `certify`, `diff`, `import`, `trust`, `prune`, `regenerate exemptions`, `add-exemption`, global flags): https://mozilla.github.io/cargo-vet/commands.html
- **[A12]** Community pattern (secondary), *exemption ratchet in CI*: https://github.com/sebastienrousseau/static-site-generator/blob/main/supply-chain/README.md
- **[A13]** OWASP CycloneDX Rust Cargo plugin (repo + crates.io docs): https://github.com/CycloneDX/cyclonedx-rust-cargo and https://crates.io/crates/cargo-cyclonedx
- **[A14]** Safeguard (secondary), *How do you generate an SBOM for a Rust project?* (CycloneDX vs cargo-auditable, tool consumers): https://safeguard.sh/resources/blog/generate-an-sbom-for-a-rust-cargo-project
- **[A15]** RustSec, *Advisory Database / data interchange (OSV, Trivy, cargo-auditable, actions)*: https://rustsec.org/
- **[A16]** Rust Project Goals 2026, *Stabilize Cargo SBOM precursor* (nightly-only, RFC #3553, `cargo metadata` inaccuracies): https://goals.rust-lang.org/2026/stabilize-cargo-sbom.html
- **[A17]** rust-lang/rfcs PR #3553, *cargo-sbom*: https://github.com/rust-lang/rfcs/pull/3553
- **[A18]** cargo-deny book, *Configuration / `graph.targets`*: https://embarkstudios.github.io/cargo-deny/checks/cfg.html
