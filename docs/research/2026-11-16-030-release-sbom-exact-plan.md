# 030 — Exact minimal implementation plan: one workspace version, one tag-gated release workflow, one validated SBOM

**Date:** 2026-11-16  
**Scope:** research + read-only audit only. **No `Cargo.toml`, workflow, config, `.gitignore`, tag, release or SBOM file was created or changed.** The only file produced by this task is this document. All YAML/shell/TOML snippets below are *descriptions of primary docs, verified against the current tree*, not applied Flora state.  
**Question:** turn cycle 029's design (`vX.Y.Z` annotated tag, centralized version, draft-first immutable release, pinned `cargo-cyclonedx 0.5.10`, lockstep assert) into the **exact** minimal edit set: every file and line, a complete release workflow, a version-bump policy, and an explicit go/no-go gate.  
**Method:** primary sources only (Cargo Book, TOML spec, GitHub Docs, GitHub CLI manual, GitHub Actions syntax, CycloneDX README/CLI docs, crates.io API, `taiki-e/install-action` README/TOOLS) plus **read-only local checks** (`cargo metadata`, `git tag -l`, `git status`, scratch-manifest experiments in `/tmp/opencode`). Multilingual cross-check in EN, pt-BR, es, de, fr, ja, zh-CN (§7); English canonical text governs every operative claim. Complements [029] and [028].

**Resumo (pt-BR):** o plano exato é pequeno, mas tem uma armadilha que o 029 não viu: a raiz do `Cargo.toml` **já** usa chaves pontilhadas `package.*` (linhas 19–23), então adicionar um cabeçalho `[workspace.package]` gera `error: duplicate key` — a edição correta é **uma linha** `package.version = "0.1.0"` dentro do bloco existente, mais `version.workspace = true` na linha 9 e nas linhas 8 dos sete crates (8 manifests, 9 linhas). O workflow novo dispara só em `push` de tag `v*`, valida a forma `vX.Y.Z` por regex, usa `cargo metadata` + `jq` para exigir que **todas as 8 versões** sejam iguais à tag, gera um BOM CycloneDX 1.5 JSON com `cargo-cyclonedx 0.5.10`, injeta tag/SHA/versão no `metadata` (com assert), valida com `cyclonedx validate --input-version v1_5 --fail-on-errors`, e só então faz `gh release create --draft` → `upload` (sem `--clobber`) → `edit --draft=false` → `verify`/`verify-asset`. A política de bump mantém `0.MINOR.PATCH` conservador (breaking antes de 1.0 sobe MINOR, não PATCH) e nunca reusa tag publicada. Go/no-go depende sobretudo de três incógnitas: imunidade de release disponível no plano, forma real do JSON gerado (8 manifests podem produzir 8 BOMs) e `--target` coerente com o artefato enviado.

---

## 1. Decision and go/no-go

**Decision (unchanged from 029, now operationalized):** adopt the manual-by-default, automation-light process — one annotated product tag `vX.Y.Z`, one workspace version source, immutable release enabled **before the first release**, draft-first asset attachment, `cargo-cyclonedx` + schema validation as a tag-gated job, plain `gh release`. Defer CycloneDX-predicate attestation / image SBOM (P2) and `release-plz` (P3). This cycle adds **exact edit points, a fail-closed workflow, a bump policy, and gate criteria**.

