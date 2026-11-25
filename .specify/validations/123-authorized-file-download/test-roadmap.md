# Roteiro de testes — ciclo 123 download autenticado

**Slug**: `123-authorized-file-download` · **Data**: 2026-11-25 · **Base**: `github/dev@74f1eda10b17`

## Escopo / IDs

Bug `authorized-file-download-401`: o botão “Baixar” em `/files` usa `<a href>` sem `Authorization`, enquanto o endpoint exige Bearer. Nenhuma FR/T de spec nova; consumidor = membro autenticado que fez upload. Superfícies: UI real (Playwright) + CLI (Vitest, gates).

## Pré-condições

- Worktree isolada `fix/123-authorized-file-download` com `node_modules` compartilhado via symlink somente leitura.
- Sessão de teste com token sintético para unitário; E2E usa seed `flora-mint-session` (jti rotacionado).
- Gate autoritativo do ALVO: `python3 scripts/ci/check_gate_integrity.py`.

## Passos

1. **Estrutura do fix** — confirmar `ApiClient.downloadBlob`, botão em `FilePreview.svelte`, chave `file.downloadFailed`, teste unitário e Passo 5c no E2E. Automatizável: sim (grep). Captura: terminal.
2. **Tipagem/estilo** — `cd apps/frontend && bun run check && bun run lint`; esperado 0 erros/0 avisos e exit 0. Automatizável: sim. Captura: terminal.
3. **Unitário** — `bun run test:unit -- --run --project server tests/unit/filesDownload.test.ts`; esperado 2 testes: Bearer enviado + 401 trava sessão. Automatizável: sim. Captura: terminal.
4. **E2E real** — subir Postgres 18 + API, executar `bun run test:e2e` (smoke completo com Passo 5c); esperado response 200, header `authorization: Bearer ...`, `download.suggestedFilename() === uploadedFileName` e axe sem achados. Requer backend local; se indisponível, registrar `evidence_status: blocked` e escalar por capacidade, sem fingir PASS. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável; esperado `PASS`, 87 pisos. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check`, `git status --short`, nenhum path de trabalho alheio tocado. Automatizável: sim. Captura: terminal.

## Rollback

Reverter os arquivos de frontend e os artefatos de bug/validação deste ciclo via commit reverso explícito; nenhuma migration, gate, workflow ou credencial é alterado. E2E e gate permanecem responsabilidade da CI remota no head do PR.
