# 028 — CycloneDX SBOM for Flora releases: minimal plan, with a documented deferral for attestation

**Date:** 2026-11-16  
**Scope:** read-only research + read-only Flora cross-check. No workflow, config, `Cargo.toml`, `.gitignore`, or SBOM file was created or changed.  
**Question:** how should Flora generate a CycloneDX SBOM for a Rust workspace at release/tag time — install and pin `cargo-cyclonedx`, choose output format, attach to a GitHub release, bind to an artifact digest via attestation, satisfy NTIA minimum elements — and what must we explicitly not do?  
**Method:** primary sources only for load-bearing claims: CycloneDX/OWASP project docs and spec, NTIA's 2021 report, the Rust Project Goals and Cargo Book, GitHub Docs and action repositories, crates.io API. Sources were consulted in **EN, ES, FR, DE, JA and zh-CN**, with a **pt-BR** lookup attempt (§2); English canonical pages are cited for every operative claim. Complements `2026-11-16-027-advisory-sbom-alternatives.md` §3.7 and §5 P2 [S27], which placed SBOM at "release only, no PR gate" without specifying the mechanics.

**Resumo (pt-BR):** o gerador correto hoje é o `cargo-cyclonedx` 0.5.10 (Apache-2.0, MSRV 1.86) — ele lê `Cargo.lock` **e** `cargo metadata`, gera JSON CycloneDX e já é o caminho de migração previsto pelo próprio projeto Rust. Mas o Flora **não tem release**: não há tags, não há workflow de release e não há release-plz. Então o plano mínimo começa por definir o gatilho `push: tags:`, gerar e validar o SBOM, e anexá-lo ao release; a **atestação vinculada ao digest** fica adiada até resolvermos (a) visibilidade/licenciamento do repositório e (b) qual artefato realmente distribuímos — hoje é uma imagem GHCR, não um binário de release. Não usar `-Zsbom` (nightly), não colocar SBOM como gate de PR, e não tratar assinatura separada como prova de vínculo.

---

## 1. Summary / decision

**Finding 1 — the generator choice is settled; the release target is not.** `cargo-cyclonedx` is the mature OWASP CycloneDX plugin for Cargo: version **0.5.10** (published 2026-11-06, MSRV 1.86.0, Apache-2.0), installed via `cargo install cargo-cyclonedx`, invoked as `cargo cyclonedx` [R1][R2][R3]. It sources data from `Cargo.lock` **and** `cargo metadata`, so it can record licenses and dependency relationships that lockfile-only tools cannot [R2]. The Rust Project's own 2026 goal explicitly plans to *convert cargo-cyclonedx to use the Cargo SBOM precursor* once stable — so choosing it now is the forward-compatible path, not a dead end [R6]. What Flora lacks is a **release process**: no git tags, no release workflow, no release-plz, no Release asset upload (§4). "Attach the SBOM to the release" therefore cannot be implemented until a tag/release trigger exists.

**Finding 2 — install and pin deterministically.** Two documented routes: `cargo install cargo-cyclonedx --version 0.5.10 --locked` (canonical upstream install, version pinned) [R1], or `taiki-e/install-action@v2` with `tool: cargo-cyclonedx@0.5.10` — `cargo-cyclonedx` is a **supported tool** in that action, installed from GitHub Releases with SHA256 checksum verification (and artifact-attestation/signature verification where published) [R4][R5]. The action itself must also be pinned (tag `@v2` or full SHA; the action repo recommends full SHA for supply-chain stability) [R4]. **Do not use the unpinned shorthand `taiki-e/install-action@cargo-cyclonedx` in a release job** — that tag floats with upstream releases [R4]. (Flora already uses `taiki-e/install-action@cargo-deny`, so the pattern is familiar; that existing shorthand is a separate issue, not changed here.)

**Finding 3 — output format is JSON; schema version lags the spec, which is fine.** `cargo-cyclonedx` emits `json` or `xml` (`-f/--format`), defaults to `bom.xml` adjacent to **every** `Cargo.toml` in the workspace, supports `--describe crate|binaries|all-cargo-targets`, `--target`, `--all` (default: direct **and** transitive), `--license-strict`, `--override-filename`, `--manifest-path`, and `--spec-version 1.3|1.4|1.5` (default **1.3**) [R2]. CycloneDX itself is at **1.7** (2025-10-21), standardised as **ECMA-424** (2025-12-10); recognised filenames are `bom.json` / `bom.xml` / `*.cdx.json`, media type `application/vnd.cyclonedx+json`, and the official attestation predicate is `https://cyclonedx.org/bom` [R7]. A 1.5 BOM is a valid CycloneDX SBOM and covers the NTIA minimum; the version delta is a documentation/validation detail, not a blocker. **But** the CycloneDX CLI validator defaults to `v1.7`, so validation must pass `--input-version v1_5` explicitly or it will validate against the wrong schema [R8].