| Gate | Go criterion | No-go if | Evidence / owner |
|---|---|---|---|
| **G0 — platform** | Release immutability is available on the repo/plan and enabled in Settings → Releases *before* the first release | Setting absent or gated by plan → fall back to P0 artifact-only path (029 step 7) and keep release manual | 029 NV3 [U7][U8] |
| **G1 — manifests** | `cargo metadata --no-deps` reports exactly **8** `flora*`/`flora-*` packages, all at the same `X.Y.Z`, equal to the tag without `v` | Any of the 8 still hard-codes a different version; `cargo metadata` fails | verified command in §3.4; assert in workflow step 5 |
| **G2 — tag** | Annotated tag `vX.Y.Z` created once on the intended commit, pushed once, not moved | Lightweight tag, force-move, reuse of a published name | [U13], [U4] |
| **G3 — tool pins** | `taiki-e/install-action` resolves `cargo-cyclonedx@0.5.10` and `cyclonedx@0.33.1` with `fallback: none` and checksums on | Any tool pin unresolvable on the runner | [U11]; versions confirmed via crates.io/GitHub API (§9) |
| **G4 — dry run** | Generator emits **exactly one** BOM at workspace root; `cyclonedx validate --input-version v1_5 --fail-on-errors` passes; `metadata.component.version` exists | Per-member BOMs, schema errors, missing `metadata.component` | new NV10/NV11; fail-closed steps in workflow |
| **G5 — identity** | Post-processed BOM has `component.version == ${TAG#v}`, `flora:release:tag == $GITHUB_REF_NAME`, `flora:release:commit == $GITHUB_SHA` | jq assert fails | 029 NV1/NV2 |
| **G6 — target** | `--target` equals the actually shipped artifact; current staging image is `aarch64-unknown-linux-gnu` | Release binary decided different, or target flag rejected by generator | 029 NV5; `docker-staging.yml:37` |
| **G7 — publish path** | `gh` authenticated via `GITHUB_TOKEN` with `contents: write`; `--verify-tag` succeeds; no pre-existing release/draft for the tag | `gh` fails at any stage; existing release would make the step non-idempotent | [U4][U6]; new NV17 |
| **G8 — trust boundary** | Workflow triggered only by tag push (never fork PR); no `--clobber`; no asset edits after publish | Any PR-triggered generation or post-publication upload | [U5][U9] |

**Decision rule:** GO only when G0–G8 all pass. If G0 or G4 fails but G1–G3 hold, ship the **artifact-only** subset (generate → validate → upload workflow artifact, stop before `gh release create`) — a net gain that needs no release at all (029 P0 step 7). NO-GO for any attempt to tag with skew between manifest and tag.

---

## 2. Verified Flora ground truth (2026-11-16, read-only)

| Item | Verified state | Evidence |
|---|---|---|
| Git tags | **Zero** (`git tag -l` empty) | local command |
| Working tree | Clean (`git status --porcelain` empty) before this document | local command |
| Workspace members | 7: `flora-core`, `flora-api`, `flora-messaging`, `flora-tasks`, `flora-files`, `flora-search`, `flora-tests`; plus root `[package] flora` = 8 packages | root `Cargo.toml:25–33`; `cargo metadata --no-deps` |
| Versions | All 8 = `0.1.0`; **no** inheritance | `cargo metadata --no-deps --format-version 1` → `[{"flora-core",0.1.0},…,{"flora",0.1.0}]` |
| Root version key style | `[package] version = "0.1.0"` at line 9; `[workspace]` uses **dotted** `package.rust-version`, `package.edition`, `package.license`, `package.authors`, `package.repository` at lines 19–23 — not a `[workspace.package]` table | root `Cargo.toml:7–23` |
| Workflows | Only `ci.yml`, `rust.yml`, `docker-staging.yml`, `osv-scan.yml`; no tag/release trigger | `.github/workflows/` |
| Toolchain in CI | `1.98.0` installed via rustup block; strict clippy, cargo-deny via `taiki-e/install-action@cargo-deny` | `rust.yml:8–37, 89–97` |
| Staging artifact | GHCR image `linux/arm64`, tags `staging` + `${{ github.sha }}`; path filter includes `Cargo.toml` | `docker-staging.yml:3–11, 32–40` |
| Local cargo used for checks | `cargo 1.99.1 (5f94df478 2026-08-27)` — newer than CI's 1.98.0; inheritance behaviour is MSRV 1.64+ and unchanged | [U1] |

**Out of scope:** `apps/web/tauri/**/Cargo.toml` (11 manifests) belong to the imported `@coparse/web` SolidJS/Tauri app, not to the Flora workspace (per `AGENTS.md`). **Do not edit them.** The release SBOM covers the 8 workspace crates only (029 Finding 2).

---

## 3. Exact file edits

### 3.1 Root `Cargo.toml` — two edits

**Edit A — make the root package inherit (current line 9):**

```toml
[package]
name = "flora"
version.workspace = true          # was: version = "0.1.0"
description = "Flora Workspace - Self-hostable corporate collaboration platform"
```

**Edit B — add the single source of truth inside the existing dotted-key block (insert as new line 19, before `package.rust-version`):**

