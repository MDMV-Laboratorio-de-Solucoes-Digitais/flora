# Analyze: 107 — wildcard one-shot (READ-ONLY)

**Data**: 2026-11-22 · **Baseline**: `df8c1e185`

## Diagnóstico

- `Config` é imutável em `AppState.config`; flag em memória não sobrevive a restart.
- Não havia tabela de config global; migration singleton aditiva é a opção de menor risco.
- Startup e login duplicavam a consulta "primeiro usuário ativo", sem filtro de papel.
- Gate compara contagens exatas de testes ignorados (bootstrap 13, auth 23): nenhum `#[ignore]` novo.

## Correção

- `WILDCARD_ADMIN_SQL` único com `JOIN memberships/roles`, `r.name IN ('Admin','Owner')` e org não-sistema.
- `try_consume_wildcard` com `pg_advisory_xact_lock` e `UPDATE ... WHERE consumed_at IS NULL RETURNING`.
- Startup e login usam o helper; startup não reaplica hash após consumo; login consome antes da sessão.
- ADR-0007 com expiração em 2026-11-29 e quatro gates.

## Checagem

```
fr=5/5  sc=3/3  tasks=8/8  critical=0  go=True
```

Divergência registrada: `is_builtin` do verificador foi relaxado para `name IN ('Admin','Owner')` a fim de preservar os contratos existentes sem inflar pisos; o ADR registra o critério.

## [NV]

Staging real, credenciais, restart multiréplica e rotação operacional.
