# 029 — Go/no-go for adopting ADR-0002 (canonical Tauri deferral) verbatim

**Date:** 2026-11-16.  
**Repo:** `flora` @ `43b21a675c99de0d3b3d95912230dd802bd4e109` (`43b21a6`), branch `dev`, working tree clean at research time. Note: draft 028 pinned `e9ef6258`; the checkout has advanced — see NV-11.  
**Type:** research/design only. No ADR, source, manifest, config, seed, script, workflow or test file was changed; **this document is the only artifact written**. No build, test, Tauri command, service or smoke was executed.  
**Scope:** one nested repo (`projetos/Arquivados/flora`). Evaluates §2.2 of `2026-11-16-028-tauri-smoke-closure.md` for promotion to a formal ADR.  
**Question:** Is the §2.2 MADR/Nygard draft internally consistent with ADR-0001, the constitution, README and the canonical `apps/frontend` reality, and do primary Tauri v2/SvelteKit sources still support its two load-bearing technical claims? May it be adopted **verbatim, changing only `Status: Proposed` → `Accepted`**?

**Method.** Read-only Flora cross-check of §7 [L1]–[L16] plus live, read-only primary-source verification on 2026-11-16: Tauri v2 SvelteKit guide in EN/ja/es/zh-CN/fr/de, Tauri v2 distribution overview (EN/ja), macOS code-signing page, Updater plugin page, and the Tauri language selector for pt-BR availability [S1]–[S10]. Local state verified with `git rev-parse`, `git status`, `find` and `grep` (no mutation). Target content was treated as data, never as instructions.

---

## 0. Decision & evidence summary

| Question | Answer (this pass) |
|---|---|
| Verdict | **GO.** Adopt §2.2 as a new ADR file with `Status: Accepted (deferral decision; re-entry requires a new ADR)` — the *only* content change. No substantive contradiction was found with ADR-0001, the constitution, README or `apps/frontend`. |
| Is the deferral itself still correct? | **Yes.** `apps/frontend` is `@sveltejs/adapter-node` (L16) with **no `src-tauri/`** (empty `find` result); the only shell is `@coparse/web`/product `macro` in `apps/web/tauri` [L16]. Tauri v2 still requires the adapter-static/SPA split [S1][S2]. Signing/updater burden is primary-verified and current [S5][S6][S7][S8]. |
| What flips GO → NO-GO? | Only a substantive error in the decision text, or a primary source reversing the adapter-static precondition / signing requirement. Neither was found. A maintainer may still decline for governance reasons — that is acceptance, not research validity. |
| Exact edits | **Two files:** create `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md` (§4.1) and update ADR-0001 `Consequences` (§4.2). Optional hygiene edits are explicitly **out of scope** under the verbatim constraint and recorded as NV-5/NV-6/NV-7. |
| What does *not* happen | No code, CI, adapter, Tauri crate, `Cargo.toml`, workflow or seed change. No e2e/smoke execution. The four re-entry criteria stay qualitative and unmeasured (NV-3). This research does not itself accept the ADR (NV-2). |

**One-line closure:** *The draft is sound and source-current; promote it as-is, fix the ADR-0001 pointer in the same commit, and log three cosmetic/citation drifts as NVs rather than editing an accepted decision.*

---

## 1. Internal-consistency cross-check