```toml
[workspace]
resolver = "3"
package.version = "0.1.0"         # NEW — single source; bump this one line
package.rust-version = "1.98"
package.edition = "2024"
package.license = "AGPL-3.0-or-later"
package.authors = ["Flora Workspace  - MDMV Laboratório de Soluções Digitais"]
package.repository = "https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/MDMV"
```

**Why not `[workspace.package]`:** the dotted keys above already define the `workspace.package` table. TOML forbids redefining a table via header: adding `[workspace.package] version = "0.1.0"` yields `error: duplicate key` (verified locally with a scratch manifest, §9). The 029/028 prose example showing `[workspace.package]` is illustrative; the minimal valid edit is the dotted line. (A cosmetic refactor converting all five dotted keys into a `[workspace.package]` table is optional, larger, and not required.)

**Why the root may inherit from its own workspace:** verified with a scratch manifest containing both `[package] version.workspace = true` and `[workspace] package.version = "0.2.0"` — `cargo metadata` returns the root package at `0.2.0` (§9). [U1]

### 3.2 The seven member manifests — one edit each

Identical replacement in all seven; current text on **line 8** of each file:

```toml
version = "0.1.0"
```
→
```toml
version.workspace = true
```

| # | File | Line | Current | Replacement |
|---|---|---|---|---|
| 1 | `flora-core/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |
| 2 | `flora-api/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |
| 3 | `flora-messaging/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |
| 4 | `flora-tasks/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |
| 5 | `flora-files/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |
| 6 | `flora-search/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |
| 7 | `flora-tests/Cargo.toml` | 8 | `version = "0.1.0"` | `version.workspace = true` |

All seven already inherit `repository/license/authors/rust-version/edition` via `.workspace = true`, so this is consistent with existing style. **Total manifest diff: 8 files, 9 lines** (8 replacements + 1 insertion). No `Cargo.lock` change is expected from version inheritance; verify with `cargo metadata --locked` during implementation (NV18).

### 3.3 New file `.github/workflows/release.yml`

Complete, fail-closed workflow. Pinned action SHAs were resolved from the GitHub API on 2026-11-16 (§9) — re-verify at implementation time (NV12).

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

**Ordering rationale (primary-doc grounded):** `gh release create` with file arguments performs draft → upload → publish as **separate, non-atomic API requests** and immutability takes effect only on publication [U4]; assets cannot be modified/deleted and the tag cannot be moved afterwards [U7]; `--clobber` deletes before re-upload and loses the original on failure [U5]. Therefore the BOM is generated, identity-asserted and schema-validated *before* any release object exists; create is called **without file arguments** and with `--draft`; assets are attached once; publish is last. `--notes-from-tag` requires an annotated tag, and `--verify-tag` aborts if the tag is not on the remote [U4]. `concurrency` uses the tag ref so a re-push cannot race a publish; the checkout uses `fetch-depth: 0` so the annotated tag object and notes are available (NV19).

**Optional control (not in the minimal diff):** put `environment: release-approval` on the job, or split publish into a second job gated on a manual approval, if the team wants a human GO click before `gh release edit --draft=false`. This does not change the ordering.

### 3.4 One-time operations (not file edits)

1. Merge the manifest edits and `release.yml`. Note that changing root `Cargo.toml` triggers a staging image rebuild on `dev` (path filter `Cargo.toml` in `docker-staging.yml:9`) — expected cost, §5.
2. Enable immutability: repo Settings → Releases → *Enable release immutability* (or org policy). It applies to **future releases only** [U8] — do this before the first tag.
3. Pre-flight locally (evidence before tagging):
   ```bash
   cargo metadata --no-deps --format-version 1 \
     | jq -e --arg v 0.1.0 '([.packages[] | select(.name=="flora" or (.name|startswith("flora-")))] | length == 8) and ([.packages[] | select((.name=="flora" or (.name|startswith("flora-"))) and .version != $v)] | length == 0)'
   ```
   (This exact expression was executed against the current tree and passed for `0.1.0`, failed for `9.9.9` — §9.)
4. Bump the one root line, commit on `dev`, push the commit, then create/push the annotated tag (never `-f`):
   ```bash
   git tag -a v0.1.0 -m "Flora v0.1.0 — Seed / FASE 0 closure"
   git push origin v0.1.0
   ```
   Push the *commit* first so the tag points to something on the remote; `--verify-tag` will then pass [U4][U13].
