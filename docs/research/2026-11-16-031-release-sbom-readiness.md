> **Superseded by PR #242 (cycle 031):** the release workflow is now committed; check `.github/workflows/release.yml` and the current manifests for live state.

# 031 — Release/SBOM readiness audit: verify cycle 030 against the current tree

**Date:** 2026-11-16  
**Scope:** research + read-only audit only. **No `Cargo.toml`, workflow, config, `.gitignore`, tag, release or SBOM file was created or changed.** The only repository file produced by this task is this document. Scratch TOML experiments were confined to `/tmp/opencode/flora-c031-*` (outside the repo). All YAML/shell/TOML below are *descriptions of primary docs, verified against the current tree*, not applied Flora state.  
**Target:** `HEAD bb16ba9a9609cc62c2cf423c4e2737b5297190cd`, branch `dev`, working tree clean at task start.  
**Question:** verify the exact design in [030] (`docs/research/2026-11-16-030-release-sbom-exact-plan.md`) against the *current* tree; produce the exact, minimal, ordered edit list for (a) centralized workspace version, (b) `.github/workflows/release.yml` (tag trigger → version assert → BOM generate/validate → draft → upload → publish), (c) any docs; confirm the dotted-key finding and the exact 8 manifests/lines; decide whether creating the workflow without cutting a tag is safe and what must remain NV; cite primary sources; give risks/NVs and go/no-go.  
**Method:** primary sources only (Cargo Book, TOML 1.0.0, GitHub Docs, GitHub CLI manual, CycloneDX READMEs, crates.io API, GitHub API, `taiki-e/install-action` TOOLS.md) plus **read-only local checks** (`git`, `cargo metadata`, `jq`, scratch manifests) and live API re-resolution of every pin. English canonical text governs. Complements [028], [029], [030].

**Resumo (pt-BR):** o plano 030 resiste à verificação integral. HEAD é `bb16ba9a`, árvore limpa, zero tags, 8 pacotes `flora*` todos em `0.1.0`. O achado da chave pontilhada está **confirmado**: a raiz já define `workspace.package` com `package.rust-version`/`package.edition`/... (linhas 19–23), então criar `[workspace.package]` daria `duplicate key` — reproduzido localmente. A edição é **1 linha nova** (`package.version = "0.1.0"`) nesse bloco + `version.workspace = true` na linha 9 da raiz e na linha 8 dos 7 crates = **8 arquivos / 9 linhas**, mais um `release.yml` novo. Nenhum doc existente fica obsoleto; este é o único novo. Os 3 SHAs de action e as versões das ferramentas foram re-resolvidos nas APIs e conferem. **Veredito: GO para criar o workflow e as edições de manifesto; NO-GO para cortar tag/publicar neste ciclo.** Criar o workflow sem tag é seguro (trigger só em `push` de tag `v*`, nenhuma tag existe, path filter de `docker-staging.yml` não inclui `.github/`), mas **arma** uma capacidade `contents: write` que dispara no primeiro `v*`. Imutabilidade, bump+tag+push, dry-run do gerador (principal risco: README diz que ele emite um BOM por `Cargo.toml`), forma real do `metadata.component`, decisão de `--target` e execução do `cyclonedx` .NET no `ubuntu-latest` (README exige `libssl1.1`/`libicu66`, ausentes no 24.04) **permanecem NV**.

---

## 1. Decision and go/no-go

**Decision:** accept [030] as the exact edit set. The current tree matches every ground-truth assertion in [030] §2; the dotted-key trap is real and the minimal fix is correctly identified; all external pins re-resolve to the same values. No design change is needed.

