# Bug Test — OIDC redirect_uri sem fallback local

**Data**: 2026-11-22

| Verificação | Comando/inspeção | Resultado |
|---|---|---|
| Rota presente | `grep -rn 'password/login' flora-api/src/routes` | `auth.rs:83` e `auth.rs:447`, exit 0 |
| Teste focado | `cargo test -p flora-api password` | **1 passed**, 39 filtered, exit 0 |
| Gate | `python3 scripts/ci/check_gate_integrity.py` | PASS, 10 legs / 77 pisos |

## [NV]

Fluxo HTTP real em produção/staging com FQDN e IdP; não executado (credenciais/URLs pendentes na PR Forgejo #133).