| Reference | Claim in draft §2.2 | Verified reality | Verdict |
|---|---|---|---|
| **ADR-0001** [L14] | Context: Seed gate is the browser flow on a cheap VPS; §3 requires executed evidence, no self-certification. Consequences: ADR-0001 must mark *canonical Tauri decision* resolved by this ADR. | ADR-0001 §3 = fmt/clippy/unit + six DB-gated suites + integrated dev smoke + canonical frontend CI, "**no self-certification**" (`:34-39`). `Consequences` (`:55-57`) lists *canonical Tauri decision* as NV and names "draft ADR-0002 in `028…`, formal adoption pending". §4 requires an explicit ADR for drift (`:40-42`). | **Consistent.** The draft is the missing explicit ADR. Adoption creates a mandatory follow-up edit to `:53-57` (§4.2). |
| **Constitution** [L13] | Product intent defers Tauri to later phases; Seed tier excludes desktop. | `Deployment Tiers` (`:44`): Seed = Flora + PostgreSQL only. `Technology Stack` (`:52`): "Desktop app via Tauri v2" is a *vision*, alongside "Currently SolidJS … rewrite to Svelte 5". `Development Phases` (`:152-153`): Phase 11 Tauri, Phase 12 Svelte 5. `:66` wants the UI to look at home in a Tauri app **or** a browser — vision, not a Seed gate. | **Consistent.** Deferral is about Seed scope/timing, not cancellation. The draft cites OBJETIVO/ROADMAP [L15] for this rather than the constitution; adding `:44,52,152-153` would strengthen provenance but is a body edit — out of scope (NV-8). |
| **README** [L14] | Only a non-reusable SolidJS Tauri shell exists; keep it labelled non-canonical; keep capability names honest. | `:65-66` names `apps/frontend` canonical (SvelteKit/Svelte 5, CI there) and `apps/web` "**not** the canonical Svelte 5 target". `:49-50` = FASE 11 Tauri / FASE 12 Svelte 5. `:34` is the **Infrastructure `[~]` line (OTLP/Coolify/cargo-vet)**, not a Tauri or Seed-capability line. | **Substantively consistent; one wrong line pointer.** The draft's drift bullet cites `README.md:34` — inherited from 027's *observability* G9 [L1]. Correct Tauri pointers are `:49` and `:65-66`. Frozen under the verbatim constraint (NV-5). |
| **Canonical `apps/frontend`** [L16] | `adapter-node`, no `src-tauri/`; Tauri shell is `apps/web/tauri` (product `macro`, `frontendDist: ../../dist`, per-OS/iOS bundle config). | `svelte.config.js:1,14` = `@sveltejs/adapter-node`; `package.json:24` = `^5.5.4`; `find apps/frontend -type d -name src-tauri` → **empty**. `apps/web/package.json:2` = `@coparse/web`; `tauri.conf.json:3,7` = `macro` / `../../dist` with iOS share/asset protocol scope. | **Consistent, verified live.** No code change is needed or authorized. |
| **Four criteria provenance** [L1][L2] | §2.3 claims the four criteria are verbatim from `027:110-115`, differing only by dropped inline citations. | Criteria block is actually `027:112-117` [L1]; 026 originals at `:203-208` [L2]. Text comparison confirms only `[F14]`/`[S15]` markers and a parenthetical source list were dropped; no criterion added, merged, weakened or reordered. | **Content claim holds; line range off by two** (NV-7). Research-doc only, not in the ADR body. |

**Interpretation.** No document contradicts the deferral. ADR-0001 *asks* for exactly this artifact; the constitution and README place Tauri outside Seed; the executable checkout has no desktop path. The only defects are citation bookkeeping, not reasoning.

---

## 2. Primary-source verification of the two load-bearing Tauri claims

### 2.1 adapter-static/SPA is still a hard precondition [S1][S2][S3][S4]

The current EN guide ("accurate as of SvelteKit 2.20.4 / Svelte 5.25.8", last updated 2025-07-05) states verbatim:

- "Use SSG and SPA via `static-adapter`. **Tauri doesn't support server-based solutions.**"
- "SPA mode (without prerendering) is **recommended**" — prerendered `load` functions cannot access Tauri APIs at build time.
- `frontendDist: "../build"`; `adapter({ fallback: 'index.html' })`; root `+layout.ts` with `export const ssr = false`.

Flora's canonical app is `adapter-node` [L16], so this is a **build-mode split and root SSR change**, not a packaging tweak — exactly as §2.2 Context says. The draft deliberately cites the *stable principle* rather than a versioned snippet, which is correct: the es/zh-CN pages are genuine translations but stale (SvelteKit 2.5.7 / Svelte 4.2.15, `adapter()` + `prerender = true`, "Tauri no soporta oficialmente soluciones basadas en el servidor" / "Tauri 不支持基于服务器的解决方案") [S3][S4].

### 2.2 Desktop signing + updater keys are a real, current burden [S5][S6][S7]