**Finding 4 — digest binding requires an SBOM *attestation*, not two signatures.** OWASP is explicit: bind the SBOM to the final artifact's cryptographic digest through a **signed statement identifying the artifact and its SBOM**; *"Signing the artifact and SBOM separately does not establish that relationship"*; build provenance is a separate claim [R9]. GitHub implements this with `actions/attest@v4`: `subject-path` (binary) or `subject-name` + `subject-digest` (container image), plus `sbom-path`, with `permissions: id-token: write, contents: read, attestations: write` [R10][R11]. Verification is `gh attestation verify <artifact> -R OWNER/REPO --predicate-type <type>`; the action supports SPDX and CycloneDX predicates [R10]. **`actions/attest-build-provenance` v4 is now a wrapper around `actions/attest`** — use `actions/attest` for new work [R11].

**Finding 5 — NTIA minimum elements are a checklist to verify, not a property to assume.** The NTIA report (2021-07-12, pursuant to EO 14028) defines seven data fields: **Supplier Name, Component Name, Version of the Component, Other Unique Identifiers, Dependency Relationship, Author of SBOM Data, Timestamp** [R12]. `cargo-cyclonedx` gives us component name, version, PURLs (unique identifiers), a direct+transitive dependency graph, author/tool metadata and a timestamp by construction [R2][R7]. The one field that cannot be assumed is **supplier name** for each third-party crate — registry metadata does not always carry one, and NTIA requires the field (with unknown coverage recorded explicitly) [R2][R9][R12]. Verify on a generated sample before claiming NTIA conformance (NV3).

**Finding 6 — what NOT to do (see §3.7 for full list).** Do not adopt Cargo's `-Z sbom`: it is nightly-only, produces *precursor* files (not a BOM), and has a known open issue; the stabilisation goal is accepted but unfunded/unfinished [R6][R13]. Do not put SBOM generation or validation in the PR gate — OWASP asks for a BOM **per release**, reconciled with the final artifact [R9], and 027 already established that world-changing checks do not belong in blocking PR gates [S27]. Do not treat an SBOM as proof that code was reviewed or that a vulnerability is absent [R9].

**Decision: minimal plan for generation + publication (P0/P1); documented deferral for attestation (P2).** P0/P1 are cheap and unblocked except for defining a tag. P2 is deferred because (a) artifact attestations are available on public repos on all current plans but require **GitHub Enterprise Cloud for private/internal repos** [R11], and Flora's visibility is unverified (NV1, inherited from [S27]); and (b) the thing Flora actually ships is a **GHCR container image** from `docker-staging.yml`, not a release binary — binding a source-level Rust SBOM to that image digest would overstate coverage (§3.6).

---

## 2. Multilingual reliability cross-check

Primary GitHub Docs pages were read in six languages to test whether the guidance is stable across translations; method follows [S27] §2. Result: **the operative substance is stable, the edges degrade**, so only EN canonical text is cited.

| Lang | Page consulted | Observation |
|---|---|---|
| EN | Managing releases in a repository [R14] | Canonical. Immutable-release tip: create as draft, attach all assets, then publish; after publishing with immutable releases you cannot add/replace/delete assets or move/delete the tag. |
| ES | Administrar lanzamientos en un repositorio [R15] | Substance intact. Minor drift: uses *"versiones inmutables"* in the tip but *"lanzamientos inmutables"* later in the same page; also mixes untranslated UI strings ("Releases", "Publish release"). |
| FR | Gestion des versions dans un référentiel [R16] | Substance intact. Terminology churn: *versions / mises en production / lancements* on one page; the draft-first tip is complete. |
| DE | Veröffentlichungen in einem Repository verwalten [R17] | Substance intact and consistent; the immutable-release note renders fully ("unveränderliche Versionen", "Ressourcen", Tag cannot be moved/deleted). |
| JA | リポジトリのリリースを管理する [R18] | Substance intact. The CLI example string is left half-translated ("this is a パブリック プレビュー release"), and one @mention sentence is grammatically broken — example text only, not guidance. |
| zh-CN | 管理存储库中的发行版 [R19] | Substance intact. Stray `****` markers after some UI labels; one line mistranslates "release notes" as *"游戏和发行说明"* ("game and release notes"). Tip and asset-attachment steps are correct. |
| pt-BR | Managing releases / Immutable releases | **404** at both `docs.github.com/pt-br/repositories/releasing-projects-on-github/managing-releases-in-a-repository` and `docs.github.com/pt-br/code-security/concepts/supply-chain-security/immutable-releases` on 2026-11-16. Same result as [S27] NV6 for Dependabot alerts. Do not assume the pages do not exist; the locale slug may differ. (NV7) |