| Gate | Go criterion | Status on current tree | Verdict |
|---|---|---|---|
| **G0 — platform** | Release immutability enabled in Settings → Releases *before* first release | Not observable from tree; plan/visibility unstated | **NV — blocking for full release path** |
| **G1 — manifests** | Exactly 8 `flora*` packages, all at same version, equal to tag minus `v` | ✅ 8 packages, all `0.1.0`; lockstep jq PASS for `0.1.0`, FAIL for `9.9.9` | **GO** (after edits) |
| **G2 — tag** | One annotated `vX.Y.Z` tag on intended commit, pushed once | ✅ zero tags today; tag creation is a deliberate future NV | **GO** (creation NV) |
| **G3 — tool pins** | `cargo-cyclonedx@0.5.10`, `cyclonedx@0.33.1`, action SHAs resolve | ✅ re-resolved from APIs on 2026-11-16 (§10 evidence log, items 9–12) | **GO, re-verify at apply time** |
| **G4 — dry run** | Exactly one BOM at root; `cyclonedx validate --input-version v1_5 --fail-on-errors` passes | ❌ cannot be tested without running the generator on a tag | **NV — blocking** |
| **G5 — identity** | `component.version` + `flora:release:*` properties injected and asserted | ❌ shape of `metadata.component` from 0.5.10 unobserved | **NV** |
| **G6 — target** | `--target` matches shipped artifact | ⚠️ staging image is `linux/arm64`; no release binary decision exists; workflow attaches **no binary**, only SBOM + checksums | **NV (low impact — see §8 R5)** |
| **G7 — publish path** | `gh` auth via `GITHUB_TOKEN` `contents: write`; `--verify-tag`; no pre-existing release | ❌ runtime-dependent; no release exists today | **NV** |
| **G8 — trust boundary** | Tag-push only; no `--clobber`; no post-publish asset edits | ✅ workflow design is fail-closed by construction | **GO** |

**Decision rule (unchanged from [030]):** GO for the *file edits* (§4) now. NO-GO for *cutting a tag or publishing a release* in cycle 031 — G0/G4/G5/G7 are unverified and the task is research-only. If G0 fails but G1–G3 hold, the artifact-only subset (generate → validate → upload workflow artifact, stop before `gh release create`) is the fallback — but note it still needs a tag push to fire, so it is also NV for this cycle.

---

## 2. Ground truth verified on the current tree (read-only)

| Item | Verified state (2026-11-16) | Evidence |
|---|---|---|
| HEAD / branch / cleanliness | `bb16ba9a…`, `dev`, clean at task start | `git rev-parse HEAD`, `git branch --show-current`, `git status --porcelain` |
| Tags | **Zero** | `git tag -l` → empty |
| Workspace members | 7 members + root package = **8 packages** | root `Cargo.toml:25–33`; `cargo metadata --no-deps` |
| Versions | All 8 = `0.1.0`, **no inheritance** | grep (§9.1); `cargo metadata --no-deps --format-version 1` |
| Root version key style | `[package] version = "0.1.0"` at **line 9**; `[workspace]` uses **dotted** `package.rust-version` … `package.repository` at **lines 19–23**; **no `[workspace.package]` header** | root `Cargo.toml:7–23`; `grep -n '\[workspace.package\]'` → absent |
| Member manifests | `version = "0.1.0"` at **line 8** of all 7; all other shared keys already `.workspace = true` | §9.1 |
| Workflows | Only `ci.yml`, `rust.yml`, `docker-staging.yml`, `osv-scan.yml`; **no `release.yml`**, no tag trigger | `ls .github/workflows/`; grep for `tags:` |
| Staging artifact | `ubuntu-24.04-arm`, `platforms: linux/arm64`, tags `staging` + `${{ github.sha }}`; path filter includes `Cargo.toml` | `docker-staging.yml:9,20,36–40` |
| CI toolchain | `1.98.0` via rustup; strict clippy; `taiki-e/install-action@cargo-deny` | `rust.yml:8–31,57–71` |
| Local cargo | `cargo 1.99.1 (5f94df478 2026-08-27)` — newer than CI 1.98.0; `workspace = true` inheritance is MSRV 1.64+, unchanged | `cargo --version` |
| `.gitignore` | No SBOM/BOM/release entries | grep (§10, item 15) |
| Docs outside `docs/research/` | No file mentions release/tag/SBOM mechanics → **no stale doc to update** | grep |

**Out of scope (confirmed):** `apps/web/tauri/**/Cargo.toml` (11 manifests) belong to the imported `@coparse/web` app, not the Flora workspace (per `AGENTS.md`). `find` excluding `apps/web` returns exactly the 8 manifests above. **Do not edit them.**

---

## 3. Dotted-key finding — CONFIRMED

### 3.1 The finding

Root `Cargo.toml` lines 17–23 (verbatim):

```toml
[workspace]
resolver = "3"
package.rust-version = "1.98"
package.edition = "2024"
package.license = "AGPL-3.0-or-later"
package.authors = ["Flora Workspace  - MDMV Laboratório de Soluções Digitais"]
package.repository = "https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/MDMV"
```

