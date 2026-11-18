# Intake: 054 — DB env fail-closed

**Data:** 2026-11-18T00:26Z · **Âncora:** `github/dev@dd982cadc` · branch `feature/054-db-env-fail-closed`

## Sinal

Os 8 módulos de contrato DB contêm 9 pontos `db_state()` que devolvem
`Ok(None)` quando `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` não estão
definidas; os 50 testes então fazem `let Some(...) = db_state().await? else {
return Ok(()); }` e são contados como `passed`. Se alguém remover as duas linhas
de env do job `db-tests` (`.github/workflows/rust.yml:65-66`), os oito legs
continuarão verdes com zero asserções — um gate de merge vacuo e invisível.

## Evidência (verificada)

- `grep -rn 'return Ok(None);' flora-tests/src/contract/*.rs` → **9 ocorrências**
  em 8 arquivos (auth tem dois `db_state()`): `auth.rs:411,697`, `files.rs:147`,
  `messaging.rs:107`, `notifications.rs:122`, `search.rs:111`, `tasks.rs:42`,
  `websocket.rs:113`, `workspace.rs:44`.
- `grep -rn 'return Ok(());' flora-tests/src/contract/*.rs` → **50 ocorrências**
  (uma por teste `#[ignore]`).
- `rust.yml:89-104`: `run_leg` soma `N passed` e compara com o piso; não há
  verificação de env dentro da função.
- `scripts/ci/check_gate_integrity.py`: `grep POSTGRES_URL|DATABASE_URL` →
  **nenhuma ocorrência** — o guardião não protege as linhas de env.
- As linhas de env existem hoje (`rust.yml:65-66`) e a CI do PR #271 passou
  com 50 testes reais — o defeito é latente, não um incêndio atual.

## 80/20

`(I×C)/(E×R)` = (5×4)/(2×1) = **10.0** — à frente do piso de auth 12→13
(5.0), dívidas de fase (0.67), N3 (1.0); C5/C6 não autônomos. É "elevar a CI",
prioridade máxima quando a confiança do gate está em jogo.

## Fronteiras

Escopo: tornar o gate fail-closed (lado Rust e/ou workflow) e espelhar a
proteção no guardião determinístico; artefatos do ciclo e notas datadas.
Fora: mudar nomes de required checks, ADR-0003, constituição, C5/C6, deploy.
Factory lida em `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7`; nada materializado.
