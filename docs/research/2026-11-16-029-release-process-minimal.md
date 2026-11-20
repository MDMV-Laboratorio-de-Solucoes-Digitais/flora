> **Superseded by PR #238/#239 (cycle 029):** this is a pre-implementation snapshot; check committed files for current state.

# 029 — Minimal, self-hosted-friendly release/tag process for Flora (unblocks the CycloneDX SBOM)

**Date:** 2026-11-16  
**Scope:** read-only research + read-only Flora cross-check. No workflow, config, `Cargo.toml`, `LICENSE`, `.gitignore`, or SBOM file was created or changed. The only file produced by this task is this research document.  
**Question:** what is the *minimal*, self-hosted-friendly release/tag process Flora must adopt so that cycle 028's CycloneDX SBOM plan stops being blocked by "there is no tag and no release workflow"? Specifically: semver/conventional tagging for a Rust workspace; draft-first GitHub Release with immutable assets; `release-plz` versus plain `gh release`; and a CycloneDX SBOM generated, validated and bound to the tag.  
**Method:** primary sources only for load-bearing claims — semver.org, the Cargo Book, Conventional Commits, git-scm, GitHub Docs and the GitHub CLI manual, release-plz docs, the CycloneDX Rust plugin README. Sources were consulted in **EN, ES, DE, FR, JA and zh-CN**, with a **pt-BR** lookup attempt (§2); English canonical text governs every operative claim. Complements `2026-11-16-028-sbom-release-plan.md` [T17], which specified *how* to build the SBOM but stopped at "define a tag trigger"; and `.specify/releases/028-fase0-closure/release.md`, which lists "release SBOM (needs tag process)" as this cycle's follow-up.

**Resumo (pt-BR):** o bloqueio não é o SBOM — é a ausência de release. O Flora não tem nenhuma tag (`git tag -l` vazio), nenhum workflow de release e sete crates todos em `0.1.0` sem uma fonte única de versão. O plano mínimo é: **uma tag de produto por release**, anotada, no formato `vX.Y.Z` (o `v` é convenção de tag, não parte do semver); **uma única fonte de versão** via `[workspace.package]` (proposta, não aplicada); **GitHub Release imutável publicada draft-first**, com o SBOM gerado e validado *antes* de anexar; e **`gh release` direto**, sem `release-plz`, que fica como caminho de upgrade documentado. Dois pontos novo e delicados: (a) o `cargo-cyclonedx` não tem flag para trocar a versão do componente raiz — ele a lê do `Cargo.toml` — então uma tag `v0.2.0` sobre manifests `0.1.0` gera um BOM que **não** corresponde à tag; (b) releases imutáveis geram **automaticamente** uma attestation contendo tag, commit SHA e digests dos assets, o que resolve amarração de integridade sem `actions/attest` — mas isso é distinto da attestation com predicado CycloneDX, que permanece adiada (P2, herdada de 028).

---

## 1. Summary / decision

**Finding 1 — the blocker is a missing versioning contract, not a missing tool.** `git tag -l` returns zero tags; `.github/workflows/` contains only `ci.yml`, `rust.yml`, `docker-staging.yml` and `osv-scan.yml`, none with a `push: tags:` or `release:` trigger. The root `[package]` is `0.1.0` and all seven members independently hard-code `version = "0.1.0"`; the workspace does **not** use `[workspace.package] version` + `version.workspace = true`. So there is no single value that a tag could even refer to. SemVer itself requires that "once a versioned package has been released, the contents of that version MUST NOT be modified" [T1] — a rule GitHub enforces for releases via immutability [T7].

**Finding 2 — one product tag, not seven per-crate tags.** Cargo workspaces may inherit a single version from `[workspace.package]` [T2]. Flora is a self-hostable *application* (one shipped API/container), not a set of crates published to crates.io; the SBOM is likewise produced once for the workspace [T17]. Therefore the release tag should be a single **product tag** `vX.Y.Z` on `dev`/the release commit. The alternative — release-plz's default multi-package pattern `{{ package }}-v{{ version }}` [T14] — would mint seven tags and seven releases for one deployment, none of which is "the Flora release" the SBOM belongs to.

