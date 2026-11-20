# Research: 061 — Dead circuit breaker

**Data:** 2026-11-18T04:06Z · **Evidência local determinística**; remoto **[NV]**.

## Primária (executada)

1. `flora-core/src/utils/circuit_breaker.rs:1-5` — doc admite placeholder
   ("forwards calls without observing failures"); 68 linhas.
2. `flora-core/src/utils/mod.rs:3-4` — apenas `jwt` e `pkce`; o módulo nunca
   é carregado, portanto o arquivo não entra na compilação.
3. `config/messaging.rs:19,21` declara `circuit_breaker_threshold` e
   `circuit_breaker_recovery_secs`; `:33,34` os inicializa; nenhum outro uso.
4. `error.rs:179-181` variante `CircuitBreakerOpen`; `:365` a inclui no grupo
   de erros mapeados para 500 — apenas o arquivo morto a constrói.
5. `config/mod.rs:44-46` — Figment + `Env::prefixed("FLORA_").split("__")`,
   sem `deny_unknown_fields`: env extras são ignorados.

## Decisão

Excluir o arquivo morto, remover os dois campos de config com seus defaults
e a variante de erro com seu braço de match. Nenhum comportamento em produção
muda (o módulo nunca compilava).

## Alternativas descartadas

- Implementar o circuit-breaker de verdade: fora de escopo, sem uso real.
- Manter os campos "para o futuro": dívida morta, contraria debloat.
- Editar ADRs: governança, aprovação pendente.

## Cobertura idiomática

PT-BR (artefatos) + EN (código). **[NV]** para CI remota.
