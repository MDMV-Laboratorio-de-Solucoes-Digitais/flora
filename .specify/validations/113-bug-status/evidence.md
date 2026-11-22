# Evidence — ciclo 113

**Data**: 2026-11-22 · **Head**: `717f9f34` · **session_title**: `flora - fechamento de bugs documentais stale`

| Passo | Ação | Saída observada | Resultado | Referência |
|---|---|---|---|---|
| 1 | `cargo test -p flora-api hitl_receipt` | 2 passed, 38 filtered, exit 0 | PASS | terminal |
| 2 | `grep -rn 'password/login' flora-api/src/routes` | `auth.rs:83`, `auth.rs:447` | PASS | terminal |
| 3 | `cargo test -p flora-api password` | 1 passed, 39 filtered, exit 0 | PASS | terminal |
| 4 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 10 legs / 77 pisos, exit 0 | PASS | terminal |
| 5 | `gh pr checks 372 --watch` | `CLEAN`; todos os checks pass no head `717f9f34` | PASS | PR #372 |
| 6 | ACR independente (`Acr372`) | nenhum blocker determinístico; veredito `correct` (advisory) | PASS | PR #372 |

## Capturas

Nenhuma imagem: escopo documental/CLI, sem UI alterada. Saídas acima coladas em texto.

## [NV]

Produção, IdP externo e staging real.