**Reliability rule:** translation agreement is a consistency check, never the citation. The one load-bearing item here — draft-first publication when immutable releases are enabled — is present and correct in all five translations, so it is safe to cite the EN original [R14].

---

## 3. Primary-source findings

### 3.1 `cargo-cyclonedx`: install, pin, invoke

From the upstream README and crates.io API [R1][R2][R3]:

- Install: `cargo install cargo-cyclonedx`; latest `0.5.10` (2026-11-06), MSRV `1.86.0`, license Apache-2.0, binary `cargo-cyclonedx`. Flora's toolchain is `1.98` (§4), so MSRV is satisfied.
- Invoke as a Cargo subcommand: `cargo cyclonedx ...`; a direct binary path also works if the subcommand is not on `PATH` [R1].
- Relevant flags: `--manifest-path <PATH>`, `-f/--format json|xml`, `--describe crate|binaries|all-cargo-targets` (default `crate`), `--target <TRIPLE>` (default host; `all` for all targets), `--all` (default — all dependencies, direct + transitive) vs `--top-level`, `--license-strict` (reject deprecated `/` license separators), `--override-filename <NAME>`, `--target-in-filename`, `--spec-version 1.3|1.4|1.5` (default 1.3) [R2].
- **Workspace caveat:** the README states a plain `cargo cyclonedx` "produces a `bom.xml` file adjacent to every `Cargo.toml` file that exists in the workspace" [R2]. Flora has seven workspace members (§4). A release job must therefore pin down exactly one deterministic output path — via `--manifest-path` and `--override-filename`, possibly by running from a staging directory — and **verify**, rather than assume, that only one file is produced (NV2).
- **Trust boundary:** the tool calls Cargo internally; Cargo may run arbitrary code (build scripts) on an untrusted project. Run only on trusted code in an isolated job [R1][R2]. On Flora's own tagged commit this is acceptable, but it is a reason not to run the generator on PRs from forks.

### 3.2 Format, schema validation, and the version skew

- CycloneDX spec: current **1.7** (released 2025-10-21), ECMA-424 published 2025-12-10; JSON/XML/Protobuf; IANA media type `application/vnd.cyclonedx+json`; recognised file names `bom.json`, `bom.xml`, `*.cdx.json`, `*.cdx.xml`; official in-toto predicate `https://cyclonedx.org/bom` [R7].
- Generator max spec version is **1.5** [R2]. Pick `--spec-version 1.5 --format json` so the BOM is as rich as the tool allows; a 1.5 document is valid CycloneDX and covers the NTIA fields.
- Validation: OWASP requires validating against the format schema before publication [R9]. The `cyclonedx` CLI (also installable via `taiki-e/install-action`, tool name `cyclonedx`, installed from GitHub Releases) provides [R5][R8]:
  ```
  cyclonedx validate --input-file bom.json --input-format json --input-version v1_5 --fail-on-errors
  ```
  `--input-version` defaults to `v1_7` in the CLI [R8]; without the explicit `v1_5`, a 1.5 document may be checked against the wrong schema. This is a real, easy-to-miss footgun.

### 3.3 NTIA minimum elements — mapping and the supplier gap

The seven NTIA fields [R12], mapped against what a `cargo-cyclonedx` BOM provides [R2][R7]:

| NTIA field | Source in a cargo-cyclonedx BOM | Status |
|---|---|---|
| Supplier Name | Component `publisher`/`supplier` from registry metadata | **Verify** — may be absent for some crates (NV3) |
| Component Name | `component.name` | Covered |
| Version | `component.version` | Covered |
| Other Unique Identifiers | `component.purl` (Package URL) | Covered |
| Dependency Relationship | `dependencies` graph; `--all` = direct + transitive | Covered |
| Author of SBOM Data | BOM `metadata.authors` / `tools` | Covered (generator identity/version) |
| Timestamp | BOM `metadata.timestamp` | Covered |

OWASP adds: capture generation scope and known gaps explicitly, and *"a component missing from an SBOM is not evidence that it is absent"* [R9]. If supplier names are missing, that is a declared gap, not a reason to stop — but the release doc must not claim full NTIA conformance until checked.

### 3.4 Release mechanics

- A release is created from a **tag**; the UI and `gh release create TAG` both attach binary assets at creation time, and GitHub also exposes the Releases API [R14].
- For CI, the REST endpoint is `POST /repos/{owner}/{repo}/releases/{release_id}/assets?name=<file>` on `https://uploads.github.com`, with raw binary body and `Content-Type`; uploading a file with an existing name returns **422** and requires deleting the old asset first [R20]. `gh release upload TAG FILE` is the CLI equivalent (same underlying API) [R14][R20].
- **Immutable releases:** if enabled, create as **draft → attach all assets → publish**; after publication you cannot add, replace or delete assets, and cannot move/delete the tag [R14]. A release job that creates the release first and uploads assets in a later step will fail under this setting. This is today a **prerequisite question**, not an observed Flora setting (NV4).
- Use a CyDX media type label where the API allows (`application/vnd.cyclonedx+json`) [R7].

### 3.5 Digest binding / attestation

- `actions/attest@v4`, permissions `id-token: write`, `contents: read`, `attestations: write` [R10]:
  ```yaml
  - uses: actions/attest@v4
    with:
      subject-path: 'PATH/TO/ARTIFACT'   # or subject-name + subject-digest for an image
      sbom-path: 'bom.json'
  ```
- For a container image: `subject-name` (FQDN, no tag), `subject-digest: 'sha256:...'` from `docker/build-push-action`'s `digest` output, `sbom-path`, `push-to-registry: true`, plus `packages: write` [R10].
- The attestation binds subject name + digest to a signed statement using in-toto; signature is short-lived Sigstore-issued; uploaded to the GitHub attestations API; verified with `gh attestation verify ... --predicate-type https://cyclonedx.org/bom` [R10][R11][R7].
- **Licensing constraint:** artifact attestations are available in public repos on all current plans; on GitHub Free/Pro/Team they are **only** available for public repos; private/internal repos need GitHub Enterprise Cloud [R11]. Combined with unknown Flora visibility (NV1), this alone justifies deferral.
- A signature authenticates the claim and its integrity — it does **not** prove the SBOM is accurate, complete, or that the software is safe [R9].

### 3.6 Coverage gap: source SBOM vs shipped container image

Flora's `Dockerfile` builds `flora-api` in `rust:1.98-slim` and copies it into `debian:trixie-slim` with `ca-certificates` and `curl`. `docker-staging.yml` pushes `linux/arm64` images to GHCR tagged `staging` and `${{ github.sha }}` (§4). A `cargo-cyclonedx` BOM inventories **Rust crate dependencies**, not the Debian base image or its APT packages. OWASP requires reconciling the inventory with the final package/container image and including bundled libraries and OS packages where applicable [R9]. So:

