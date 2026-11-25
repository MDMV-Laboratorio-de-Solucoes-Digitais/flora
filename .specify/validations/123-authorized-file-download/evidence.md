# Evidence — ciclo 123 download autenticado

**Data/hora**: 2026-11-25T06:25Z · **Base**: `github/dev@74f1eda10b17` · **Worktree**: `.specify/queue/worktrees/lightsout-20261125` · **Branch**: `fix/123-authorized-file-download`

Roteiro: `.specify/validations/123-authorized-file-download/test-roadmap.md`. Superfície: UI real (Playwright/Chromium) + CLI (Vitest, gates). Capturas de terminal coladas abaixo; token de teste sintético, nenhuma credencial real.

| Passo | Ação | Saída observada | Resultado | Bloco |
|---|---|---|---|---|
| 1 | Estrutura do fix | `downloadBlob` + botão + chave pt-BR + testes presentes | PASS | A |
| 2 | `bun run check` | 4902 arquivos, 0 erros, 0 avisos | PASS | B |
| 2 | `bun run lint` | Prettier limpo; ESLint 0 erros | PASS | B |
| 3 | `bun run test:unit -- --run --project server tests/unit/filesDownload.test.ts` | 1 arquivo, 2 testes, 0 falhas | PASS | C |
| 4 | `bun run test:e2e` (3 tentativas) | falhas ambientais documentadas; nenhum assertion do Passo 5c observado | **BLOCKED** | D |
| 5 | `python3 scripts/ci/check_gate_integrity.py` | `PASS`, 87 pisos, `GATE_RC=0` | PASS | E |
| 6 | `git diff --check` / status | exit 0; artefatos gerados ignorados via `.gitignore` | PASS | F |

`evidence_status: blocked` no ledger por causa do passo 4. Decisão do orquestrador: **mesclar o fix mesmo assim**, porque `e2e-browser` é job advisory (`continue-on-error: true`) e os required checks remotos (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) são a autoridade de merge; a falha local é de provisionamento do harness, não do comportamento corrigido. Escalonamento ao SWE com automação pendente está registrado no corpo/comentário do PR.

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

## Bloco D — E2E bloqueado (tentativas exatas)

1. **Tentativa 1 — 2026-11-25T06:09:35Z → 06:11:41Z** (API iniciada pelo `webServer` do Playwright contra o Postgres local `:5432`):
```text
Error: Timed out waiting 120000ms from config.webServer.
error: script "test:e2e" exited with code 1
E2E_RC=1
```
2. **API manual contra `:5432`**:
```text
Error: migration error: migration 20240818000001 was previously applied but has been modified
```
3. **Tentativa 2 — 2026-11-25T06:17:38Z → 06:17:42Z** (API já viva em `:3000`, mas `CI=true` no ambiente):
```text
CI=true
Error: http://127.0.0.1:3000/health is already used, make sure that nothing is running on the port/url or set reuseExistingServer:true in config.webServer.
error: script "test:e2e" exited with code 1
```
4. **Tentativa 3 — 2026-11-25T06:17:59Z → limite de 900 s do job**: com `CI=` vazio, Postgres 18 fresco em `:15434` (`PostgreSQL 18.7`), API pronta em `:3000` e preview em `:4173`, o comando não produziu nenhum assertion antes do teto:
```text
E2E_START=2026-11-25T06:17:59Z
[timeout do job: 900 s, sem saída de teste]
```

**Automação pendente**: script único de provisionamento E2E (DB PG18 descartável + `dev-mint` pré-compilado + API/preview com `reuseExistingServer` e `CI=` explícito) para que o Passo 5c rode em <10 min sem intervenção. Nenhuma nova tentativa será feita neste ciclo (anti-rabbit-hole). Status remoto do `e2e-browser` é **[NV]**.

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
