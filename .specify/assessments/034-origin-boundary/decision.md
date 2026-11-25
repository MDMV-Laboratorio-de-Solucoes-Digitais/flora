# Decision: 034 — Fronteira de origem do WebSocket

- **Slug**: 034-origin-boundary
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O critério #2 de reentrada de ADR-0002 segue aberto: WS não verifica origem, o FE não resolve URLs de runtime e não há prova ao vivo não-browser (intake.md:5; research.md:5-7; problem.md:9). |
| Evidence strength | strong | 6 subagentes R1, plano exato GO (E1–E8), pesquisa de segurança em 7 linguagens/13 primários e prova ao vivo viável em instância isolada (research.md:4-7) — regra e evidência determinísticas. |
| Value vs. inaction | strong | Fecha uma não-verificação transversal antiga com testes determinísticos e evidência rastreável enquanto a árvore canônica está limpa (research.md:11; problem.md:37-39). |
| Feasibility / appetite | adequate | Medium, mas incerto: depende de janela isolada para a prova ao vivo (porta 3100 / `flora_034_t8`) e amplia superfície em FE, API e testes (concept.md:11-12; research.md:6). |
| Strategic fit | strong | Maior valor verificado por esforço: encerra o critério de reentrada com CIs capazes de prová-lo, diferindo Documentos FASE 7 e limpeza de affordances (research.md:7-8,11). |
| Risk posture | adequate | Prova não pode tocar o binário desatualizado em `:3000` nem o trabalho alheio em `/metrics`; origem é fronteira de navegador, não autenticação, e literais divergentes/proxy que remove o cabeçalho invalidam a garantia (concept.md:12-13). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: o scorecard é `strong` em validade, evidência, valor e fit, com apetite e risco `adequate` pela dependência de janela isolada. A Option A de `concept.md` — origem verificada com prova isolada — é a única que cobre todas as métricas (predicado puro, testes oneshot sem banco, prova ao vivo em :3100 e nota honesta na ADR); a Option B cobre só a checagem e a Option C não produz evidência versionada (concept.md:27-29). O critério #2 só é considerado fechado após E1–E8 verdes, prova ao vivo, lane de regressão em navegador e nota na ADR; até lá, o não-claim é preservado (decision.md antigo:15).

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante do recorte.

## If go — Handoff to `/speckit.specify`

- **Problem**: ADR-0002 #2 permanece aberto — o canal WebSocket aceita conexões sem examinar a origem, o FE copia literais em vez de resolver endpoints de runtime e falta prova ao vivo não-browser da fronteira.
- **Chosen approach**: Option A — origem verificada com prova isolada (`concept.md:9-12,29`).
- **In scope / out of scope**: In — em `flora-api/src/routes/websocket.rs`, predicado puro `origin_allowed()` + 403 antecipado antes de auth/DB (origem ausente aceita; `null` e não listadas negadas) e 3 testes puros; em `flora-tests/src/contract/websocket.rs`, 2 testes oneshot sem banco (`connect_lazy`); FE em `static/config.js`, `app.html`, `app.d.ts`, resolvers em `client.ts`, `websocket.ts` + mock, `tests/unit/resolver.test.ts`, `.env.example`; instância canônica isolada em :3100 / `flora_034_t8` para preflight T8, `select-organization` com Bearer e sondas de `Origin` de WS; nota no gate da ADR-0002 com evidência. Out — afirmar #2 atendido antes de E1–E8 + prova ao vivo + lane T7 em navegador + nota na ADR; Documentos FASE 7, limpeza de affordances, cargo-vet, tag/release, load/restore; tocar o processo em `:3000` ou o `/metrics` alheio (decision.md antigo:8-16).
- **Success metrics**: Predicado puro cobre ausência/`null`/não listada com testes determinísticos; 2 testes oneshot sem banco; provas ao vivo em :3100 com banco próprio (preflight, `select-organization` com Bearer, sondas de `Origin`); nota na ADR registra evidência e limites sem reivindicar além do provado (baseline: nenhuma checagem no WS, nenhum teste DB-free, binário desatualizado em `:3000` — problem.md:30-35).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: literais de origem necessários para Tauri em cada sistema operacional]; [NEEDS CLARIFICATION: o proxy reverso encaminha `Origin` ao backend?]; [NEEDS CLARIFICATION: como tratar o vazamento de token por `?token=`]; [NEEDS CLARIFICATION: quem executa e valida a lane de regressão em navegador]; [NEEDS CLARIFICATION: configuração de runtime é lida apenas no startup — comportamento após inicialização]; [NV: binário em `:3000` desatualizado e pertence a outro agente].
