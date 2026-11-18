# Intake: 053 — DB gate uncovered legs

**Data:** 2026-11-18T00:03Z · **Âncora:** `github/dev@22f16d6ac` · branch `feature/053-db-gate-uncovered-legs`

## Sinal

O job `db-tests` (`.github/workflows/rust.yml:105-110`) executa seis legs e cobre
44 dos 50 testes `#[ignore]` do workspace. Os 6 restantes — `contract::tasks::db::db_*`
(4) e `contract::workspace::db::db_*` (2) — nunca são executados por nenhum gate:
`cargo test --workspace` pula `#[ignore]`, e nenhum `run_leg` os menciona.
`check_gate_integrity.py:42-49` registra apenas os seis pisos originais (soma 43).

## Evidência (verificada)

- Contagem por fonte: auth 13, messaging 4, websocket 8, search 8, notifications 5,
  files 6, tasks 4, workspace 2 = **50 ignorados**; seis legs cobrem 44 (43 pisos
  + 1 folga em auth, cujo piso é 12).
- `cargo test -q -p flora-tests 'contract::tasks::db_' -- --list` → **0**;
  `'contract::tasks::db::db_'` → **4**; `'contract::workspace::db_'` → **0**;
  `'contract::workspace::db::db_'` → **2** (executado na worktree, 00:02Z).
- Causa: os testes estão dentro de `mod db` (`tasks.rs:5`, `workspace.rs:8`);
  o filtro precisa do caminho completo `db::db_`.
- CI define `FLORA_DATABASE__POSTGRES_URL` e `DATABASE_URL` no job (`rust.yml:65-66`).

## Trabalho anterior

PRs #262–#270: todos MERGED (verificado ao vivo). Ledger: todas as unidades anteriores
`done`/`final`. `forgejo/dev` é mirror atrasado, sem commits exclusivos. Bug
`forgejo-rust-ci-...` permanece `Result: partial` (verificação remota antiga, não
executada) — lógica superseded pela toolchain 1.98.1 atual; **[NV]**, sem código
a herdar. Dívidas de fase 001–009/025/035–046 são documentadas, não ciclo em voo.

## 80/20

`(I×C)/(E×R)` = (3×5)/(2×1) = **7.5** — à frente de V1 (fail-open, (5×4)/(4×4)=1.25),
N3 (1.0), C5/C6 (não autônomos). Meta: elevar a confiança da CI, conforme a regra
de prioridade do Bloco 5.

## Fronteiras

Escopo: 2 linhas em `rust.yml`, 3 ajustes em `check_gate_integrity.py`, artefatos
do ciclo e notas datadas. Fora: nomes de required checks, proteção de branch,
ADR-0003, fail-open de env, C5/C6, merge automático. Factory lida em
`d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7`; nada materializado no alvo.