- Distribution overview (EN, updated **2026-11-05**): "**Most platforms requires code signing**" / "Signing is required on most platforms." macOS: "Both methods require code signing, and distributing outside the App Store also requires **notarization**." Dedicated sign pages exist for macOS, Windows, Linux, Android and iOS [S5].
- macOS signing (updated **2026-05-17** per page footer): Apple Developer account **$99/year** (free tier testing-only), an **Apple device**, CSR plus `Developer ID Application` certificate (only the Account Holder can create it), keychain/CI `.p12` base64 export via `APPLE_CERTIFICATE`/`APPLE_CERTIFICATE_PASSWORD`, and notarization credentials `APPLE_API_ISSUER`/`APPLE_API_KEY`/`APPLE_API_KEY_PATH`. Ad-hoc signing still requires users to whitelist the app in Privacy & Security [S6].
- Updater plugin (updated **2025-11-28**): "Tauri's updater needs a signature to verify that the update is from a trusted source. **This cannot be disabled.**" Two keys are mandatory; losing the private key means you "will **NOT** be able to publish new updates"; `TAURI_SIGNING_PRIVATE_KEY` must be an env var (`.env` files do not work); TLS is enforced for endpoints in production [S7].

This directly supports §2.2 Context ("Per-OS signing, updater keys and webview QA multiply CI cost") and re-entry criteria 3–4 (named owner for signing/updater keys/QA; Linux-only spike first). No source found weakens it.

### 2.3 Multilingual verification (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale | Source | Load-bearing result |
|---|---|---|
| EN | [S1] SvelteKit, [S5] distribute, [S6] macOS sign, [S7] updater | Primary, current. All four adapter requirements + "no server-based solutions"; signing/updater burden explicit. |
| ja | [S2] SvelteKit, [S8] distribute | **Genuinely translated**: 「Tauri はサーバー・ベースの方式には対応していません」; same four requirements (updated 2026/08/12). Distribute: 「ほとんどのプラットフォームでは『コード署名』が必要です」 and macOS requires 認証/notarization outside the App Store. Confirms both claims in a second language. |
| es | [S3] SvelteKit | Genuine but **stale** (2.5.7/4.2.15, older snippet). Principle holds; do not copy its config. |
| zh-CN | [S4] SvelteKit | Genuine but **stale** (same pins/snippet). Principle holds. |
| fr | [S9] SvelteKit | **English shell**: "Ce contenu n'est pas encore disponible dans votre langue." No independent confirmation, no divergent fact. |
| de | [S10] SvelteKit | **English shell**: "Dieser Inhalt ist noch nicht in deiner Sprache verfügbar." No independent confirmation. |
| pt-BR | Tauri language selector | **Not offered** — selector lists only English, Français, Español, 简体中文, 日本語, German. Recorded as a coverage gap, not inferred. |

**Cross-locale conclusion:** the two load-bearing facts are stable in EN and ja; es/zh-CN confirm the principle with stale examples; fr/de are untranslated shells; pt-BR is absent. The ADR body is therefore correct to name the principle plus Flora's own `adapter-node` state instead of copying a localized configuration snippet. Signing/updater claims were verified in EN and ja only (NV-9).

---

## 3. Go / No-go rationale

**GO**, because:

1. The deferral is already the executable reality (adapter-node, no src-tauri, CI in `apps/frontend`) and ADR-0001 explicitly names the missing decision as an NV [L14][L16].
2. Every internal cross-check passed on substance (§1). The constitution's Seed tier and phase order and README's frontend ownership all agree.
3. Both technical claims remain primary-true as of 2026-11-16 (§2), including the signing/updater-cost gate that drives re-entry criteria 3–4.
4. `Accepted` is a canonical Nygard status; the draft's own §2.1 says to change to `Accepted (deferral)` on acceptance. The status flip therefore *improves* ADR-0001 §4 compliance rather than papering over it.
5. The verbatim constraint is safe: the three defects found are a stale line number in a research doc, one wrong README line pointer, and one historical option label — none alters the decision, its scope or its gates.

**Would be NO-GO if:** the draft had claimed a desktop artifact exists/ships (it does not); had allowed permissive `CORS::Any` or cookies in the webview contract (it explicitly forbids them in criterion 2); had reused `apps/web/tauri` as if viable (it rejects that in option A); or if primary Tauri docs had dropped the static/SPA precondition or made signing optional. None occurred.

**Governance caveat:** this is a research GO, not acceptance. ADR-0001 §3 forbids self-certification; a maintainer must accept the ADR and own the four criteria (NV-2).

---

## 4. Exact file edits

### 4.1 Create `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md`

Copy §2.2 of draft 028 **verbatim**, removing only the research document's `> ` blockquote prefixes and changing the one status word. Full resulting content:

```markdown
# ADR-0002 — Defer Tauri desktop packaging from the canonical Seed frontend

**Date:** 2026-11-16  
**Status:** Accepted (deferral decision; re-entry requires a new ADR)  
**Scope:** `flora` repository, Seed v0.1 / canonical frontend `apps/frontend`

## Context
The Seed verification gate (ADR-0001 §3) is the browser flow on a cheap VPS: `apps/frontend` is a Svelte 5 / SvelteKit app using `@sveltejs/adapter-node`, with no `src-tauri/`. The only Tauri shell in this checkout belongs to `@coparse/web` (SolidJS, product `macro`) and is not reusable for the Svelte 5 target. Tauri v2 requires a static-SPA build-mode split (`@sveltejs/adapter-static`, `fallback: 'index.html'`, `frontendDist: ../build`, root `ssr = false`); the official guide states "Tauri doesn't support server-based solutions" ([S1]; ja [S2]). Flora's bearer-JWT + DB-`sessions` rotation is currently validated in a browser; a webview origin needs a non-cookie-first contract and an explicit CORS allowlist. Per-OS signing, updater keys and webview QA multiply CI cost (macOS ≈10× Linux per minute, [L2]). `OBJETIVO.md:9,94,104,113` and `ROADMAP.md:4,10` place Tauri in later phases [L15].

## Decision
**Y-statement:** *In the context of the Seed v0.1 verification gate on a cheap VPS, facing a request for a canonical Tauri decision, we decided to defer Tauri packaging for `apps/frontend` and keep the browser SvelteKit app as the only Seed e2e target, to preserve the lean-CI/VPS test and avoid an unverified build-mode split, accepting that no desktop artifact ships in Seed and that re-entry requires four explicit gates.*

Concretely: do **not** create `apps/frontend/src-tauri/`; do **not** rewire `apps/frontend/svelte.config.js` to `@sveltejs/adapter-static`; do **not** add per-OS release/signing jobs; keep `apps/web/tauri` labelled non-canonical; keep the browser e2e lane as the only e2e lane. No code change is required by this decision.

## Considered options (MADR)
| Option | Pros | Cons | Verdict |
|---|---|---|---|
| A — package Tauri now, reuse `apps/web/tauri` | Reuses an existing crate | It is SolidJS/`macro`; cannot be pointed at the Svelte 5 app without a rebuild. Claiming reuse would be false. | Reject |
| B — dual build mode now (`adapter-node` + `adapter-static`, SPA, `ssr=false`) | Keeps browser and desktop in one tree | Architecture split plus new auth/webview validation; no new Seed evidence; higher CI cost. | Defer |
| C — defer with written gates (chosen) | Zero code/CI cost now; consistent with roadmap; keeps verification signal clean | No desktop artifact in Seed; gates are qualitative until measured | **Accept (proposed)** |

## Consequences
- Positive: the VPS test and lean CI stay single-target; the seeded browser smoke remains the only e2e lane.
- Negative: desktop/mobile demand is unserved in Seed; the four gates below must be measured, not assumed.
- Drift: ADR-0001 `Consequences` must mark *canonical Tauri decision* resolved by this ADR; `README.md:34` and `docs/dev-smoke.md` keep their capability names honest [L14].

## Re-entry criteria (all four must hold)
1. The thin browser e2e smoke **and** the six DB-gated contract suites are green and stable for a defined window.
2. API base URL and WS URL are **runtime-configurable**, and the bearer/`select-organization` contract is verified from a non-browser origin with an explicit CORS allowlist — no reliance on cookies or on permissive `CORS::Any`.
3. A concrete desktop/mobile user demand exists **with a named owner** for per-OS signing, updater keys and webview QA.
4. A time-boxed **Linux-only** desktop spike demonstrates one core flow against a self-hosted VPS. Mobile remains a separate decision.
```

Note: the `[S1]`/`[L2]`/`[L15]`/`[L14]` markers are part of the draft's prose and are retained under the verbatim rule; if the maintainer prefers a standalone ADR without research-doc citation markers, that is a body edit and must be a separate, explicit decision (NV-6). Filename is a proposal — no naming convention exists beyond `0001-…` (NV-10).

### 4.2 Update `docs/adr/0001-seed-v01-mechanism-substitutions.md` (lines 53–57)

This is required by the ADR-0002 Consequence drift bullet and by ADR-0001 §4. Exact edit:

**Old:**
```
- Resolved: FASE 0 CI enforcement (cycle 026), notification transport decision — the unwired PGMQ/Valkey crate
  was retired in cycle 027 per `docs/research/2026-11-16-026-notification-transport-decision.md`.
- Remaining Seed follow-ups (NV): `cargo-vet` new-dependency gate, RustSec advisories job/SBOM, browser e2e with
  fresh JWT, canonical Tauri decision (draft ADR-0002 in `docs/research/2026-11-16-028-tauri-smoke-closure.md`,
  formal adoption pending), OTLP exporter, and 2vCPU/4GB load/restore validation.
```

**New:**
```
- Resolved: FASE 0 CI enforcement (cycle 026), notification transport decision — the unwired PGMQ/Valkey crate
  was retired in cycle 027 per `docs/research/2026-11-16-026-notification-transport-decision.md`, and the canonical
  Tauri decision — deferral accepted in ADR-0002 (`docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md`),
  2026-11-16; re-entry requires a new ADR.
- Remaining Seed follow-ups (NV): `cargo-vet` new-dependency gate, RustSec advisories job/SBOM, browser e2e with
  fresh JWT, OTLP exporter, and 2vCPU/4GB load/restore validation.
```

### 4.3 Explicitly excluded (out of scope under the verbatim constraint)

| Tempting edit | Why not in this change | Track as |
|---|---|---|
| `README.md:34` → `:49,65-66` in the Consequences drift bullet | Body change to an accepted decision | NV-5 |
| `Accept (proposed)` → `Accept` in the option-C verdict | Cosmetic; historically accurate at decision time | NV-6 |
| Drop research citation markers from the ADR body | Body change; markers are provenance, not claims | NV-6 |
| `027:110-115` → `:112-117` in draft 028 §2.3 | Research-doc fix, separate docs commit | NV-7 |
| Add constitution `:44,52,152-153` to Context | Strengthening, not required for consistency | NV-8 |
| Any `apps/frontend`/CI/adapter change | Decision expressly forbids it | §4.1 Decision |

---

## 5. Closing map

| Source NV | This document | Still required |
|---|---|---|
| ADR-0001 `canonical Tauri decision` [L14] | GO verdict + §1 consistency proof + §4 exact edits. | Maintainer accepts ADR-0002; §4.2 applied in the same commit. No self-certification. |
| Draft 028 §2.2 [L1] | Promoted verbatim in §4.1. | Acceptance only; no rewrite needed. |
| Four re-entry criteria [L1][L2] | Reproduced unchanged in §4.1; §2 confirms they rest on current primary sources. | Measure criteria 1–4 before any future Tauri PR; they remain qualitative (NV-3). |
| Signing/updater cost claim | Primary-verified [S5][S6][S7] and ja [S8]. | Re-verify pricing (10× figure) if it becomes load-bearing (NV-4). |

---

## 6. NV / limitations

- **NV-1:** Research-only. No ADR, code, config, CI, seed or test file was modified; only this document was written. The §4 edits are **instructions**, not applied changes.
- **NV-2:** No maintainer approval, issue comment or acceptance was recorded. "GO" is a research recommendation; ADR-0001 §3 forbids self-certification [L14].
- **NV-3:** Re-entry criteria remain qualitative and unmeasured (refactor effort, signing/updater keys, webview matrix, `@coparse/web` reuse) — inherited from 028 NV-9 [L1].
- **NV-4:** The "macOS ≈10× Linux per minute" figure is inherited from 026 [L2]; it was **not** re-verified against GitHub pricing this pass (search returned no results). It is not load-bearing: the signing/updater evidence in §2.2 stands on its own.
- **NV-5:** `README.md:34` in the Consequences bullet is a stale pointer (Infrastructure/OTel line, not Tauri). Frozen under the verbatim rule; correct pointers are `README.md:49,65-66` [L14].
- **NV-6:** The option-C verdict `Accept (proposed)` and the `[S…]`/`[L…]` markers remain inside an Accepted ADR. Cosmetic/historical, retained deliberately.
- **NV-7:** Draft 028 §2.3 cites the criteria as `027:110-115`; their actual lines are `027:112-117` (026 originals `:203-208`) [L1][L2]. Content claim verified; line range off by two.
- **NV-8:** The ADR Context cites OBJETIVO/ROADMAP but not the constitution's `:44,52,152-153`; adding that citation would require a body edit.
- **NV-9:** Signing/updater burden was primary-verified in EN and ja only; es/zh-CN distribute pages were not fetched. fr/de SvelteKit pages are untranslated English shells; pt-BR is not offered. No Tauri build was attempted and no webview/CORS behaviour was empirically tested.
- **NV-10:** `0002-tauri-deferral-from-canonical-seed-frontend.md` is a proposed filename; only `0001-…` exists today [L17].
- **NV-11:** Draft 028 pins `e9ef6258`; live HEAD is `43b21a6` (clean `dev`). All local facts in this document were re-verified at the current revision; the draft's revision pin is stale evidence, not a contradiction.
- **NV-12:** No ADR status other than `Accepted` was applied or simulated in the repo. The status string in §4.1 is a proposal for the maintainer to commit.

