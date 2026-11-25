# Evidence — ciclo 124 exclusão de arquivos em /files

**Data/hora**: 2026-11-25T07:40Z · **Base**: `github/dev@cab1cab8ed74` · **Worktree**: `.specify/queue/worktrees/lightsout-124` · **Branch**: `fix/124-file-delete-ui`

Roteiro: `.specify/validations/124-file-delete-ui/test-roadmap.md`. Superfície: UI real + CLI. Nenhuma credencial real em artefato.

| Passo | Ação | Saída observada | Resultado | Bloco |
|---|---|---|---|---|
| 1 | Estrutura do fix | ação Excluir + confirmação inline + callback + chaves pt-BR + Passo 5d | PASS | A |
| 2 | `bun run check` | 4902 arquivos, 0 erros/0 avisos | PASS | B |
| 2 | `bun run lint` | Prettier + ESLint exit 0 | PASS | B |
| 3 | Contrato backend existente | não reexecutado localmente; DB `:5432` com checksum antigo | local não executado / `db-tests` remoto PASS | C |
| 4 | E2E local / remoto | local **BLOCKED**; `e2e-browser` remoto **SUCCESS** no head `40d299e246` | local BLOCKED / remoto PASS | D |
| 5 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 87 pisos, `GATE_RC=0` | PASS | E |
| 6 | `git diff --check` / status | exit 0; somente paths deste ciclo | PASS | F |

`[EV]` remoto no head exato `40d299e246f205ae5eac2c7d0c045561071fca60`: `frontend-build`, `rust-build`, `db-tests`, `dependency-review` e `e2e-browser` SUCCESS. A evidência de UI vem da CI remota; nenhum PASS local de E2E é afirmado.

## Bloco A — estrutura

```text
apps/frontend/src/lib/components/files/FilePreview.svelte:12  isConfirmingDelete / isDeleting
apps/frontend/src/lib/components/files/FilePreview.svelte:32-53  remove() com DELETE e confirmação
apps/frontend/src/routes/(app)/files/+page.svelte:85-89  handleDeleted -> fileStore.removeFile
apps/frontend/src/lib/i18n/pt-BR.ts:143-147  file.delete*, files.deleted
apps/frontend/tests/e2e/seed-flow.e2e.ts:188-208  Passo 5d com asserção Bearer no DELETE
```

## Bloco B — check e lint

```text
$ bun run check
1795591809678 COMPLETED 4902 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
CHECK_RC=0

$ bun run lint
Checking formatting...
All matched files use Prettier code style!
LINT_RC=0
```

Uma correção foi necessária durante a validação: `confirmingDelete` renomeado para `isConfirmingDelete` para satisfazer `unicorn/consistent-boolean-name`; novo check/lint PASS.

## Bloco C — contrato backend

O endpoint `DELETE /api/v1/files/{id}` já existe (`flora-api/src/routes/files.rs:98,308-329`) e o contrato cobre owner `204`, não-owner `403` (`flora-tests/src/contract/files.rs:483-548`) e cross-org `404` (`db_cross_org_cannot_read_download_delete`, `:369-430`); a ausência na listagem vem do filtro `is_deleted = true` em `file_repository.rs:52`. Não foi reexecutado localmente porque o Postgres compartilhado em `:5432` falha com `migration 20240818000001 was previously applied but has been modified` desde o ciclo 123. O `db-tests` remoto foi SUCCESS no head do PR.

## Bloco D — E2E local bloqueado / remoto verde

Não houve nova tentativa local: o mesmo harness apresentou, no ciclo 123, `Timed out waiting 120000ms from config.webServer`, erro de migration checksum e teto de 900 s sem assertion. Antes de repetir, é necessário o script de provisionamento registrado ao SWE (PG18 descartável + `dev-mint` pré-compilado + `reuseExistingServer`/`CI=`).

`[EV]` remoto: `e2e-browser` SUCCESS no head `40d299e246`, com `seed-flow.e2e.ts` e o Passo 5d — https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/45568843684/job/139050220020 . As três sugestões P3 da ACR foram aplicadas antes deste resultado: asserção de `Authorization: Bearer` no DELETE, `role="status"` na confirmação e citações exatas do contrato.

## Bloco E — gate determinístico

```text
2026-11-25T07:03:50Z
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 11 db contract floors intact (87 total)
GATE_RC=0
2026-11-25T07:15:40Z
```

## Bloco F — higiene

```text
=== STATUS ===
M .specify/memory/STATE.md
M apps/frontend/src/lib/components/files/FilePreview.svelte
M apps/frontend/src/lib/i18n/pt-BR.ts
M apps/frontend/src/routes/(app)/files/+page.svelte
M apps/frontend/tests/e2e/seed-flow.e2e.ts
?? .specify/validations/124-file-delete-ui/
=== DIFF CHECK ===
DIFFCHECK_RC=0
```

Nenhum path de outra sessão foi tocado. `node_modules` é symlink somente leitura para o checkout compartilhado; não versionado.
