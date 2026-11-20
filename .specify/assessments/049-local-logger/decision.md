# Decision: 049 — Local logger

**Verdict:** go (scoped)

## Decisão

Tornar o logger frontend local-only (buffer limitado + `snapshot()`), sem transporte
de rede, e reconciliar o contrato 002. API pública inalterada.

## Delimitadores

- Nenhuma rota/serviço backend, nenhum exporter OTel, nenhuma dependência nova.
- Nenhuma regra no-console enfraquecida; nenhum call site alterado.
- PR independente, base `github/dev@55363b3ca`; PR #262 (DIRTY) não tocado.

## NV

Execução em browser real e CI ao vivo: **[NV]**.