5. Watch the tag job; if it fails before `gh release create`, fix and delete any draft manually before re-running (step "Refuse to publish over an existing release/draft" fails closed by design — NV17).

**No changes** to `rust.yml`, `ci.yml`, `osv-scan.yml`, `docker-staging.yml`, `LICENSE`, `NOTICE`, `.gitignore`. The generated BOM lives only in the runner workspace and in workflow artifacts/release assets — never commit it.

---

## 4. Version-bump policy

### 4.1 The contract

- `workspace.package.version` (root `Cargo.toml`, inserted line 19) is the **only** number. The tag is `v` + that number. Bare `X.Y.Z` goes in manifests and in the BOM; `v` is a tag convention, not SemVer [029 T1].
- **Lockstep is enforced mechanically**: all 8 crates inherit; the workflow's `cargo metadata` + `jq` step fails if any package differs from `${GITHUB_REF_NAME#v}`. No per-crate versions may ever be reintroduced.
- One **product tag per release**; no seven per-crate tags (029 Finding 2).
- Released tags are immutable: never move, delete-and-reuse, or force-push. A correction is a **new** version. GitHub also forbids reusing a tag name even after deleting an immutable release [U7].

### 4.2 Bump map while version is `< 1.0.0`

| Commits since last tag | Bump | `0.1.0` → | Rationale |
|---|---|---|---|
| `fix:` only | PATCH | `0.1.1` | Conventional Commits → SemVer [029 T4] |
| `feat:` (backward-compatible) | MINOR | `0.2.0` | [029 T4] |
| `BREAKING CHANGE:` footer or `type!:` — **while < 1.0.0** | MINOR + say "breaking" in release notes | `0.2.0` | Cargo/SemVer: in `0.y.z`, `y` is the left-most non-zero component, so `y` carries incompatible changes; the public API is not stable yet [029 T1][029 T3] |
| `docs:` / `chore:` / `test:` / `ci:` / `refactor:` with no user-visible change | **No automatic bump**; explicit human decision. If a release is wanted anyway, PATCH | `0.1.1` | These types "have no implicit effect in Semantic Versioning" [029 T4] |
| First declaration of a stable public API | MAJOR | `1.0.0` | SemVer §5 [029 T1] |
| Release candidate | **Not in minimal flow** | — | Would need `--prerelease` and `-rc.N` handling; deferred (NV20) |

Caveat (029 §3.1): Flora is primarily an application, so Cargo's library API compatibility rules are advisory; still, treat `0.x` bumps conservatively and explain them in the annotated tag message / release notes rather than pretending a stable API exists.

### 4.3 Bump procedure (exact)

1. Edit **root `Cargo.toml` line 19 only**: `package.version = "0.1.0"` → `package.version = "0.1.1"`.
2. Run the §3.4 pre-flight assert locally; commit `chore(release): v0.1.1`; push to `dev`.
3. After the commit is on the remote, `git tag -a v0.1.1 -m "…"` and `git push origin v0.1.1`.
4. Let the workflow generate/validate/publish. Humans choose the bump; automation only enforces consistency (no `release-plz` bump detection in this cycle — 029 P3).

---

## 5. CI cost and security notes

**Cost**
- Trigger is `push` of tags only: **zero extra CI cost** for ordinary pushes/PRs. One job per release (a few minutes), using prebuilt tool binaries — no `cargo install` from source.
- `fetch-depth: 0` makes checkout fuller/slower than the repo's usual depth-1; alternative is default depth plus `git fetch --tags origin` (NV19).
- Manifest edit will cause one extra `aarch64` staging image build (path filter in `docker-staging.yml:9`).
- Release asset storage: one JSON SBOM (typically tens–hundreds of KB) + tiny `checksums.txt` per release. No new paid GitHub features are strictly required **pending G0/NV3**.
- No new workflow for PRs; no extra matrix; `concurrency` prevents duplicate release runs.

