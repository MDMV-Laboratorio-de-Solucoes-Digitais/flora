# Test roadmap — ciclo 113

**Data**: 2026-11-22 · **Superfície**: CLI/gates (documentação, sem UI de produto alterada)

## Escopo

FR-001–004. Nenhuma mudança de código; validar que as notas de fechamento são consistentes com evidência determinística.

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `cargo test -p flora-api hitl_receipt` | 2 passed, exit 0 | CLI | sim |
| 2 | `grep -rn 'password/login' flora-api/src/routes` | rota em `auth.rs` | CLI | sim |
| 3 | `cargo test -p flora-api password` | 1 passed, exit 0 | CLI | sim |
| 4 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 10 legs / 77 pisos | CLI | sim |
| 5 | `gh pr checks 372 --watch` | required checks verdes no head exato | CI | sim |
| 6 | ACR independente | nenhum blocker determinístico crítico/high | PR | sim |

## Capturas

Saídas de terminal coladas em `evidence.md`; nenhuma imagem necessária (sem UI alterada).

## Rollback

Novo commit de reversão dos artefatos documentais; sem force-push.
