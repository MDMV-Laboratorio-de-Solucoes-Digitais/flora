# Evidence — ciclo 123 download autenticado

**Data/hora**: 2026-11-25T06:45Z · **Base**: `github/dev@74f1eda10b17` · **Worktree**: `.specify/queue/worktrees/lightsout-20261125` · **Branch**: `fix/123-authorized-file-download`

Roteiro: `.specify/validations/123-authorized-file-download/test-roadmap.md`. Superfície: UI real (Playwright/Chromium) + CLI (Vitest, gates). Capturas de terminal coladas abaixo; token de teste sintético, nenhuma credencial real.

| Passo | Ação | Saída observada | Resultado | Bloco |
|---|---|---|---|---|
| 1 | Estrutura do fix | `downloadBlob` + botão + chave pt-BR + testes presentes | PASS | A |
| 2 | `bun run check` | 4902 arquivos, 0 erros, 0 avisos | PASS | B |
| 2 | `bun run lint` | Prettier limpo; ESLint 0 erros | PASS | B |
| 3 | `bun run test:unit -- --run --project server tests/unit/filesDownload.test.ts` | 1 arquivo, 2 testes, 0 falhas | PASS | C |
| 4 | `bun run test:e2e` local (3 tentativas) / `e2e-browser` remoto | local **BLOCKED**; remoto **SUCCESS** (`4 passed`) no head exato | local BLOCKED / remoto PASS | D |
| 5 | `python3 scripts/ci/check_gate_integrity.py` | `PASS`, 87 pisos, `GATE_RC=0` | PASS | E |
| 6 | `git diff --check` / status | exit 0; artefatos gerados ignorados via `.gitignore` | PASS | F |

`evidence_status` do roteiro **local** permanece `blocked` pelo passo 4; a decisão de merge usou os required checks remotos como autoridade e o `e2e-browser` remoto como confirmação de UI. Escalonamento de automação ao SWE registrado no PR de promoção #433.

## Bloco A — estrutura

```text
apps/frontend/src/lib/api/client.ts:147  async downloadBlob(endpoint: string): Promise<Blob>
apps/frontend/src/lib/components/files/FilePreview.svelte:10  async function download()
apps/frontend/src/lib/i18n/pt-BR.ts:138  'file.downloadFailed'
apps/frontend/tests/unit/filesDownload.test.ts  (2 casos)
apps/frontend/tests/e2e/seed-flow.e2e.ts:174-186  Passo 5c
.specify/bugs/authorized-file-download-401/{bug,assessment,fix,test}.md
```

## Bloco B — check e lint

```text
$ bun run check
$ svelte-kit sync && svelte-check --tsconfig ./tsconfig.json
1795586306519 COMPLETED 4902 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
CHECK_RC=0

$ bun run lint
Checking formatting...
All matched files use Prettier code style!
LINT_RC=0
```

## Bloco C — teste unitário

```text
$ vitest --run --project server tests/unit/filesDownload.test.ts
 Test Files  1 passed (1)
      Tests  2 passed (2)
   Duration  7.54s
UNIT_RC=0
```

Casos: (1) header `Authorization: Bearer <token>` enviado e `Blob` retornado; (2) 401 aciona `authState.lockSession()` e lança `ApiError`.

## Bloco D — E2E local bloqueado / E2E remoto verde

Tentativas locais exatas:

1. **2026-11-25T06:09:35Z → 06:11:41Z** (API pelo `webServer` contra Postgres `:5432`):
```text
Error: Timed out waiting 120000ms from config.webServer.
error: script "test:e2e" exited with code 1
E2E_RC=1
```
2. **API manual contra `:5432`**:
```text
Error: migration error: migration 20240818000001 was previously applied but has been modified
```
3. **2026-11-25T06:17:38Z → 06:17:42Z** (`CI=true`, API já viva):
```text
CI=true
Error: http://127.0.0.1:3000/health is already used, make sure that nothing is running on the port/url or set reuseExistingServer:true in config.webServer.
error: script "test:e2e" exited with code 1
```
4. **2026-11-25T06:17:59Z → teto de 900 s**: com `CI=` vazio, Postgres 18 fresco em `:15434` (`PostgreSQL 18.7`), API pronta em `:3000` e preview em `:4173`, nenhum assertion foi produzido antes do timeout.

`[EV]` remoto 2026-11-25T06:40:33Z: no head exato `fc88a0930540e066cd143319b8bb155fbfeceb69`, o job advisory `e2e-browser` terminou `SUCCESS` com `Running 4 tests`, `✓ seed-flow.e2e.ts:62` e `4 passed (1.0m)` — https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45561750102/job/139026845488 . O Passo 5c está nesse smoke: o download autenticado foi exercitado em UI real na CI, embora não na máquina local.

**Automação pendente**: script único de provisionamento E2E (DB PG18 descartável + `dev-mint` pré-compilado + API/preview com `reuseExistingServer` e `CI=` explícito) para que o Passo 5c rode em <10 min localmente. Nenhuma nova tentativa foi feita neste ciclo (anti-rabbit-hole).

## Bloco E — gate determinístico (baseline no mesmo checkout)

```text
2026-11-25T05:48:17Z
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 11 db contract floors intact (87 total)
GATE_RC=0
2026-11-25T06:03:01Z
```

## Bloco F — higiene

```text
=== STATUS ===
M .specify/memory/STATE.md
M .gitignore
M apps/frontend/src/lib/api/client.ts
M apps/frontend/src/lib/components/files/FilePreview.svelte
M apps/frontend/src/lib/i18n/pt-BR.ts
M apps/frontend/tests/e2e/seed-flow.e2e.ts
?? .specify/bugs/authorized-file-download-401/
?? .specify/validations/123-authorized-file-download/
?? apps/frontend/tests/unit/filesDownload.test.ts
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

`playwright-report/` e `test-results/` passaram a ser ignorados em `.gitignore` (diff explícito registrado). Nenhum path de trabalho alheio foi tocado.