**Security**
- Least privilege: top-level `contents: read`; the release job alone gets `contents: write`. No `attestations: write` needed — immutability generates the release attestation automatically [U7].
- Action pins are full commit SHAs resolved from the GitHub API on 2026-11-16 (§9). `taiki-e` explicitly warns that pinning its `@tool` shorthand by hash causes an "impostor commit" reference [U11]; therefore pin the **action** (`@v2` SHA) and pin **tool versions** via the `tool:` input — never pin `@cargo-cyclonedx` by hash.
- `fallback: none` + `checksum: true`: both tools are in `install-action`'s manifest (GitHub Releases, SHA256-verified) [U11]; this also avoids passing `github.token` to `cargo-binstall` and its API-rate-limit path.
- The generator invokes Cargo and may execute build scripts; run it only on this trusted, tagged first-party tree — never on fork PRs [U9][029 T16].
- No `--clobber`, no post-publication uploads/asset edits, no force-moved tags [U4][U5][U7][U13].
- `checksums.txt` is hygiene only; the authoritative integrity binding is the automatic release attestation, verified with `gh release verify` / `gh release verify-asset` [U6][U7]. It is **not** a CycloneDX-predicate attestation (029 P2 remains deferred).
- Self-hosting portability: the tag/version layer is pure git; `gh release` and `runs-on` are forge-specific. Migrating to Gitea/GitLab later needs a release-step swap (the 029 P3 `release-plz` trigger), not a manifest redesign.

---

## 6. Go/no-go checklist (operator-facing)

Before the first tag: ☐ G0 immutability enabled · ☐ §3.1–3.2 edits applied (9 lines) · ☐ `cargo metadata` assert passes · ☐ `release.yml` merged with pins re-verified · ☐ dry run produced exactly one BOM and `cyclonedx validate` passed · ☐ `--target` confirmed · ☐ no draft/release exists for the tag · ☐ tag annotated and pushed once.

At the tag: ☐ job green through "Upload BOM for inspection" → **artifact GO**; ☐ job green through "Verify release attestation and asset digest" → **release GO**.

Stop/rollback rules: a failure *before* `gh release edit --draft=false` can be fixed in a draft (or by deleting the draft and re-running); a failure *after* publication cannot be repaired by replacing assets — issue a new version instead [U5][U7]. If any G-gate is red, do not tag; use the artifact-only fallback.

---

## 7. Multilingual reliability cross-check

Primary pages were consulted in multiple languages as a consistency check only; EN canonical text governs. Method continues [029] §2.

| Language | Page | Observation |
|---|---|---|
| EN | Cargo Book §Workspaces [U1]; TOML 1.0 [U2]; GitHub immutable releases [U7], preventing changes [U8], `gh` manuals [U4][U5][U6]; CycloneDX READMEs [U9][U10]; install-action [U11] | Canonical. Confirms version inheritance, table-redefinition rule, draft-first, non-atomic asset upload, `--verify-tag`, `--clobber` loss, `verify-asset` syntax, `--input-version` default v1.7. |
| pt-BR | TOML v1.0.0 [U2pt] | Substance intact: "Como as tabelas não podem ser definidas mais de uma vez, não é permitido redefinir tais tabelas usando um cabeçalho `[tabela]`" — matches EN and the local `duplicate key` experiment. |
| es / de / fr | Immutable releases (consulted in 029) | Substance stable; terminology churn (*versiones inmutables / releases*, *Releases / Versionen*, *ressources / artéfacts*). No operative divergence. |
| ja | *リリースに対する変更の防止* [U8ja] | "リリース不変性を有効にする"; "不変性は将来のリリースにのみ適用される" — future-only scope intact; UI strings partly English. |
| zh-CN | *阻止对版本进行更改* [U8zh] | "Enable release immutability" and future-only note intact; same stray `****` Markdown artifact around bolded UI strings as [029] §2 — translation-rendering noise, not substance. |
| pt-BR (404s) | Cargo Book `pt-BR/.../workspaces.html`; GitHub Docs `pt-br/.../immutable-releases` and `.../prevent-release-changes` | All 404 on 2026-11-16. Cargo Book `es/.../workspaces.html` also 404. Do not cite translated Cargo/GitHub pages for the new load-bearing claims until slugs are re-checked. (NV16) |

**Reliability rule:** agreement across translations is a consistency check, never the citation. The draft-first rule and future-only immutability survive all available translations; the version/table mechanics are cited from EN TOML/Cargo docs and local verification.

---

## 8. Risks / NVs (not verified in this research)

Carried from [029] (NV1–NV9) unless sharpened below:

- **NV1** (carried) — `cargo-cyclonedx` has no root name/version override; post-processing shape and schema interaction not executed. The dry-run gate G4 is mandatory.
- **NV2** (carried) — `metadata.properties` placement/names against the 1.5 schema not validated on a real generated file. The workflow's jq guard fails closed if `metadata.component.version` is absent (new NV11).
- **NV3** (carried, blocking) — immutable-release availability on Flora's plan/visibility not stated on the EN pages; verify in Settings before G0.
- **NV4** (carried) — default branch is `dev` locally/in `docker-staging.yml`; no remote default-branch check was performed. The tag trigger is branch-independent, but push the version commit to the real default branch.
- **NV5** (carried) — `--target aarch64-unknown-linux-gnu` chosen from the current staging image; no release-artifact decision exists. If a release binary replaces the image, change this one value (G6).
- **NV6** (carried) — pt-BR GitHub Docs 404s; extended here to the *prevent-release-changes* how-to and to Cargo Book pt-BR/es (NV16).
- **NV7** (carried) — `release-plz` config untrialled; deferred P3.
- **NV8** (partially addressed) — no measured CI minutes; §5 gives qualitative sizing only. Release asset storage and any plan gating remain unsized.
- **NV9** (carried) — research only; no repository file changed.
- **NV10 (new)** — with 8 manifests, `cargo-cyclonedx --override-filename` may emit one BOM per `Cargo.toml` (README says default output is adjacent to every manifest) rather than a single root file. The workflow asserts exactly one file at root and fails otherwise; the dry run must confirm actual behaviour before first release.
- **NV11 (new)** — the exact `metadata.component` shape produced by `cargo-cyclonedx 0.5.10` for a workspace with `--describe crate` was not observed; the `jq -e '.metadata.component.version? // error(...)'` guard is deliberately fail-closed, not assumed correct.
- **NV12 (new)** — action SHAs in §3.3 were resolved from the GitHub API on 2026-11-16 (`checkout@v4` → `11d5960…`, `upload-artifact@v4` → `ea165f8…`, `install-action@v2`/`v2.90.5` → `ab49d3c…`). Tags are mutable aliases; re-resolve and re-verify before applying.
- **NV13 (new)** — `cyclonedx` CLI 0.33.1 is a .NET tool with runtime library dependencies; its successful execution on `ubuntu-latest` was not trialled (G3).
- **NV14 (new)** — whether `cargo cyclonedx --target aarch64-unknown-linux-gnu` requires an installed target toolchain/std was not tested; dry run required (G4/G6).
- **NV15 (new)** — `sha256sum` output embeds the filename; it is hygiene, not a portable digest record. The release attestation is authoritative.
- **NV16 (new)** — translated Cargo Book (pt-BR, es) and pt-BR GitHub Docs pages 404 (§7).
- **NV17 (new)** — draft/publish steps are not idempotent: `gh release create` against an existing tag fails. The workflow fails closed before create; recovery is a manual draft delete, not `--clobber`.
- **NV18 (new)** — `Cargo.lock` change from version inheritance not measured; check with `cargo metadata --locked` / `git diff` during implementation.
- **NV19 (new)** — `fetch-depth: 0` chosen conservatively for annotated-tag notes; whether depth-1 checkout already surfaces `--notes-from-tag` was not tested.
- **NV20 (new)** — prerelease (`-rc.N`, `--prerelease`) policy intentionally omitted; define before using release candidates.
- **NV21 (new)** — translation noise (zh-CN `****`) and EN/pt-BR/es/de/fr terminology drift were observed but not used as citations; only EN canonical text is load-bearing.

---

## 9. Evidence log (commands actually executed, read-only)