These dotted keys **already define** the `workspace.package` table. TOML 1.0.0 forbids redefining a table by header; adding `[workspace.package] version = "0.1.0"` therefore fails. [U2]

### 3.2 Local reproduction (scratch, `/tmp/opencode`, repo untouched)

- Scratch A — `[workspace] package.version = "0.2.0"` + `[package] version.workspace = true` → `cargo metadata` resolves the package at **`0.2.0`**. Inheritance works from a dotted key, and a root package may inherit from its own workspace.
- Scratch B — same dotted `package.*` block **plus** a trailing `[workspace.package] version = "0.2.0"` header →

  ```
  error: duplicate key
    --> Cargo.toml:10:12
     |
  10 | [workspace.package]
     |            ^^^^^^^
  ```

**Conclusion:** the only valid minimal edit is a new **dotted line** `package.version = "0.1.0"` inside the existing block. Adding a `[workspace.package]` table header (as the illustrative 028/029 prose shows) is wrong and will break `cargo metadata`. [U1][U2]

### 3.3 The exact 8 manifests and 9 lines

| # | File | Exact line | Current text | Replacement |
|---|---|---|---|---|
| 1 | `Cargo.toml` (root `[package]`) | **9** | `version = "0.1.0"` | `version.workspace = true` |
| 2 | `Cargo.toml` (root `[workspace]`) | **new 19**, before `package.rust-version` | — | `package.version = "0.1.0"` **(single source of truth)** |
| 3 | `flora-core/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |
| 4 | `flora-api/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |
| 5 | `flora-messaging/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |
| 6 | `flora-tasks/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |
| 7 | `flora-files/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |
| 8 | `flora-search/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |
| 9 | `flora-tests/Cargo.toml` | **8** | `version = "0.1.0"` | `version.workspace = true` |

**Totals: 8 files, 9 lines** (8 replacements + 1 insertion), plus one new file `release.yml` = 9 files touched. No `Cargo.lock` change is expected; verify with `cargo metadata --locked` / `git diff` at apply time (NV18).

---

## 4. Exact minimal ordered edit list

Apply in this order. Nothing else in the repo is edited.

### 4.1 Root `Cargo.toml` — two edits

**Edit A — line 9:**

```toml
[package]
name = "flora"
version.workspace = true          # was: version = "0.1.0"
description = "Flora Workspace - Self-hostable corporate collaboration platform"
```

**Edit B — insert as new line 19, inside the existing dotted block:**

```toml
[workspace]
resolver = "3"
package.version = "0.1.0"         # NEW — single source; bump this one line only
package.rust-version = "1.98"
package.edition = "2024"
package.license = "AGPL-3.0-or-later"
package.authors = ["Flora Workspace  - MDMV Laboratório de Soluções Digitais"]
package.repository = "https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/MDMV"
```

Do **not** add `[workspace.package]` (§3.2). A cosmetic refactor converting all five dotted keys into a table is optional, larger, and explicitly out of scope for the minimal diff.

### 4.2 Seven member manifests — one edit each

Identical replacement on **line 8** of each file listed in §3.3, rows 3–9:

```toml
version = "0.1.0"
```
→
```toml
version.workspace = true
```

All seven already inherit `repository/license/authors/rust-version/edition` via `.workspace = true`, so this matches existing style. [U1]

### 4.3 New file `.github/workflows/release.yml`

Create exactly the fail-closed workflow from [030] §3.3. Reproduced here verbatim for a self-contained edit list; it is **not applied state**. Pins were re-verified from the GitHub API on 2026-11-16 (§10, items 9–11), but are mutable tag aliases — re-resolve at apply time (NV12).