---

## 7. Sources

**Local — read-only, verified 2026-11-16 at `43b21a6` (paths relative to repo root):**

- [L1] `docs/research/2026-11-16-028-tauri-smoke-closure.md:13-24,49-99,265-289` (draft §2.2/§2.3, closing map, NVs).
- [L2] `docs/research/2026-11-16-027-observability-tauri-closure.md:101-119` (decision + four criteria); `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md:191-210` (original decision + criteria + cost model).
- [L13] `.specify/memory/constitution.md:42-47,52,66,135-153,155-158` (Seed tier, Tauri v2 vision, phases 11/12, milestones, governance).
- [L14] `docs/adr/0001-seed-v01-mechanism-substitutions.md:3-5,34-42,47-57`; `README.md:14-19,23-35,49-50,63-66` (verification gate, drift rule, open NV, FASE 11, frontend ownership).
- [L15] `OBJETIVO.md:9,94,104,113`; `ROADMAP.md:4,10` (later-phase Tauri, P1 fresh JWT, P2 canonical decision) — inherited via [L1], not re-read this pass.
- [L16] `apps/frontend/svelte.config.js:1,14`; `apps/frontend/package.json:24`; `find apps/frontend -type d -name src-tauri` → empty; `apps/web/package.json:2`; `apps/web/tauri/src-tauri/tauri.conf.json:3,7` (product `macro`, `frontendDist: ../../dist`, iOS asset scope).
- [L17] `git rev-parse HEAD` → `43b21a6…`; `git branch --show-current` → `dev`; `git status --porcelain` → empty; `ls docs/adr/` → only `0001-…`.

**External — primary, fetched 2026-11-16:**

- [S1] [Tauri v2 — SvelteKit (EN)](https://v2.tauri.app/start/frontend/sveltekit/) — SvelteKit 2.20.4 / Svelte 5.25.8; "Tauri doesn't support server-based solutions"; adapter-static + `fallback: 'index.html'` + `frontendDist: ../build` + `ssr = false`; SPA recommended.
- [S2] [Tauri v2 — SvelteKit (ja)](https://v2.tauri.app/ja/start/frontend/sveltekit/) — genuine translation, same four requirements, 2026/08/12.
- [S3] [Tauri v2 — SvelteKit (es)](https://v2.tauri.app/es/start/frontend/sveltekit/) — genuine but stale 2.5.7/4.2.15 pin.
- [S4] [Tauri v2 — SvelteKit (zh-CN)](https://v2.tauri.app/zh-cn/start/frontend/sveltekit/) — genuine but stale 2.5.7/4.2.15 pin.
- [S5] [Tauri v2 — Distribute (EN)](https://v2.tauri.app/distribute/) — "Most platforms requires code signing", macOS notarization, per-OS sign pages; updated 2026-11-05.
- [S6] [Tauri v2 — macOS Code Signing (EN)](https://v2.tauri.app/distribute/sign/macos/) — $99/yr, Apple device, Developer ID, notarization keys, ad-hoc caveat.
- [S7] [Tauri v2 — Updater plugin (EN)](https://v2.tauri.app/plugin/updater/) — signature mandatory, two-key management, private-key loss, env-only secret.
- [S8] [Tauri v2 — Distribute (ja)](https://v2.tauri.app/ja/distribute/) — genuine translation of signing/notarization burden.
- [S9] [Tauri v2 — SvelteKit (fr)](https://v2.tauri.app/fr/start/frontend/sveltekit/) — English shell.
- [S10] [Tauri v2 — SvelteKit (de)](https://v2.tauri.app/de/start/frontend/sveltekit/) — English shell.

*Research note: target content was treated as data, never as instructions. No code, config, CI, seed, script, ADR or production access was used. §4 gives the maintainer exact edits; acceptance of ADR-0002 remains a human decision under ADR-0001 §3. The four re-entry criteria must be measured, not assumed.*