1. `cargo --version` → `cargo 1.99.1 (5f94df478 2026-08-27)`; `git tag -l` → empty; `git status --porcelain` → empty.
2. `cargo metadata --no-deps --format-version 1 | jq -c '[.packages[]|{name,version}]'` → 8 packages: `flora-core`, `flora-api`, `flora-messaging`, `flora-search`, `flora-tasks`, `flora-files`, `flora-tests`, `flora` — all `0.1.0`.
3. Scratch manifest in `/tmp/opencode/flora-c030-check`: `[workspace] package.version = "0.2.0"` + member `version.workspace = true` → member resolves to `0.2.0`. A second scratch manifest with both `[package] version.workspace = true` and `[workspace] package.version = "0.2.0"` → root package resolves to `0.2.0`.
4. Appending `[workspace.package] version = "0.2.0"` to a manifest already containing `package.version = "0.2.0"` → `error: duplicate key` at the header. This is why §3.1 uses a dotted line.
5. Lockstep jq expression from §3.4 run against real metadata: PASS for `EXPECTED=0.1.0`, correctly FAIL for `EXPECTED=9.9.9`. The identity-assert jq expression from §3.3 was also tested on a sample BOM: accepts matching tag/SHA, rejects a mismatched SHA.
6. crates.io API: `cargo-cyclonedx` max/default version `0.5.10`, published 2026-11-06, `rust_version = 1.86.0`, not yanked (compatible with the repo's 1.98). `cyclonedx-cli` latest release `v0.33.1` (2026-07-23, GitHub API).
7. `taiki-e/install-action` TOOLS.md lists both `cargo-cyclonedx` and `cyclonedx` as GitHub-Releases tools; README documents `@version` pinning, `fallback: none`, checksums, and the impostor-commit warning for hash-pinned `@tool` shorthands.
8. GitHub API tag refs: `actions/checkout@v4` → `11d5960a326750d5838078e36cf38b85af677262`; `actions/upload-artifact@v4` → `ea165f8d65b6e75b540449e92b4886f43607fa02`; `taiki-e/install-action@v2` (and `v2.90.5`) → `ab49d3c64680143e184118bed66a46419fc15519`.

---

## 10. Citations

- **[U1]** The Cargo Book, *Workspaces* — `workspace.package` inheritance (`version` supported), root package, `version.workspace = true`, MSRV 1.64+: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-package-table
- **[U2]** TOML v1.0.0, *Keys* / *Table* — dotted keys define tables; a table cannot be redefined by a header: https://toml.io/en/v1.0.0 · **[U2pt]** pt-BR: https://toml.io/pt/v1.0.0
- **[U3]** GitHub Docs, *Workflow syntax for GitHub Actions* — `on.push.tags` glob filter, `permissions`, `concurrency`: https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
- **[U4]** GitHub CLI manual, `gh release create` — auto-tag vs `--verify-tag`, `--notes-from-tag`, `--draft`, non-atomic draft→upload→publish, immutability applies on publication: https://cli.github.com/manual/gh_release_create
- **[U5]** GitHub CLI manual, `gh release upload` — `--clobber` deletes/re-uploads, original lost on failure: https://cli.github.com/manual/gh_release_upload
- **[U6]** GitHub CLI manuals, `gh release verify` / `gh release verify-asset` (`[<tag>] <file-path>`): https://cli.github.com/manual/gh_release_verify · https://cli.github.com/manual/gh_release_verify-asset
- **[U7]** GitHub Docs, *Immutable releases* — tag/asset protections, automatic release attestation (tag + commit SHA + assets), three-step draft-first best practice: https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases
- **[U8]** GitHub Docs, *Preventing changes to your releases* — enable per repo or org; future releases only · **[U8ja]** https://docs.github.com/ja/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/prevent-release-changes · **[U8zh]** https://docs.github.com/zh-cn/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/prevent-release-changes
- **[U9]** OWASP CycloneDX, `cargo-cyclonedx` README — CLI flags (no version override), per-manifest output, Cargo.lock + `cargo metadata`, build-script trust boundary: https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/main/cargo-cyclonedx/README.md · crate metadata: https://crates.io/api/v1/crates/cargo-cyclonedx
- **[U10]** OWASP CycloneDX, `cyclonedx-cli` README — `validate --input-file --input-format --input-version (default v1_7) --fail-on-errors`: https://github.com/CycloneDX/cyclonedx-cli/blob/main/README.md
- **[U11]** `taiki-e/install-action` README + TOOLS.md — `@version` tool pinning, `fallback`, checksums, impostor-commit warning: https://github.com/taiki-e/install-action · https://github.com/taiki-e/install-action/blob/main/TOOLS.md
- **[U13]** Git, *git-tag(1)* — annotated vs lightweight, no re-tagging of published tags: https://git-scm.com/docs/git-tag
- **[T1][T3][T4][T5][T17]** Inherited from [029] (SemVer, Cargo SemVer, Conventional Commits, git-tag, prior SBOM research). Full citation list: `docs/research/2026-11-16-029-release-process-minimal.md`.
- **[028]** `docs/research/2026-11-16-028-sbom-release-plan.md` — generator flags, NTIA fields, P2 deferral.