```yaml
name: Release (tag -> immutable GitHub Release + CycloneDX SBOM)

on:
  push:
    tags:
      - 'v*'   # glob filter only; exact vX.Y.Z shape is asserted in step 2

concurrency:
  group: release-${{ github.ref }}
  cancel-in-progress: false

permissions:
  contents: read   # least privilege by default; job elevates only where needed

jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write   # gh release create / upload / edit (and verify)
    env:
      GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      SBOM: flora-sbom.cdx.json
      TARGET: aarch64-unknown-linux-gnu   # matches docker-staging.yml shipped image
    steps:
      - name: Checkout (full history so annotated tag notes resolve)
        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262 # v4
        with:
          fetch-depth: 0

      - name: Assert tag shape is vX.Y.Z
        shell: bash
        run: |
          set -euo pipefail
          [[ "$GITHUB_REF_NAME" =~ ^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] \
            || { echo "::error::tag must be vX.Y.Z (no leading zeroes): $GITHUB_REF_NAME"; exit 1; }
          echo "RELEASE_VERSION=${GITHUB_REF_NAME#v}" >> "$GITHUB_ENV"

      - name: Set up Rust 1.98.0
        shell: bash
        run: |
          set -euo pipefail
          rustup toolchain install 1.98.0 --profile minimal --no-self-update
          rustup default 1.98.0
          rustc --version && cargo --version

      - name: Install pinned SBOM tools (checksummed, no fallback)
        uses: taiki-e/install-action@ab49d3c64680143e184118bed66a46419fc15519 # v2
        with:
          tool: cargo-cyclonedx@0.5.10,cyclonedx@0.33.1
          fallback: none     # both tools ship prebuilt GitHub Release binaries
          checksum: true

      - name: Assert lockstep version across all 8 manifests
        shell: bash
        run: |
          set -euo pipefail
          cargo metadata --no-deps --format-version 1 > /tmp/flora-metadata.json
          jq -e --arg v "$RELEASE_VERSION" '
            ([.packages[] | select(.name == "flora" or (.name | startswith("flora-")))] | length == 8)
            and ([.packages[] | select((.name == "flora" or (.name | startswith("flora-"))) and .version != $v)] | length == 0)
          ' /tmp/flora-metadata.json

      - name: Generate deterministic CycloneDX 1.5 JSON
        shell: bash
        run: |
          set -euo pipefail
          cargo cyclonedx --manifest-path Cargo.toml --format json \
            --describe crate --all --spec-version 1.5 --license-strict \
            --target "$TARGET" --override-filename "$SBOM"
          test -f "$SBOM" || { echo "::error::BOM not written to workspace root"; exit 1; }
          # fail closed if the generator emitted per-manifest duplicates
          if find . -type f -name "$SBOM" -print | grep -Fxv "./$SBOM" | grep -q .; then
            echo "::error::generator emitted additional BOMs; need explicit selection/merge"; exit 1
          fi

      - name: Inject tag identity and assert it (idempotent)
        shell: bash
        run: |
          set -euo pipefail
          jq -e '.metadata.component.version? // error("metadata.component.version missing")' "$SBOM" >/dev/null
          jq --arg ver "$RELEASE_VERSION" --arg tag "$GITHUB_REF_NAME" --arg sha "$GITHUB_SHA" '
            .metadata.component.version = $ver
            | .metadata.properties = (
                ((.metadata.properties // []) | map(select((.name // "") | startswith("flora:release:") | not)))
                + [ {name:"flora:release:tag",    value:$tag},
                    {name:"flora:release:commit", value:$sha},
                    {name:"flora:release:source", value:"https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora"} ]
              )
          ' "$SBOM" > "$SBOM.tmp"
          mv "$SBOM.tmp" "$SBOM"
          jq -e --arg ver "$RELEASE_VERSION" --arg tag "$GITHUB_REF_NAME" --arg sha "$GITHUB_SHA" '
            (.metadata.component.version == $ver)
            and ([.metadata.properties[] | select(.name == "flora:release:tag" and .value == $tag)] | length == 1)
            and ([.metadata.properties[] | select(.name == "flora:release:commit" and .value == $sha)] | length == 1)
          ' "$SBOM" >/dev/null

      - name: Validate BOM against CycloneDX 1.5 schema
        shell: bash
        run: cyclonedx validate --input-file "$SBOM" --input-format json --input-version v1_5 --fail-on-errors

      - name: Checksum (pre-publication)
        shell: bash
        run: |
          set -euo pipefail
          sha256sum "$SBOM" > checksums.txt
          cat checksums.txt

      - name: Upload BOM for inspection (net gain even without a release)
        uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02 # v4
        with:
          name: flora-sbom-${{ github.ref_name }}
          path: |
            ${{ env.SBOM }}
            checksums.txt
          if-no-files-found: error

      - name: Refuse to publish over an existing release/draft
        shell: bash
        run: |
          set -euo pipefail
          if gh release view "$GITHUB_REF_NAME" >/dev/null 2>&1; then
            echo "::error::release/draft already exists for $GITHUB_REF_NAME; resolve manually, never clobber"; exit 1
          fi

      - name: Create DRAFT (no assets attached here)
        shell: bash
        run: gh release create "$GITHUB_REF_NAME" --draft --verify-tag --notes-from-tag --title "$GITHUB_REF_NAME"

      - name: Attach assets (no --clobber)
        shell: bash
        run: gh release upload "$GITHUB_REF_NAME" "$SBOM" checksums.txt

      - name: Publish (irreversible once immutability is on)
        shell: bash
        run: gh release edit "$GITHUB_REF_NAME" --draft=false

      - name: Verify release attestation and asset digest
        shell: bash
        run: |
          set -euo pipefail
          gh release verify "$GITHUB_REF_NAME"
          gh release verify-asset "$GITHUB_REF_NAME" "$SBOM"
```

