# Tasks: 072 — refresh verificado de estado

## Fase 1 — apêndices documentais (append-only)

- **T001** `OBJETIVO.md` — apêndice “Estado verificado — 2026-11-19” com âncora `cdf50eaff`, contagens 61/58/57/57/4/137 e intervalo 001–070. *(FR-001, FR-002, FR-003, SC-001, SC-003)*
- **T002** `ROADMAP.md` — apêndice equivalente. *(FR-001, FR-002, FR-003, SC-001, SC-003)*
- **T003** `docs/PROGRESSO.md` — apêndice datado cobrindo 069/070, preservando a nota 02:10Z. *(FR-001, FR-002, FR-003, SC-001)*
- **T004** `docs/research/index.md` — apêndice com contagens corretas e 137/137 preservado. *(FR-001, FR-002, FR-003, SC-001)*
- **T005** `index.md` (raiz) — nota curta se apontar âncora antiga. *(FR-005, SC-001)*

## Fase 2 — rastreabilidade e entrega

- **T006** `.specify/memory/STATE.md` — decisão 80/20, execução e fechamento com `[NV]`. *(FR-004, FR-005)*
- **T007** `specs/072-verified-state-refresh/analyze.md` (READ-ONLY) — corrigir fonte se houver conflito e reexecutar. *(FR-005, SC-002)*
- **T008** `specs/072-verified-state-refresh/converge.md` — append-only. *(FR-005)*
- **T009** Gates locais do alvo; `.specify/validations/072-verified-state-refresh/validate.md` (R4) e `.specify/releases/072-verified-state-refresh/release.md` (R7). *(FR-004, SC-004)*
- **T010** PR via `gh` para `dev/`, sem merge; higiene da branch/worktree própria. *(FR-004, SC-002, SC-004)*

## Convenção de commit

Um arquivo por commit, Conventional Commits em pt-BR, paths explícitos; nunca `git add -A`/`.`; não tocar trabalho alheio.