**Finding 3 — the `v` prefix is a tag convention, not SemVer.** SemVer's own FAQ: "v1.2.3 is not a semantic version … prefixing a semantic version with a `v` is a common way … to indicate it is a version number"; `git tag v1.2.3 -m "Release version 1.2.3"` is the canonical example [T1]. Use tag `vX.Y.Z`, put bare `X.Y.Z` in `Cargo.toml` and in the BOM. Never parse the leading `v` into the version field.

**Finding 4 — draft-first is not optional once immutability is on.** GitHub's stated best practice is: create as draft → attach all assets → publish [T7]; the same tip appears in *Managing releases* [T6]. `gh release create` with file arguments actually performs draft → upload each asset → publish, but the manual is explicit that these are **separate API requests and not a single atomic operation**, and that immutability protections take effect only on publication [T10]. Consequence: generate and validate the SBOM **before** creating the release, then create with `--draft`, upload, and publish with `gh release edit --draft=false`.

**Finding 5 — plain `gh release` wins on minimality; `release-plz` is a documented deferral, not a rejection.** `release-plz` is mature, open-source and supports GitHub/Gitea/GitLab [T12][T14], so it is *compatible* with self-hosting. But it is built around a bot-maintained release PR, version/changelog rewriting, registry version detection (unless `git_only = true`), extra workflow permissions (`pull-requests: write`, `contents: write`), a fork guard, and full-history checkout [T13][T14]. All of that solves problems Flora does not have yet (crates.io publishing, per-crate changelogs, API-breaking detection for library consumers). For "one tag → one release → one SBOM asset" the smaller auditable surface is `git tag` + `gh release`. Revisit triggers are in §5 P3.

**Finding 6 — new: tag↔BOM version skew is the sharpest technical risk.** The `cargo-cyclonedx` CLI exposes `--manifest-path`, `--format`, `--describe`, `--target`, `--all/--top-level`, `--override-filename`, `--target-in-filename`, `--license-strict`, `--license-accept-named`, `--spec-version` and feature flags — **there is no flag to override the root component's name or version** [T16]. The generator takes those from the manifest. If the tag says `v0.2.0` but `Cargo.toml` still says `0.1.0`, the BOM says `0.1.0` and the "SBOM bound to the tag" claim is false. Two resolutions exist (§5 P0 steps 2 and 5): bump versions in lockstep before tagging, or deterministically post-process the JSON and re-validate. Lockstep bump is preferred; post-processing is the minimal escape hatch.

**Finding 7 — new: immutable releases give free integrity binding.** Creating an immutable release automatically generates a **release attestation** — "a cryptographically verifiable record … containing the release tag, the commit SHA, and the release assets" [T7]. Consumers verify with `gh release verify TAG` and `gh release verify-asset TAG PATH` [T9]. That binds the SBOM *file* to the tag and commit at the digest level without adding `actions/attest`. It does **not** produce a CycloneDX-predicate statement (`https://cyclonedx.org/bom`), so 028's P2 deferral [T17] stands unchanged; this just narrows what is left to do.

**Decision — adopt a manual-by-default, automation-light process:** single annotated `vX.Y.Z` product tag; single workspace version source; immutable release enabled **before the first release** (nothing legacy to migrate); draft-first asset attachment; `cargo-cyclonedx` JSON + schema validation as a tag-gated job; plain `gh release`. **Defer** `release-plz` (P3) and CycloneDX-predicate attestation / image-SBOM binding (P2, inherited from 028). No file in Flora was changed.

---

## 2. Multilingual reliability cross-check

Primary GitHub Docs pages were read in six languages to test whether the new load-bearing items (immutability protections, automatic release attestation, draft-first best practice) survive translation. Method follows [T17] §2. Result: **substance is stable, terminology is not** — only EN canonical text is cited.