The YAML above is byte-for-byte the workflow in [030] §3.3 (pins re-verified, §10 items 9–11), including the `gh release view "$GITHUB_REF_NAME"` guard in the "Refuse to publish" step.

**Ordering rationale (primary-doc grounded):** `gh release create` with file arguments performs draft → upload → publish as separate, non-atomic requests, and immutability takes effect only on publication [U4]; assets cannot be modified/deleted and the tag cannot be moved afterwards [U7]; `--clobber` deletes before re-upload and loses the original on failure [U5]. Therefore the BOM is generated, identity-asserted and schema-validated *before any release object exists*; create is called **without file arguments** and with `--draft`; assets are attached once; publish is last. `--notes-from-tag` requires an annotated tag and `--verify-tag` aborts if the tag is not on the remote [U4]. `fetch-depth: 0` makes the annotated tag object and notes available (NV19).

**Optional control (not in the minimal diff):** an `environment: release-approval` gate, or a second publish job behind manual approval, if a human GO click is wanted before `gh release edit --draft=false`. Does not change ordering.

### 4.4 Docs

**No existing doc requires an edit.** A grep over `docs/` excluding `docs/research/` found no file describing release/tag/SBOM mechanics; `README`, `AGENTS.md`, `LICENSE`, `NOTICE` and the two ADRs are unaffected. **The only doc produced by this cycle is this readiness audit.** A future short release runbook that records the one-time Settings step and the bump procedure would be useful but is explicitly deferred (NV), not part of the minimal diff.

### 4.5 Files that must NOT change

`rust.yml`, `ci.yml`, `osv-scan.yml`, `docker-staging.yml`, `LICENSE`, `NOTICE`, `.gitignore`, any `apps/web/tauri/**/Cargo.toml`. The generated BOM lives only in the runner workspace and in workflow artifacts/release assets — never committed. [030] §3.4

---

## 5. One-time operations (not file edits — all remain NV)

1. Merge §4.1–4.4. Note: changing root `Cargo.toml` triggers one extra staging image rebuild on `dev` (path filter `Cargo.toml` in `docker-staging.yml:9`) — expected cost.
2. Enable immutability: repo Settings → Releases → *Enable release immutability* (or org policy). Applies to **future releases only** [U8] — do this before the first tag. **G0, blocking.**
3. Pre-flight locally (§9 command), evidence before tagging.
4. Bump the one root line, commit on `dev`, push commit, then create one annotated tag (`git tag -a v0.1.0 …`, `git push origin v0.1.0`) — never `-f`. Push the commit first so `--verify-tag` passes [U4][U13].
5. Watch the job. A failure before `gh release edit --draft=false` is repairable in the draft (or by deleting the draft and re-running); a failure after publication is **not** repairable by replacing assets [U5][U7].

---

## 6. Is creating the workflow (without cutting a tag) safe?

**Yes — mechanically inert, with one arming caveat.** Concretely:

- **Trigger cannot fire.** `on.push.tags: ['v*']` matches only tag pushes. Merging `release.yml` on `dev` is a branch push; `git tag -l` is empty today. No run occurs, no release object is created, no BOM is generated.
- **No collateral CI.** `docker-staging.yml` path filter is `Dockerfile`, `Cargo.toml`, `Cargo.lock`, `flora-*/**` — `.github/workflows/**` is **not** included, so adding the workflow alone does **not** rebuild the staging image. `rust.yml` runs on every push but a YAML-only addition cannot affect its fmt/clippy/test result. `ci.yml` runs in `apps/frontend`. (If the manifest edits land in the same commit, the staging rebuild does happen — expected, §5.1.)
- **Fail-closed by construction** once it does run: tag-shape regex, lockstep `cargo metadata` + `jq` assert, duplicate-BOM guard, `metadata.component.version` guard, refuse-over-existing-release. No `--clobber`, no post-publication uploads.

