# Intake: 049 — Local logger

**Data:** 2026-11-17T15:05Z · **Âncora:** `github/dev@55363b3ca` · branch `feature/049-local-logger`

## Sinal

C3 do ranking: `apps/frontend/src/lib/utils/logger.ts` envia lotes para
`POST /api/v1/telemetry/logs` (`:40` via `sendBeacon`, `:106` via `ApiClient`),
mas `flora-api/src` não registra essa rota (`rg telemetry flora-api/src` → 0).
O endpoint é acionado em carregamento normal (`+layout.svelte` → `wsClient.connect()`
→ `websocket.ts:135` → flush em 5 s), não apenas em erros.

## Evidência verificada (agente + verificador isolado)

- 8 arquivos importam o logger; `flush()` não tem chamador externo.
- `specs/002-svelte-frontend-rewrite/contracts/logger-contract.md:20-34` promete um
  endpoint de ingestão que não existe — drift documental.
- Nenhum teste cobre telemetria; nenhum bug report em `.specify/bugs/`.

## 80/20

`(I×C)/(E×R)` = (2×5)/(1×1) = **10.0** — topo da fila após N1 (resolvido no ciclo 048).
Alternativa (b) adicionar rota backend: 150–250 LOC, superfície de ingestão não autenticada
e nenhum coletor OTel real — descartada.

## Fronteiras

Escopo: `logger.ts`, contrato 002, teste unitário. Fora: backend, constituição, ADRs,
PR #262 (DIRTY, não tocado). Decisão A preservada (`gh`/GitHub, gate do alvo).