| Lang | Page consulted | Observation |
|---|---|---|
| EN | Immutable releases [T7] | Canonical. "release attestation", "release assets", three-step draft-first workflow. |
| ES | Versiones inmutables [T18a] | Substance intact. Terminology churn: *versiones inmutables / versiones / lanzamientos* on one page; `release` left untranslated in one bullet. Automatic *atestación de versión* present. |
| FR | Versions immuables [T18c] | Substance intact. Drift: *ressources* vs *ressources de mise en production* vs *artéfacts*; three-step workflow complete. |
| DE | Unveränderliche Releases [T18b] | Substance intact but inconsistent heading vocabulary (*Releases* / *Versionen*); protections and *Releasenachweis* fully rendered. |
| JA | 変更不可リリース [T18d] | Substance intact. Mixed scripts (変更不可 / 不変); one UI badge string left as `Immutable` in English; three steps correct. |
| zh-CN | 不可变版本 [T18e] | Substance intact. Stray `****` after "加密可验证发布记录" (same class of artifact as 028's zh-CN finding); three steps correct. |
| pt-BR | Immutable releases | **404** at `docs.github.com/pt-br/code-security/concepts/supply-chain-security/immutable-releases` on 2026-11-16. Same result as [T17] NV7 for *Managing releases*. Do not cite a pt-BR translation until the slug is re-checked. (NV6) |

**Reliability rule:** agreement across translations is a consistency check, never the citation. The three load-bearing items are present and correct in all five translations, so [T6][T7] are safe to cite.

---

## 3. Primary-source findings

### 3.1 SemVer and a Rust workspace

From the SemVer 2.0.0 specification [T1]:

- Normal versions are `X.Y.Z` with no leading zeroes; patch/minor/major increment according to fix / backward-compatible feature / incompatible change (§6–8).
- §3: released content MUST NOT be modified; §4: `0.y.z` is initial development and "anything MAY change at any time", the API SHOULD NOT be considered stable; §5: `1.0.0` defines the public API.
- Pre-release (`-rc.1`) and build metadata (`+sha`) are legal but build metadata is ignored for precedence (§9–11). The recommended regex accepts bare `X.Y.Z`, not a `v` prefix.
- FAQ: "start your initial development release at `0.1.0` and then increment the minor version for each subsequent release".

Cargo narrows this for Rust libraries [T3]: "only changes in the left-most non-zero component are considered incompatible" — in `0.y.z`, changes in `y` are major and `z` is minor; `0.0.z` is always major. Flora is primarily an application, so [T3]'s *Application compatibility* guidance applies rather than the library API rules, but the `0.x` convention still affects how a bump is *explained* in release notes. **Recommendation:** while version `< 1.0.0`, treat `0.MINOR.PATCH` bumps conservatively (a breaking deployment/API change bumps `MINOR`, not just `PATCH`) and say so in the release notes; do not pretend a stable public API exists.

Cargo workspaces [T2] allow a single source of version truth:

```toml
[workspace.package]
version = "0.1.0"          # one place to bump
# members:
[package]
version.workspace = true
```

Flora does **not** use this today (all eight manifests carry their own `0.1.0`). Introducing it is a small, isolated change and is the cleanest way to make "the tag version" and "the manifest version" the same value. This is a proposal only — not applied.

### 3.2 Conventional Commits as a bump convention

Conventional Commits 1.0.0 [T4] gives an explicit SemVer mapping: `fix:` → PATCH, `feat:` → MINOR, `BREAKING CHANGE:` footer or `!` after type/scope → MAJOR. Flora's history already uses this vocabulary (`feat(028)`, `docs(028)`, `chore(supply-chain)`, …), so the convention is culturally in place. But the spec also says types other than `feat`/`fix` "have no implicit effect in Semantic Versioning" [T4] — a release made mostly of `docs:` and `chore:` commits still needs an explicit bump decision. **Recommendation:** use Conventional Commits to *inform* the bump, keep the final `vX.Y.Z` a deliberate human decision on the tag, and do not yet automate bump detection (that is precisely the part `release-plz` would take over — §3.5).

### 3.3 Tag mechanics (annotated, push once, never re-tag)

The git-scm `git-tag` manual [T5] is explicit:

- `git tag -a <tag> -m <msg>` creates an **annotated** tag object (tagger, date, message, optional signature); a lightweight tag is "simply a name for an object". "Annotated tags are meant for release while lightweight tags are meant for private or temporary object labels." `git describe` ignores lightweight tags by default.
- `-s`/`-u` create cryptographically signed tags; useful later, not required for the minimal process.
- **On re-tagging:** if a tag has been pushed and others may have seen it, do not force-move it — use a new name; changing a published tag is "a big security issue, in that people MUST be able to trust their tag-names" [T5]. GitHub's immutable releases enforce the same rule mechanically, and additionally forbid reusing a tag name even after deleting the release [T7].

`gh release create` can create a tag itself from the default branch via `--target`, but `--verify-tag` aborts if the tag does not already exist on the remote, and `--notes-from-tag` reads release notes from an annotated tag [T10]. **Recommendation:** create an annotated tag locally, push it, then use `--verify-tag --notes-from-tag` so the release cannot silently drift to a different commit.

### 3.4 Draft-first, immutable assets, and what immutability actually protects

From GitHub Docs [T6][T7][T8]:

| Property | Effect |
|---|---|
| Git tag | Locked to a commit; cannot be moved or deleted while the release exists. After deleting the release, the tag can be deleted but the name cannot be reused. |
| Release assets | Cannot be modified or deleted after publication. |
| Still editable | Title, release notes, pre-release flag, latest flag. |
| Automatic | An immutable release generates a **release attestation** with tag, commit SHA and assets. |
| Scope | Enabled per repository (Settings → Releases → *Enable release immutability*) or org-wide policy; applies to **future releases only**. |
| Extra | Protection against repository-resurrection tag reuse. |

`gh release create` handles asset attachment as draft → upload → publish but **non-atomically** [T10]. `gh release upload` without `--clobber` fails on a duplicate filename; `--clobber` deletes and re-uploads, and "if the upload fails, the original assets will be lost" [T11] — precisely the operation immutability forbids after publication. Therefore:

1. Enable immutability **before** the first release (Flora has none, so no migration).
2. Generate + validate every asset first.
3. `gh release create TAG --draft --verify-tag --notes-from-tag …`
4. `gh release upload TAG bom.json checksums.txt …` (no `--clobber`).
5. `gh release edit TAG --draft=false`.
6. Verify: `gh release verify TAG`, `gh release verify-asset TAG bom.json` [T9].

### 3.5 `release-plz` versus plain `gh release`

`release-plz` is an open-source Rust tool that automates version bump, changelog (git-cliff), `cargo publish`, git tags and GitHub/Gitea/GitLab releases; it uses Conventional Commits for version inference and integrates `cargo-semver-checks` [T12][T15]. Its configuration surface is large and directly relevant to this decision [T14]:

| Field | Relevance to Flora |
|---|---|
| `git_only = true` | Derives versions from git tags, **skips `cargo publish`**, tags packages with `publish = false`. This is the mode Flora would need (application, not a crates.io library). |
| `version_group` | Forces several packages to share a version — needed to avoid seven divergent crate versions, but must list all seven. |
| `git_tag_name` | Default in a multi-package workspace is `{{ package }}-v{{ version }}`; must be overridden to `v{{ version }}` for one product tag. |
| `git_release_draft` | Can create the release as a draft — compatible with the immutable-release workflow. |
| `release_always = false` | Releases only when the release PR is merged; GitHub-only. |
| `changelog_update` | Adds a CHANGELOG.md per crate by default; Flora has none today. |
| `semver_check` | `cargo-semver-checks` "only works with packages containing a library" — most Flora crates are libs, but `flora-tests` is not a publishable product. |
| `features_always_increment_minor` | The docs warn it violates Cargo SemVer rules for `0.x` — a concrete trap for a workspace at `0.1.0`. |

Operational cost from the official quickstart [T13]: two CI jobs, `fetch-depth: 0`, `persist-credentials: false`, `contents: write` + `pull-requests: write` (or `read` for the release job), `CARGO_REGISTRY_TOKEN`/trusted publishing, and an `if: github.repository_owner == '…'` fork guard.

| Criterion | Plain `git tag` + `gh release` | `release-plz` |
|---|---|---|
| New dependencies | `gh` (already standard), pinned generator + validator in one job | Action + bot PR + registry/token setup + config file |
| Tag model | One `vX.Y.Z` product tag, chosen explicitly | Per-crate by default; one tag only with `git_only` + `version_group` + `git_tag_name` overrides |
| Self-hosted portability | Tag layer is pure git; asset API is forge-specific | Supports GitHub, Gitea, GitLab [T14] |
| Auditability | Every step visible in one workflow + git history | Version/changelog rewritten by a generated PR |
| Immutable-release fit | Native: generate → draft → upload → publish | `git_release_draft = true` exists, but asset order still needs care |
| Solves crates.io publishing | No | Yes |
| **Verdict for cycle 029** | **Adopt** | **Defer (P3)** |

### 3.6 Binding the CycloneDX SBOM to the tag

Three distinct bindings must not be conflated:

1. **Identity binding (version).** `cargo-cyclonedx` has no `--version`/`--name` override [T16]; `metadata.component.version` comes from `Cargo.toml`. With a single `[workspace.package] version` kept equal to the tag, this is automatic. Without it, either bump all eight manifests before tagging or post-process the JSON. Any post-processing must be followed by re-validation, and the transformation itself must be in the workflow (auditable) and idempotent. (NV1)
2. **Provenance binding (tag + commit).** Put the tag and full commit SHA into the BOM's metadata (e.g. `metadata.properties` name/value pairs and/or `metadata.authors`) at generation time, derived from `GITHUB_REF_NAME` and `GITHUB_SHA` — never typed by hand. Verify the shape against the schema in the dry run (NV2). This complements the seven NTIA fields, of which *supplier name* remains the one to verify explicitly [T17].
3. **Integrity binding (digest).** Enable immutable releases → the platform records tag + SHA + asset digests in a release attestation [T7]; consumers run `gh release verify-asset` [T9]. This is stronger than a `checksums.txt` (which is still useful hygiene [T17]) and cheaper than `actions/attest`. A **CycloneDX-predicate** attestation (`https://cyclonedx.org/bom`) remains deferred per 028 P2 [T17] because repo visibility/plan and the source-vs-image coverage gap are unresolved; also, Flora ships a GHCR image, not a release binary.

Generator and validator mechanics are unchanged from [T17]: pin `cargo-cyclonedx` (0.5.10) / `cyclonedx` CLI via `taiki-e/install-action@v2` with explicit tool version and pinned action SHA; generate `--format json --spec-version 1.5 --describe crate --all --license-strict` with a deterministic `--override-filename`; validate with `cyclonedx validate --input-file … --input-format json --input-version v1_5 --fail-on-errors` (the CLI defaults to v1_7). Do not run the generator on fork PRs — it invokes Cargo and may execute build scripts [T16][T17].

### 3.7 What NOT to do

1. Do not tag without bumping the manifest version (or without a documented post-processing step) — BOM/tag skew (§3.6).
2. Do not use a lightweight tag for a release; do not force-move a published tag [T5].
3. Do not let `gh release create` auto-create the tag from the default branch — use an annotated tag + `--verify-tag` [T10].
4. Do not publish before the SBOM is generated, validated and attached; do not add/replace assets after publication [T6][T7][T10][T11].
5. Do not use `gh release upload --clobber` on a published immutable release [T11].
6. Do not mint seven per-crate tags/releases for one deployment by adopting release-plz defaults [T14].
7. Do not treat the automatic release attestation as a CycloneDX attestation — different predicate and different claim [T7][T17].
8. Do not call a crate-level SBOM a complete image SBOM; the shipped artifact is a GHCR image [T17].
9. Do not reintroduce `-Z sbom` (nightly precursor) or make SBOM generation a PR gate [T17].

---

## 4. Flora state (read-only cross-check, 2026-11-16)

| Item | Verified state | Evidence |
|---|---|---|
| Git tags | **Zero** (`git tag -l` empty) | `git tag -l` |
| Release trigger | **Absent** in all four workflows; no `push: tags:`, no `release:`, no `gh release`, no softprops/upload action | `.github/workflows/*.yml` |
| Version source | Root `[package] version = "0.1.0"`; **no** `[workspace.package] version`; all 7 members hard-code `version = "0.1.0"` | root `Cargo.toml`, `flora-*/Cargo.toml` |
| Workspace | 7 members; `resolver = "3"`, `rust-version = "1.98"`, `edition = "2024"`, `license = "AGPL-3.0-or-later"` | root `Cargo.toml` |
| Commit convention | Already Conventional Commits in practice (`feat(028)`, `docs(028)`, `chore(supply-chain)`, `fix(gitignore)`) | `git log --oneline -10` |
| Distribution | `linux/arm64` GHCR image on push to `dev`, tags `staging` + `${{ github.sha }}`, then Coolify webhook. No release binary | `.github/workflows/docker-staging.yml` |
| Supply-chain CI | fmt check, strict clippy (`--lib --bins`), `cargo deny check bans licenses sources`, dependency-review, informational cargo-vet, weekly OSV. **No SBOM** | `.github/workflows/rust.yml`, `osv-scan.yml` |
| License / notice | Full AGPL-3.0 in `LICENSE`; `NOTICE` includes §13 remote-source note and canonical URL `…/flora` | `LICENSE`, `NOTICE` |
| Repo URL drift | `Cargo.toml` `package.repository` = `…/MDMV` vs `NOTICE` = `…/flora` (pre-existing, out of scope) | `Cargo.toml`, `NOTICE` |
| Prior research | 028 specified the SBOM mechanics but explicitly stopped at "define a tag trigger"; 028 release notes list this as 029 follow-up | `docs/research/2026-11-16-028-sbom-release-plan.md`, `.specify/releases/028-fase0-closure/release.md` |
| Working tree | Clean — this research added only this document | `git status --porcelain` |

**Consequence:** cycle 028 is correct that the SBOM plan is blocked. The missing primitive is a *versioning contract*: a single number, a tag that names it, and an immutable release that carries it. Everything else in 028 can then execute unchanged.

---

## 5. Exact minimal plan (proposal only — nothing applied)

### P0 — make a version and a tag exist (unblocks 028; ~1 small manifest change + 1 tag)

1. **Adopt one version source.** Add `[workspace.package] version = "0.1.0"` and switch the seven members (and root `[package]`) to `version.workspace = true` [T2]. One file to bump thereafter. (Not applied here.)
2. **Choose the bump.** Pick `vX.Y.Z` from Conventional Commits since the last release [T4], with the `0.x` caveat from [T1][T3]. Keep bare `X.Y.Z` in manifests; `v` only on the tag [T1].
3. **Tag once, annotated, on the intended commit** [T5]:
   ```bash
   git tag -a v0.1.0 -m "Flora v0.1.0 — Seed / FASE 0 closure"
   git push origin v0.1.0
   ```
   No `-f`, ever, after the push.
4. **Add a tag-gated job** `on: push: tags: ['v*']` (new `release.yml` or a gated job in `rust.yml`), toolchain `1.98.0`, generator pinned as in [T17]. Do not trigger on every push or PR.
5. **Generate one deterministic BOM with tag identity injected.** Verify the exact output path in a dry run (028 NV2). Illustrative, not applied:
   ```bash
   cargo cyclonedx --manifest-path Cargo.toml --format json --describe crate \
     --spec-version 1.5 --target <shipped-target> --license-strict \
     --override-filename flora-sbom.cdx.json
   # post-process: set metadata.component.version = "${GITHUB_REF_NAME#v}",
   # add metadata.properties for git-tag=$GITHUB_REF_NAME and commit-sha=$GITHUB_SHA
   ```
   If step 1 is adopted, the version assignment is a no-op assertion; if not, it is a real rewrite and must be called out in the release notes. Assert `metadata.component.version == ${GITHUB_REF_NAME#v}` and fail the job otherwise (NV1).
6. **Validate before anything is published** [T17]:
   ```bash
   cyclonedx validate --input-file flora-sbom.cdx.json --input-format json \
     --input-version v1_5 --fail-on-errors
   ```
   Check the seven NTIA fields on the sample; record missing supplier names as an explicit gap [T17].
7. Upload the JSON as a workflow artifact for inspection. This step alone is a net gain and needs no GitHub Release.

### P1 — publish it immutably (needs P0 + one repo setting)

8. **Enable release immutability in repo Settings → Releases** (or org policy) *before* the first release; it applies to future releases only [T8]. There is no legacy to migrate.
9. **Draft → attach → publish**, all in one tag job [T6][T7][T10][T11]:
   ```bash
   gh release create "$GITHUB_REF_NAME" --draft --verify-tag --notes-from-tag \
     --title "$GITHUB_REF_NAME"
   # note source is either --notes-from-tag (annotated tag) OR --generate-notes [T10]
   sha256sum flora-sbom.cdx.json > checksums.txt
   gh release upload "$GITHUB_REF_NAME" flora-sbom.cdx.json checksums.txt
   gh release edit "$GITHUB_REF_NAME" --draft=false
   ```
   Job permission: `contents: write` only. No `--clobber`, no later asset edits.
10. **Verify the result** [T9]: `gh release verify "$GITHUB_REF_NAME"` and `gh release verify-asset "$GITHUB_REF_NAME" flora-sbom.cdx.json`. The automatic release attestation now ties tag + commit SHA + asset digest together [T7].

### P2 — DEFERRED: CycloneDX-predicate attestation / image SBOM (inherited from 028)

Unchanged from [T17] §5 P2: no `actions/attest@v4`, no `sbom-path`, no `https://cyclonedx.org/bom` predicate, until (a) repo visibility/plan is known, (b) the shipped artifact is decided (GHCR image vs release binary), and (c) a complete image SBOM or an explicit coverage declaration exists. The immutable-release attestation from P1 is a *partial* substitute for integrity binding, not for the CycloneDX predicate claim.

### P3 — DEFERRED: `release-plz` automation

Adopt only if/when: crates are actually published to crates.io; per-crate changelogs are wanted; API-breaking detection for library consumers becomes load-bearing; or the forge is migrated to Gitea/GitLab and a shared release bot is justified [T12][T15]. Configuration required at that point: `git_only = true`, `version_group` covering all seven crates, `git_tag_name = "v{{ version }}"`, `git_release_draft = true`, `semver_check` scoped to real libraries, `release_always = false`, plus the workflow permissions and fork guard from [T13]. **Do not** enable its default per-package tag pattern on Flora [T14].

---

## 6. Risks / NVs (not verified in this research)

- **NV1** — `cargo-cyclonedx` version/tag skew: no version override exists in the documented CLI [T16]; the post-processing shape and its interaction with schema validation were not executed. The dry run in P0 step 5 is mandatory, not optional. Carries forward 028 NV2/NV3.
- **NV2** — CycloneDX `metadata.properties` placement and property names were not validated against the 1.5 schema on a generated file; treat the example as illustrative and confirm during the dry run.
- **NV3** — Whether immutable releases are available on Flora's plan/visibility was not found stated on the EN concept page [T7] or the enablement how-to [T8]. Verify in repo Settings before depending on the automatic attestation. Same visibility uncertainty as 028 NV1.
- **NV4** — Repo default branch is `dev` in local history and in `docker-staging.yml`, while release-plz's quickstart assumes `main` [T13]. The tag trigger and any future automation must name the correct branch; no remote default-branch check was performed.
- **NV5** — `--target` must match the shipped artifact. Staging builds `aarch64-unknown-linux-gnu`; a future release binary could differ. No release-artifact decision exists (028 NV5).
- **NV6** — pt-BR GitHub Docs returned 404 for *Immutable releases*, as it did for *Managing releases* in 028 [T17] NV7. A translated page may exist under a different slug; do not cite one until re-checked.
- **NV7** — release-plz's `git_only` + `version_group` + single-tag configuration was read from docs [T14] but not trialled; the exact TOML needed for seven mixed lib/bin crates is unverified.
- **NV8** — No cost measurement: one extra tag job, Release asset storage, and any plan gating for immutability/attestations were not sized.
- **NV9** — Research only. No workflow, manifest, config, `.gitignore`, tag, release, or SBOM file was created or changed. All YAML/shell snippets are descriptions of primary docs, not applied Flora state.

---

## Citations

- **[T1]** semver, *Semantic Versioning 2.0.0* (rules 2–11, 0.y.z, FAQ on `v` prefix): https://semver.org/spec/v2.0.0.html
- **[T2]** The Cargo Book, *Workspaces* (`[workspace.package]` inheritance incl. `version`; root vs virtual manifest): https://doc.rust-lang.org/cargo/reference/workspaces.html
- **[T3]** The Cargo Book, *SemVer Compatibility* (left-most non-zero component for `0.y.z`; application compatibility): https://doc.rust-lang.org/cargo/reference/semver.html
- **[T4]** Conventional Commits, *Specification 1.0.0* (`fix`→PATCH, `feat`→MINOR, `BREAKING CHANGE`/`!`→MAJOR): https://www.conventionalcommits.org/en/v1.0.0/
- **[T5]** Git, *git-tag(1)* (annotated vs lightweight, signed tags, on re-tagging): https://git-scm.com/docs/git-tag
- **[T6]** GitHub Docs, *Managing releases in a repository* (draft-first tip, `gh release create`, immutable-release note): https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[T7]** GitHub Docs, *Immutable releases* (tag/asset protections, automatic release attestation, three-step best practice): https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases
- **[T8]** GitHub Docs, *Preventing changes to your releases* (enable per repository or org; future releases only): https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/establish-provenance-and-integrity/prevent-release-changes
- **[T9]** GitHub Docs, *Verifying the integrity of a release* (`gh release verify`, `gh release verify-asset`): https://docs.github.com/en/code-security/how-tos/secure-your-supply-chain/secure-your-dependencies/verify-release-integrity
- **[T10]** GitHub CLI manual, `gh release create` (auto-tag vs `--verify-tag`, `--notes-from-tag`, `--target`, `--draft`, non-atomic draft→upload→publish): https://cli.github.com/manual/gh_release_create
- **[T11]** GitHub CLI manual, `gh release upload` (`--clobber` deletes and re-uploads; loss on failure): https://cli.github.com/manual/gh_release_upload
- **[T12]** release-plz docs, *Introduction* (features, workspace support, git tags, GitHub/Gitea/GitLab releases): https://release-plz.dev/docs/
- **[T13]** release-plz docs, *GitHub Action quickstart* (two jobs, permissions, `fetch-depth: 0`, registry token, fork guard): https://release-plz.dev/docs/github/quickstart
- **[T14]** release-plz docs, *Configuration* (`git_only`, `version_group`, `git_tag_name`, `git_release_draft`, `release_always`, `features_always_increment_minor` warning, semver_check): https://release-plz.dev/docs/config
- **[T15]** release-plz docs, *Why yet another release tool* (cargo-semver-checks integration, comparison with release-please/cargo-release): https://release-plz.dev/docs/why
- **[T16]** OWASP CycloneDX, *cargo-cyclonedx README* (full CLI option list — no version override; workspace output; Cargo.lock + cargo metadata; build-script trust boundary): https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/main/cargo-cyclonedx/README.md
- **[T17]** Flora prior research (read-only): `docs/research/2026-11-16-028-sbom-release-plan.md` (generator pin/flags, validation, NTIA fields, immutable-release draft-first, P2 deferral) and `2026-11-16-027-advisory-sbom-alternatives.md`.
- **[T18a]** GitHub Docs (ES), *Versiones inmutables*: https://docs.github.com/es/code-security/concepts/supply-chain-security/immutable-releases
- **[T18b]** GitHub Docs (DE), *Unveränderliche Releases*: https://docs.github.com/de/code-security/concepts/supply-chain-security/immutable-releases
- **[T18c]** GitHub Docs (FR), *Versions immuables*: https://docs.github.com/fr/code-security/concepts/supply-chain-security/immutable-releases
- **[T18d]** GitHub Docs (JA), *変更不可リリース*: https://docs.github.com/ja/code-security/concepts/supply-chain-security/immutable-releases
- **[T18e]** GitHub Docs (zh-CN), *不可变版本*: https://docs.github.com/zh-cn/code-security/concepts/supply-chain-security/immutable-releases