**Arming caveat / mitigation:** merging the file installs a latent `contents: write` capability that activates on the **first `v*` tag push by anyone who can push tags**. The guards prevent accidents and clobbering but do not replace tag-push discipline. Recommended before merging: (a) restrict who can push tags (tag protection / branch protection), (b) keep immutability enablement and the tag+push as deliberate manual NV steps (§5), (c) optionally add the manual-approval environment gate from §4.3. None of these change the edit list.

**Answer:** safe to create in this (or the implementation) cycle **provided no tag is pushed** and the one-time operations stay NV. It is safe precisely *because* the design is tag-gated — creating the workflow is not equivalent to cutting a release.

---

## 7. What must remain NV

| ID | Must remain NV | Why it cannot be closed by creating files |
|---|---|---|
| **G0 / NV3** | Release immutability availability + enablement (repo Settings / org policy) | Platform-side setting, not a file edit; availability on this plan/visibility unstated [U7][U8] |
| **G4 / NV10–NV11, NV14** | Generator dry run: number of BOM files, `--override-filename` behaviour, `metadata.component` shape, `--target` without installed std | Requires actually running `cargo cyclonedx` on the tagged tree; README says default output is adjacent to **every** `Cargo.toml` [U9] |
| **G4 / NV13 (sharpened)** | `cyclonedx` CLI actually executes on `ubuntu-latest` | README states .NET Core runtime deps are required and lists `libssl1.1`/`libicu66` for Ubuntu [U10] — not present on 24.04 (see §8 R2) |
| **G5** | Identity injection shape validated on a real BOM | Cannot assert `metadata.properties` placement until a file is generated |
| **G6 / NV5** | `--target` vs shipped-artifact decision | No release binary decision exists; workflow attaches only SBOM + checksums (§8 R5) |
| **G7 / NV17** | `gh` auth, `--verify-tag`, no pre-existing release/draft | Runtime state; workflow fails closed but is not idempotent |
| **NV4** | Remote default-branch check | Tag trigger is branch-independent, but the version commit must be pushed to the real default branch |
| **NV12** | Action SHA re-resolution at apply time | Tags `v4`/`v2` are mutable aliases; re-verify immediately before applying |
| **NV18** | `Cargo.lock` diff from inheritance | Verify with `cargo metadata --locked` / `git diff` during implementation |
| **NV9** | The research-only boundary itself | No tag, release, SBOM or config change was made in this cycle |

---

## 8. Risks / NVs (carried, sharpened, new)

Carried from [030] NV1–NV21 unless sharpened below. Load-bearing additions:

- **R1 — per-manifest BOM output (NV10, confirmed source).** [U9] README: "`cargo cyclonedx` … produces a `bom.xml` file adjacent to **every `Cargo.toml` file that exists in the workspace**." With 8 manifests, `--override-filename` may yield 8 files named `flora-sbom.cdx.json` in 8 directories. The workflow's `find … | grep -Fxv` guard fails closed, but the *first* real run may therefore stop at generation. Requires either confirming `--describe crate` + `--manifest-path Cargo.toml` produces a single root file, or adding an explicit selection/merge step. **Blocking dry-run gate G4.**
- **R2 — `cyclonedx` CLI runtime on `ubuntu-latest` (NV13, sharpened with primary source).** [U10] README: "**.NET Core runtime dependencies are required.** For Ubuntu these are libc6 libgcc1 libgssapi-krb5-2 libicu66 libssl1.1 libstdc++6 zlib1g." `libssl1.1` and `libicu66` are not in `ubuntu-24.04` (it ships `libssl3`/`libicu74`). The 0.33.1 release does publish a 80 MB `cyclonedx-linux-x64` single-file binary [API log], which may bundle the runtime, but the documented native deps may still bite. Mitigations if it fails: use the documented `cyclonedx/cyclonedx-cli` Docker image, `apt` the missing libs, or pin the job to `ubuntu-22.04`. **Must be trialled before G0/G3 are green.**
- **R3 — duplicate-key trap (§3, resolved in the plan).** Adding `[workspace.package]` breaks `cargo metadata`. The edit list uses a dotted line. Verified locally.
- **R4 — tool pin mutability (NV12).** All three action SHAs and both tool versions were re-resolved today, but `v4`/`v2` are aliases. Re-resolve at apply time.
- **R5 — `--target` is a documentation choice, not a binary binding (NV5 refined).** The workflow attaches **no compiled binary** — only the SBOM and `checksums.txt`. The staging image is already published by `docker-staging.yml` (`linux/arm64`) [tree]. Setting `TARGET: aarch64-unknown-linux-gnu` makes the SBOM describe the right dependency graph, but no attached artifact is validated against it. Low impact; decide deliberately if a release binary is ever added.
- **R6 — immutability is irreversible (G0/NV3).** Once on, published assets/tag cannot change and a tag name cannot be reused even after deleting a release [U7][U8]. Do not tag until G0–G8 pass.
- **R7 — research-only (NV9).** No repo file except this document was created; scratch work stayed in `/tmp/opencode`. No defect was found in the [030] design itself; the only corrections this audit adds are the sharpened R1/R2 evidence and the explicit "no binary attached" clarification in R5.