- A source-level SBOM is valid and useful for crate-level incident response, but **must not be labelled a complete image SBOM**.
- Binding it to the GHCR image digest via `sbom-path` would assert a relationship to an artifact it does not fully describe. Options: (a) also generate an image SBOM (GitHub's own docs point to `anchore-sbom-action` for SBOM attestations [R10]); or (b) explicitly record incomplete/unknown coverage in the BOM and the release notes [R9]. This is a design decision, deferred with P2.

### 3.7 What NOT to do

1. **Do not use Cargo's `-Z sbom`.** It is listed under *Unstable Features* ("only available on the nightly channel"); it emits *precursor* files named `<artifact>.cargo-sbom.json`, not a BOM; it has at least one known issue; the 2026 stabilization goal is Accepted but explicitly notes funding may not materialise [R6][R13]. Revisit after stabilization and after cargo-cyclonedx is converted to consume it [R6].
2. **Do not make SBOM generation/validation a PR gate.** OWASP asks for a BOM per release, reconciled with the final artifact [R9]; cargo-cyclonedx executes Cargo/build scripts, so PRs from forks are a trust-boundary problem [R2]. 027's rule — world-changing checks do not block unrelated work — applies here too [S27].
3. **Do not sign the artifact and the SBOM separately and call it binding.** That does not establish the relationship; use an SBOM attestation [R9].
4. **Do not publish before schema validation**, and do not skip the `--input-version v1_5` flag [R8][R9].
5. **Do not build the release, publish it, then upload the SBOM in a later step** if immutable releases are enabled — draft first, attach all assets, then publish [R14].
6. **Do not upload an asset with a filename that already exists on the release** — 422, and the release must be edited to remove it [R20].
7. **Do not treat the SBOM as evidence of review, absence of vulnerabilities, or inventory completeness.** Valid signatures do not prove accuracy; missing components are not evidence of absence; VEX "not affected" claims must be authenticated [R9].
8. **Do not run three overlapping scanners as three gates** — 027 established they share upstream RustSec data [S27]. SBOM generation is not a scanner at all.

---

## 4. Flora state (read-only cross-check, 2026-11-16)

| Item | Verified state | Evidence |
|---|---|---|
| Workspace | 7 members (`flora-core`, `flora-api`, `flora-messaging`, `flora-tasks`, `flora-files`, `flora-search`, `flora-tests`); `resolver = "3"`, `package.rust-version = "1.98"`, `edition = "2024"`, `license = "AGPL-3.0-or-later"` | root `Cargo.toml` |
| Toolchain in CI | `1.98.0` installed explicitly in both `build` and `db-tests` jobs | `.github/workflows/rust.yml` |
| Release trigger | **Absent.** No tag push, no `release:` trigger, no `release-plz`, no `gh release`/softprops upload anywhere in `.github/workflows/`; `git tag -l` empty | `.github/workflows/*.yml`, `git tag -l` |
| Distribution | `docker-staging.yml` builds `linux/arm64` image on push to `dev` (path-filtered), pushes `ghcr.io/…/flora:staging` and `:${{ github.sha }}`, then POSTs to a Coolify webhook. **No release binary.** | `.github/workflows/docker-staging.yml` |
| Dockerfile | `rust:1.98-slim` builder → `debian:trixie-slim` + `ca-certificates` + `curl`; `cargo build --release -p flora-api` | `Dockerfile` |
| License / notice | Full AGPL-3.0 text in `LICENSE` (34,523 B); `NOTICE` (1,029 B) includes section 13 remote-source note and canonical source URL `…/flora` | `LICENSE`, `NOTICE` |
| Repo URL mismatch | `Cargo.toml` `package.repository` = `…/MDMV`, while `NOTICE` points to `…/flora` | `Cargo.toml`, `NOTICE` |
| Existing supply-chain CI | fmt check, strict clippy (`--lib --bins`), `cargo deny check bans licenses sources` via `taiki-e/install-action@cargo-deny`; no advisories, audit, OSV or SBOM step | `.github/workflows/rust.yml`; [S27] |
| Working tree | Clean — this research changed nothing except adding this file | `git status --porcelain` |
| Visibility / plan | **Not verified** — gates the attestation option [R11] | NV1 |

**Consequence:** the question "how do we attach an SBOM to a release" is currently **blocked on having a release**. The plan below therefore starts by defining a tag trigger; without it, generating an SBOM on every push would produce unreferenced artifacts and no release to bind them to.

---

## 5. Minimal plan (proposal only — nothing applied)

### P0 — define the release trigger and generate a valid BOM (unblocked, ~1 workflow)

1. Add `on: push: tags: ['v*']` to a new `release.yml` (or extend `rust.yml` with a gated job). Do **not** trigger on every push or PR.
2. Pin the toolchain to `1.98.0` as existing jobs do; install the generator pinned:
   ```
   cargo install cargo-cyclonedx --version 0.5.10 --locked
   ```
   or
   ```yaml
   - uses: taiki-e/install-action@v2
     with:
       tool: cargo-cyclonedx@0.5.10
       fallback: none      # fail rather than silently fall back to an unpinned install
   ```
   Pin the action itself to a tag or full SHA [R1][R2][R4][R5].
3. Generate one deterministic JSON BOM (verify the output path/name in a dry run — NV2):
   ```
   cargo cyclonedx --manifest-path Cargo.toml --format json --describe crate \
     --spec-version 1.5 --target <shipped-target> --license-strict \
     --override-filename flora-sbom.cdx.json
   ```
   `--all` is the default and gives direct + transitive relationships; `--target` must match whatever the release actually ships (NV5).
4. Validate before publishing [R8][R9]:
   ```
   cyclonedx validate --input-file flora-sbom.cdx.json --input-format json \
     --input-version v1_5 --fail-on-errors
   ```
5. Check the seven NTIA fields on the generated sample; if supplier names are missing, record the gap explicitly rather than claiming full conformance [R12][R9] (NV3).
6. Upload the JSON as a workflow artifact for inspection. This step alone is a net gain and needs no GitHub Release.

### P1 — attach to a GitHub release (needs P0 + a decision on immutable releases)

7. Create the release as a **draft**, attach all assets, then publish [R14]. Use `gh release create "$GITHUB_REF_NAME" --draft …` followed by `gh release upload "$GITHUB_REF_NAME" flora-sbom.cdx.json …`, then `gh release edit … --draft=false`; or the Releases API equivalent [R14][R20]. Grant only `contents: write` for this job.
8. Attach a `checksums.txt` (sha256) alongside the BOM so consumers can verify the downloaded document; this is hygiene, not the digest binding of §3.5.
9. Confirm whether immutable releases are enabled for the repository and order steps accordingly (NV4).

### P2 — DEFERRED: signed digest binding / attestation

**Deferred, not rejected.** Do not add `actions/attest` yet, because:

- repo visibility and plan are unverified, and private/internal repos require GitHub Enterprise Cloud [R11] (NV1);
- the distributed artifact is a GHCR image, not a release binary, so a source-level Rust SBOM would need either a companion image SBOM or an explicit coverage declaration before being attested against the image digest [R9][R10] (§3.6);
- no release binary exists to use as `subject-path` today.

Revisit P2 when a release binary or a deliberate image-SBOM decision exists. At that point use `actions/attest@v4` with `sbom-path` and either `subject-path` or `subject-name` + `subject-digest`, verify with `gh attestation verify … --predicate-type https://cyclonedx.org/bom`, and pin the action to a full SHA [R10][R11][R7].

---

## 6. Risks / NVs (not verified in this research)

- **NV1** — Repository visibility and GitHub plan unknown. Artifact attestations are public-repo-only on Free/Pro/Team; private/internal needs Enterprise Cloud [R11]. Also carried from [S27].
- **NV2** — Exact output behaviour of `cargo-cyclonedx 0.5.10` on this seven-member workspace was **not executed**. The README says a default run writes `bom.xml` adjacent to every `Cargo.toml` [R2]; whether `--manifest-path` + `--override-filename` yields exactly one file in one known location must be verified in a dry run.
- **NV3** — No BOM was generated, so presence of all seven NTIA fields — especially per-component **supplier name** — is unverified [R12]. Also unverified: whether `--license-strict` passes on the current 452-package lockfile (026 recorded alias/pin problems in the license gate [S27]).
- **NV4** — Immutable-releases setting for the repository is unknown; it changes the required step order for asset upload [R14].
- **NV5** — `--target` vs shipped artifact: staging builds `linux/arm64` (`aarch64-unknown-linux-gnu`), while a future release binary could be `x86_64-unknown-linux-gnu`. SBOM target must match the artifact; no decision exists [R2], §4.
- **NV6** — `cargo-cyclonedx` MSRV is 1.86 and the crate is edition 2021 [R3]; it installed cleanly is assumed from metadata, not tested on the `1.98.0` CI toolchain.
- **NV7** — pt-BR GitHub Docs returned 404 at both attempted URLs (§2). Portuguese-facing guidance could not be checked from the canonical source; re-check the slug before citing a translated page. Same pattern as [S27] NV6.
- **NV8** — CycloneDX CLI `--input-version v1_5` support was read from the current README [R8]; exact behaviour against a generator-produced 1.5 file was not executed.
- **NV9** — No cost measurement: runner minutes for an extra tag job, Release asset storage, and any Enterprise Cloud licensing were not sized.
- **NV10** — Research only. No workflow, config, `Cargo.toml`, `.gitignore`, or SBOM file was modified; YAML/shell snippets are descriptions of upstream docs, not applied Flora state.

---

## Citations

- **[R1]** CycloneDX, *cyclonedx-rust-cargo* repository README (install, `cargo cyclonedx`, security considerations): https://github.com/CycloneDX/cyclonedx-rust-cargo
- **[R2]** CycloneDX, *cargo-cyclonedx* crate README (CLI options, workspace output, `Cargo.lock` + `cargo metadata`, security considerations): https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/main/cargo-cyclonedx/README.md
- **[R3]** crates.io API, `cargo-cyclonedx` (0.5.10, 2026-11-06, MSRV 1.86.0, Apache-2.0, bin `cargo-cyclonedx`): https://crates.io/api/v1/crates/cargo-cyclonedx
- **[R4]** taiki-e, *install-action* README (`tool: <tool>@<version>`, pinning, checksums, `fallback`): https://github.com/taiki-e/install-action
- **[R5]** taiki-e, *install-action* `TOOLS.md` (`cargo-cyclonedx` → GitHub Releases, SHA256; `cyclonedx` CLI): https://github.com/taiki-e/install-action/blob/main/TOOLS.md
- **[R6]** Rust Project Goals 2026, *Stabilize Cargo SBOM precursor* (nightly-only, known issue, RFC #3553, plan to convert cargo-cyclonedx, funding not secured): https://goals.rust-lang.org/2026/stabilize-cargo-sbom.html
- **[R7]** CycloneDX, *Specification Overview* (v1.7, ECMA-424, media types, recognised filenames, predicate `https://cyclonedx.org/bom`): https://cyclonedx.org/specification/overview/
- **[R8]** CycloneDX CLI, README (`validate`, `--input-file`, `--input-format`, `--input-version` default v1.7, `--fail-on-errors`): https://github.com/CycloneDX/cyclonedx-cli
- **[R9]** OWASP Cheat Sheet Series, *Dependency Graph & SBOM* (per release, schema validation, signed attestation binding, incomplete coverage, limits of signatures and VEX): https://cheatsheetseries.owasp.org/cheatsheets/Dependency_Graph_SBOM_Cheat_Sheet.html
- **[R10]** GitHub Docs, *Using artifact attestations to establish provenance for builds* (`actions/attest@v4`, `subject-path`, `subject-digest`, `sbom-path`, permissions, `gh attestation verify`, SPDX/CycloneDX predicates): https://docs.github.com/en/actions/security-for-github-actions/using-artifact-attestations/using-artifact-attestations-to-establish-provenance-for-builds
- **[R11]** GitHub, `actions/attest-build-provenance` README (v4 is a wrapper on `actions/attest`; in-toto + Sigstore; public-plan/Enterprise-Cloud attestation availability): https://github.com/actions/attest-build-provenance
- **[R12]** NTIA, *The Minimum Elements For a Software Bill of Materials (SBOM)*, 2021-07-12 (seven fields): https://www.ntia.doc.gov/report/2021/minimum-elements-software-bill-materials-sbom ; PDF: https://www.ntia.doc.gov/sites/default/files/publications/sbom_minimum_elements_report_0.pdf
- **[R13]** The Cargo Book, *Unstable Features* (`-Z sbom` nightly-only, precursor files, tracking issue/RFC): https://doc.rust-lang.org/cargo/reference/unstable.html#sbom
- **[R14]** GitHub Docs, *Managing releases in a repository* (tag-based creation, asset attachment, `gh release create`, immutable-release draft-first rule): https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[R15]** GitHub Docs (ES), *Administrar lanzamientos en un repositorio*: https://docs.github.com/es/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[R16]** GitHub Docs (FR), *Gestion des versions dans un référentiel*: https://docs.github.com/fr/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[R17]** GitHub Docs (DE), *Veröffentlichungen in einem Repository verwalten*: https://docs.github.com/de/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[R18]** GitHub Docs (JA), *リポジトリのリリースを管理する*: https://docs.github.com/ja/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[R19]** GitHub Docs (zh-CN), *管理存储库中的发行版*: https://docs.github.com/zh-cn/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **[R20]** GitHub Docs, *REST API endpoints for release assets* (`POST …/releases/{release_id}/assets`, raw binary, 422 on duplicate filename): https://docs.github.com/en/rest/releases/assets
- **[S27]** Flora prior research (read-only): `docs/research/2026-11-16-027-advisory-sbom-alternatives.md` (§3.7 SBOM at release, §5 P2, NV1/NV6)
