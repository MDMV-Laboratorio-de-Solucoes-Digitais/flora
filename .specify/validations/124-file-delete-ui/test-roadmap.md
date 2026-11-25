# Roteiro de testes — ciclo 124 exclusão de arquivos em /files

**Slug**: `124-file-delete-ui` · **Data**: 2026-11-25 · **Base**: `github/dev@cab1cab8ed74`

## Escopo / IDs

Candidato `BF-residual` (IMPLEMENTAÇÃO FE): backend `DELETE /api/v1/files/{id}`, handler owner-only e contrato DB já existem; falta a ação na UI. Consumidor = dono do arquivo em `/files`. Superfícies: UI real (Playwright/axe) + CLI (svelte-check, ESLint, gate).

## Pré-condições

- Worktree isolada `fix/124-file-delete-ui` com `node_modules` compartilhado somente leitura.
- Backend inalterado; contrato `db_delete_owner_and_non_owner_behavior` já cobre 204/403/404.
- Gate autoritativo do ALVO: `python3 scripts/ci/check_gate_integrity.py`.

## Passos

1. **Estrutura do fix** — `FilePreview` com ação “Excluir” e confirmação inline; callback `onDelete` em `+page.svelte` chamando `fileStore.removeFile`; chaves pt-BR `file.delete*`/`files.deleted`; Passo 5d no E2E. Automatizável: sim (grep). Captura: terminal.
2. **Tipagem/estilo** — `cd apps/frontend && bun run check && bun run lint`; esperado 0 erros/0 avisos. Automatizável: sim. Captura: terminal.
3. **Contrato backend existente** — `cargo test -p flora-tests db_delete_owner_and_non_owner_behavior -- --ignored --test-threads=1` com PG18; esperado owner 204, non-owner 403, cross-org 404. Automatizável: sim (depende de PG). Captura: terminal. Se o Postgres local estiver contaminado (checksum antigo), registrar `blocked` com o erro exato e usar o `db-tests` remoto como autoridade.
4. **E2E real** — `bun run test:e2e` com Passo 5d: clique em Excluir → Confirmar → `DELETE` 204 → item ausente no reload → `auditAccessibility` sem achados. Automatizável: sim, mas o harness local já apresentou três falhas de provisionamento no ciclo 123; se repetir, registrar `blocked` e usar `e2e-browser` remoto como evidência de UI.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável; esperado `PASS`, 87 pisos. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check`, `git status --short`, nenhum path alheio tocado. Automatizável: sim. Captura: terminal.

## Rollback

Reverter os quatro arquivos de frontend e os artefatos de validação/notas via commit reverso explícito; nenhuma migration, gate, workflow ou credencial é alterado.