---

## 9. Pre-flight / acceptance checks (exact commands, read-only where marked)

```bash
# 9.1 Current state (already executed — §10)
git rev-parse HEAD; git status --porcelain; git tag -l
grep -n '^version = ' Cargo.toml flora-{core,api,messaging,tasks,files,search,tests}/Cargo.toml

# 9.2 Post-edit lockstep assert (run locally before tagging)
cargo metadata --no-deps --format-version 1 \
  | jq -e --arg v 0.1.0 '([.packages[] | select(.name=="flora" or (.name|startswith("flora-")))] | length == 8) and ([.packages[] | select((.name=="flora" or (.name|startswith("flora-"))) and .version != $v)] | length == 0)'

# 9.3 Lockfile hygiene (NV18)
cargo metadata --locked --format-version 1 >/dev/null && git diff --exit-code -- Cargo.lock

# 9.4 Tool pins (re-run at apply time)
# crates.io API max_version == 0.5.10 ; GitHub API cyclonedx-cli latest tag == v0.33.1
# actions/checkout@v4, actions/upload-artifact@v4, taiki-e/install-action@v2 SHAs == §10 items 9–11
```

Acceptance: 9.2 PASS for the intended version and FAIL for a wrong one; 9.3 no diff; 9.4 pins unchanged. Then, and only then, do the §5 one-time operations.

---

## 10. Evidence log (commands/requests actually executed, read-only)

1. `git rev-parse HEAD` → `bb16ba9a9609cc62c2cf423c4e2737b5297190cd`; `git branch --show-current` → `dev`; `git status --porcelain` → empty; `git tag -l` → empty.
2. `grep -n '^version = '` across root + 7 members → exactly the 8 lines in §3.3, all `0.1.0`. `grep -n '\[workspace.package\]' Cargo.toml` → absent.
3. `cargo metadata --no-deps --format-version 1 | jq -c '[.packages[]|{name,version}]'` → 8 packages (`flora-core`, `flora-api`, `flora-messaging`, `flora-search`, `flora-tasks`, `flora-files`, `flora-tests`, `flora`), all `0.1.0`. `cargo --version` → `1.99.1`.
4. Lockstep jq expression executed against live metadata: prints `true` for `EXPECTED=0.1.0`, `false` for `EXPECTED=9.9.9`.
5. Scratch A (`/tmp/opencode/flora-c031-dotted`): dotted `package.version = "0.2.0"` + `version.workspace = true` → metadata resolves `0.2.0`.
6. Scratch B (`/tmp/opencode/flora-c031-dup`): dotted `package.*` block + `[workspace.package]` header → `error: duplicate key` at the header (§3.2).
7. `find . -name Cargo.toml -not -path './target/*' -not -path './apps/web/*'` → exactly the 8 workspace manifests.
8. `.github/workflows/` listing → `ci.yml`, `rust.yml`, `docker-staging.yml`, `osv-scan.yml`; `release.yml` absent; no `tags:` trigger. `docker-staging.yml:9,20,36–40` and `rust.yml:8–31,57–71` read directly.
9. crates.io API `GET /api/v1/crates/cargo-cyclonedx` → `max_version`/`default_version` `0.5.10`, published `2026-11-06T10:35:19Z`, `rust_version 1.86.0`, not yanked.
10. GitHub API tag refs → `actions/checkout@v4` `11d5960a326750d5838078e36cf38b85af677262`; `actions/upload-artifact@v4` `ea165f8d65b6e75b540449e92b4886f43607fa02`; `taiki-e/install-action@v2` `ab49d3c64680143e184118bed66a46419fc15519`.
11. GitHub API `cyclonedx-cli/releases/latest` → `v0.33.1` (2026-07-23), includes `cyclonedx-linux-x64` asset (80,337,458 B, sha256 digest published).
12. `taiki-e/install-action` `TOOLS.md` → lists both `cargo-cyclonedx` and `cyclonedx` as GitHub-Releases tools (Linux supported), validating `fallback: none` + `checksum: true`.
13. `cargo-cyclonedx` 0.5.10 README (crates.io readme endpoint) → confirms all workflow flags (`--manifest-path`, `--format json`, `--describe crate`, `--all`, `--spec-version 1.5`, `--license-strict`, `--target`, `--override-filename`) and the per-`Cargo.toml` output behaviour (R1).
14. `cyclonedx-cli` README (raw `main`) → confirms `validate --input-file --input-format json --input-version v1_5 --fail-on-errors` (default `--input-version` is v1.7) and the .NET runtime dependency list (R2).
15. `grep` over `docs/` excluding `docs/research/` → no release/SBOM/tag doc to update (§4.4).

