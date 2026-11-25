# Decision: 049 — Logger local sem transporte fantasma

- **Slug**: 049-local-logger
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `logger.ts` envia lotes para `/api/v1/telemetry/logs` (`sendBeacon:40`, `ApiClient:106`) mas `flora-api/src` não registra a rota (`rg telemetry` → 0), gerando 404 silencioso e perda de entradas em carregamento normal (intake.md:7-12; research.md:5-9; problem.md:9). |
| Evidence strength | strong | Evidência local verificada: 8 arquivos importam o logger, `flush()` sem chamador externo, contrato 002 descreve endpoint e `flush()` inexistentes, e teste unitário no projeto `server` é viável (intake.md:13-18; research.md:12-18). |
| Value vs. inaction | strong | Elimina 404 silencioso e drift documental sem criar ingestão não autenticada — alternativa de backend descartada por 150–250 LOC e nenhum coletor real (intake.md:23-25; problem.md:38-41). |
| Feasibility / appetite | strong | Small: API pública preservada nos 8 call sites, sem dependências novas e sem superfície HTTP (concept.md:11; shape.md:16-19). |
| Strategic fit | adequate | Higiene de frontend bem delimitada, que abre mão de telemetria centralizada já inexistente neste checkout e mantém diagnóstico no processo local (concept.md:12; research.md:24-27). |
| Risk posture | strong | Nenhuma rota/serviço backend, nenhum `console.log` (regra no-console preservada), nenhuma persistência, reversível por `git revert` (define.md:14-20; shape.md:16-19). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: validade, evidência, valor, apetite e risco são `strong`, fit `adequate` pelo caráter local da mudança. A Option A de `concept.md` — coletor local com teto de 200 entradas e `snapshot()` — é a que encerra o 404 e o drift do contrato 002 preservando `debug/info/warn/error`; a Option B deixa o buffer sem limite e o contrato desatualizado, e a Option C cria ingestão não autenticada sem coletor (concept.md:27-29; shape.md:10-14). O comportamento real do `sendBeacon` em produção e um eventual coletor OTel externo permanecem `[NV]`, não bloqueantes do recorte aprovado (decision.md antigo:16-18).

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante do recorte aprovado.

## If go — Handoff to `/speckit.specify`

- **Problem**: O logger frontend promete envio a um endpoint de telemetria que não existe, descartando entradas com 404 silencioso e mantendo um contrato documental desatualizado.
- **Chosen approach**: Option A — coletor local com teto e diagnóstico (`concept.md:9-12,29`; forma em `shape.md:5-8`).
- **In scope / out of scope**: In — reescrever `logger.ts` como coletor em memória (remover import de `ApiClient`, `flush()`, `setInterval`, listener `beforeunload` e `sendBeacon`), buffer com teto de 200 entradas, `snapshot(): readonly LogEntry[]` com cópia, contrato 002 reconciliado e teste `logger.spec.ts` no projeto `server`. Out — rota/serviço backend, exporter OTel, dependências novas, `console.log` ou enfraquecimento da regra no-console, alteração dos 8 call sites, constituição, ADRs, PR #262 (DIRTY, não tocado), `localStorage`/persistência (decision.md antigo:9-15; define.md:16-20; shape.md:10-14).
- **Success metrics**: `rg '/api/v1/telemetry/logs' apps/frontend/src` → 0 ocorrências; teste unitário no projeto `server` prova ausência de `fetch`/`sendBeacon` e teto de 200; contrato 002 descreve apenas comportamento local; API pública inalterada com os 8 call sites compilando sem edição; gates frontend (check/lint/test/build) e Rust verdes (baseline: 2 ocorrências em `logger.ts:40,106`, nenhum teste de telemetria, contrato promete endpoint `flush()` inexistentes — problem.md:30-37).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: o teto de 200 entradas atende ao diagnóstico necessário ou deve ser revisado?]; [NEEDS CLARIFICATION: quem consumirá `snapshot()` para diagnóstico local?]; [NEEDS CLARIFICATION: existe coletor OTel externo previsto para este checkout? — `[NV]`]; [NEEDS CLARIFICATION: comportamento em browser real e CI ao vivo será observado em qual PR? — `[NV]`]; PR independente baseado em `github/dev@55363b3ca`; PR #262 (DIRTY) não tocado (decision.md antigo:14-18).
