# Problem Definition: Logger local sem transporte fantasma

- **Slug**: 049-local-logger
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`apps/frontend/src/lib/utils/logger.ts` promete envio ao backend: importa `ApiClient` (`:1`), agenda flush a cada 5 s (`:19-30`), usa `navigator.sendBeacon('/api/v1/telemetry/logs', ...)` (`:40`) e `ApiClient.request(..., POST)` no mesmo caminho (`:106`). `flora-api/src` não registra essa rota (`rg telemetry flora-api/src` → 0; `routes/mod.rs` registra 13 módulos, nenhum de telemetria), de modo que cada flush gera um 404 silencioso e descarta entradas (intake.md:7-12; research.md:5-9). O envio acontece em carregamento normal (`+layout.svelte` → `wsClient.connect()` → `websocket.ts:135`), não apenas em erros. Há também drift documental: `specs/002-svelte-frontend-rewrite/contracts/logger-contract.md:20-34` descreve `flush()` e uma Backend Ingestion API inexistentes. Oito arquivos importam o logger, `flush()` não tem chamador externo e nenhum teste cobre telemetria (intake.md:14-17). Ranqueamento 80/20 = 10.0; criar rota backend custaria 150–250 LOC sem coletor real (intake.md:20-25).

## Affected Users & Stakeholders

- **Users**: pessoas que usam a aplicação frontend e cujas entradas de log são descartadas sem aviso a cada flush; não há mensagem de erro visível porque o logger é o único destino do `catch` (intake.md:9-12).
- **Stakeholders**: mantenedores do frontend e do contrato 002, que precisam eliminar o 404 silencioso e o drift documental sem introduzir `console.log` nem superfície HTTP não autenticada (`define.md:9-14`; `decision.md:9-13`).

## Goals

- Fazer o logger operar apenas localmente: sem `fetch`, `sendBeacon`, `setInterval` ou listener `beforeunload` (`define.md:7`; `research.md:24-27`).
- Manter `debug/info/warn/error` com a mesma assinatura, preservando os 8 call sites (`define.md:8`; `decision.md:11`).
- Limitar o buffer a no máximo 200 entradas e expor `snapshot(): readonly LogEntry[]` com cópia para diagnóstico local (`define.md:9-10`).
- Atualizar o contrato 002 para descrever o comportamento local e provar por teste unitário no projeto `server` a ausência de requisição de rede e o teto do buffer (`define.md:11`; `shape.md:7-9`).

## Non-Goals

- Não criar rota/serviço backend nem exporter OTel; não adicionar dependências (`define.md:14-15`; `decision.md:10`).
- Não introduzir `console.log` nem enfraquecer a regra no-console (`define.md:16`).
- Não alterar os 8 call sites, constituição, ADRs ou PR #262 (`define.md:17`; `decision.md:13`).
- Não usar `localStorage` ou persistência: crescimento ilimitado sem necessidade (`shape.md:13`).

## Success Metrics

- `rg '/api/v1/telemetry/logs' apps/frontend/src` → 0 ocorrências (baseline: 2 ocorrências em `logger.ts:40,106` — research.md:6-7).
- Teste unitário no projeto `server` prova ausência de `fetch`/`sendBeacon` e teto de 200 entradas (baseline: nenhum teste de telemetria — intake.md:17).
- Contrato 002 descreve apenas comportamento local (baseline: promete endpoint e `flush()` inexistentes em `:20-34` — research.md:16).
- API pública inalterada: os 8 arquivos que importam o logger seguem compilando sem edição (baseline: 8 call sites — research.md:8).
- Gates frontend (check/lint/test/build) e Rust permanecem verdes (baseline: verdes; mudança sem superfície HTTP — define.md:22-23).
- Comportamento real do `sendBeacon` em produção e existência de coletor OTel externo: **[NV]** (research.md:29).

## Cost of Inaction

Cada flush continua gerando 404 silencioso e perdendo registros, enquanto o contrato 002 documenta um endpoint que não existe. Quem tentar corrigir o transporte sem evidência pode acabar criando ingestão não autenticada e nenhum coletor para receber os dados — alternativa já descartada com custo de 150–250 LOC (intake.md:23-25; shape.md:11-12).

## Open Questions

- [NEEDS CLARIFICATION: o teto de 200 entradas atende ao diagnóstico necessário ou deve ser revisado?] (`define.md:9`)
- [NEEDS CLARIFICATION: quem consumirá `snapshot()` para diagnóstico local?] (`define.md:10`; `shape.md:7`)
- [NEEDS CLARIFICATION: existe coletor OTel externo previsto para este checkout?] (`research.md:29`; **[NV]**)
- [NEEDS CLARIFICATION: comportamento em browser real e CI ao vivo será observado em qual PR?] (`decision.md:17`; **[NV]**)