---

## 11. Citations

- **[U1]** The Cargo Book, *Workspaces* — `workspace.package` inheritance (incl. `version`), root package, `version.workspace = true`, MSRV 1.64+: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-package-table
- **[U2]** TOML v1.0.0, *Keys* / *Table* — dotted keys define tables; a table cannot be redefined by a header: https://toml.io/en/v1.0.0
- **[U3]** GitHub Docs, *Workflow syntax for GitHub Actions* — `on.push.tags`, `permissions`, `concurrency`: https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
- **[U4]** GitHub CLI manual, `gh release create` — `--verify-tag`, `--notes-from-tag`, `--draft`, non-atomic draft→upload→publish; immutability applies on publication: https://cli.github.com/manual/gh_release_create
- **[U5]** GitHub CLI manual, `gh release upload` — `--clobber` deletes/re-uploads; original lost on failure: https://cli.github.com/manual/gh_release_upload
- **[U6]** GitHub CLI manuals, `gh release verify` / `gh release verify-asset`: https://cli.github.com/manual/gh_release_verify · https://cli.github.com/manual/gh_release_verify-asset
- **[U7]** GitHub Docs, *Immutable releases* — tag/asset protections, automatic release attestation, three-step draft-first best practice: https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases
- **[U8]** GitHub Docs, *Preventing changes to your releases* — per-repo/org enablement; future releases only: https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/prevent-release-changes
- **[U9]** OWASP CycloneDX, `cargo-cyclonedx` 0.5.10 README — flags, per-manifest output, Cargo.lock + `cargo metadata`, build-script trust boundary: https://crates.io/api/v1/crates/cargo-cyclonedx/0.5.10/readme · repo: https://github.com/CycloneDX/cyclonedx-rust-cargo
- **[U10]** OWASP CycloneDX, `cyclonedx-cli` README — `validate` flags, default v1.7, .NET runtime deps: https://github.com/CycloneDX/cyclonedx-cli/blob/main/README.md
- **[U11]** `taiki-e/install-action` README + TOOLS.md — `@version` pinning, `fallback`, checksums, impostor-commit warning: https://github.com/taiki-e/install-action · https://github.com/taiki-e/install-action/blob/main/TOOLS.md
- **[U13]** Git, *git-tag(1)* — annotated vs lightweight; no re-tagging of published tags: https://git-scm.com/docs/git-tag
- **[T1][T3][T4][T5]** SemVer, Cargo SemVer, Conventional Commits, git-tag — inherited from [029] §Citations.
- **[028]** `docs/research/2026-11-16-028-sbom-release-plan.md` — generator flags, NTIA fields, P2 deferral.
- **[029]** `docs/research/2026-11-16-029-release-process-minimal.md` — SemVer/Conventional Commits/tag mechanics, draft-first, carried NV1–NV9.
- **[030]** `docs/research/2026-11-16-030-release-sbom-exact-plan.md` — the design under audit (8 manifests, one tag-gated workflow, validated SBOM, bump policy, G0–G8).
