# Plan: 114 — blindagem WCAG 2.2 AA no e2e-browser e rota /files

**Data**: 2026-11-22 · **Baseline**: `github/dev@b32f7cafe`

## Mapeamento FR/SC → Arquivos e Verificação

| FR / SC | Arquivos alvo | Verificação |
|---|---|---|
| FR-001 / SC-001 | `apps/frontend/src/lib/utils/a11yAudit.ts`, `apps/frontend/tests/unit/a11yAudit.test.ts` | TDD RED→GREEN com `bun run test:unit -- --run --project server tests/unit/a11yAudit.test.ts` |
| FR-002 / SC-002 | `apps/frontend/src/lib/i18n/pt-BR.ts`, `apps/frontend/src/lib/components/files/FileUpload.svelte`, `apps/frontend/src/lib/components/files/FilePreview.svelte` | `bun run check`, `bun run lint`, `bun run build` e auditoria `/files` no job `e2e-browser` |
| FR-003 / SC-003 | `apps/frontend/tests/e2e/seed-flow.e2e.ts` | Job `e2e-browser` do PR no GitHub Actions com `"sem achados"` em `/tasks`, `/channels` e `/files` |
| FR-004 / SC-004 | `.specify/memory/STATE.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `OBJETIVO.md`, `ROADMAP.md`, `docs/governance/2026-11-22-114-wcag-ci-guard.md` | Contagem determinística via `find` + `git diff --check` |
| FR-005 / SC-005 | `.specify/validations/114-wcag-ci-guard/{test-roadmap,validate,evidence}.md`, `.specify/releases/114-wcag-ci-guard/release.md`, `specs/114-wcag-ci-guard/converge.md` | `python3 scripts/ci/check_gate_integrity.py`, `gh pr checks`, ACR independente |

## Passos ordenados

1. Concluir artefatos pré-implementação (`checklist.md` → `tasks.md` → `analyze.md` read-only).
2. Executar passo obrigatório `11b taskstoissues` criando 5 issues GitHub (`T-001`..`T-005`) sem label `dark-factory:eligible`.
3. Implementar `T-001` em TDD estrito (teste falhando primeiro → `a11yAudit.ts` → teste passando).
4. Implementar `T-002` (`/files` + pt-BR) e `T-003` (`seed-flow.e2e.ts`), validar com `check`, `lint`, `test:unit -- --run --project server`, `build`, `check_gate_integrity.py` e `git diff --check`.
5. Implementar `T-004` (docs + contagens via `find`), `converge.md`, `test-roadmap.md` (antes do PR), `validate.md` inicial e `release.md`; commitar com paths explícitos.
6. Executar `T-005`: abrir PR para `dev`, acompanhar CI remoto (`gh pr checks --watch`) e log do `e2e-browser`, conduzir ACR independente em contexto limpo (modelo distinto), gravar `evidence.md` + atualização R4, aguardar novo verde no head final e mesclar em `dev` com `--match-head-commit`.
7. Atualizar PR #371 (`release/112-promotion` → `main`), executar ACR + babysitting até GREEN, escalar ao SWE (7b.4) sem mesclar `main` nem fazer deploy.

## Venue (0.8)

Build/teste pesado e Chromium `e2e-browser` executam no GitHub Actions (repositório público, Venue #1); localmente rodam apenas checagens baratas (`check`, `lint`, Vitest `server`, `build`, `check_gate_integrity.py`).
