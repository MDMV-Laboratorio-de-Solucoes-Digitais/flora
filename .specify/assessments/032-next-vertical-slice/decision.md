# Decision: 032 — Allowlist de origem e deriva de governança

- **Slug**: 032-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `app.cors_origins` existe mas é ignorado em runtime e valores vazios/`*`/ininterpretáveis não falham rápido (research.md:5; problem.md:9) — lacuna de segurança real em instalação self-hosted multi-tenant. |
| Evidence strength | strong | Research GO com duas correções obrigatórias (`axum::http`, fixture de header inválido com `\n`), gate de 4 testes e `AppConfig::validate()` já chamado em `main.rs:28` (research.md:3) — verificador executável, zero deps novas. |
| Value vs. inaction | strong | Sem allowlist efetiva o Seed aceita cross-origin arbitrário e o operador perde o controle pela configuração; custo é exposição e erosão de confiança (problem.md:37-39). |
| Feasibility / appetite | adequate | Medium: uma única chamada em `routes/mod.rs` atualizada atomicamente e nenhuma dependência nova, mas exige que o operador defina origens de produção hoje não listadas (concept.md:11-12). |
| Strategic fit | strong | Fecha lacuna de segurança com fronteira de autorização honesta — pesquisa/rascunho permitidos, promoção de required checks depende de autorização humana (research.md:4,11). |
| Risk posture | adequate | Muda comportamento cross-origin; CORS não protege WebSocket, bearer não é requisição credenciada e a fronteira de autorização é preservada (concept.md:12-13). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: o scorecard é forte em validade do problema, evidência executável e valor, com apetite e risco `adequate`. A recomendação de `concept.md` (Option A — origem efetiva com verificação determinística) é a única que satisfaz as métricas de preflight permitido/negado e rejeição de allowlist inválida sem promover checks ou criar release; a Option B perde a verificação positiva e a Option C não tem evidência de que o proxy preserva `Origin` (concept.md:27-29). As incertezas — URLs de runtime do frontend, verificação de `Origin` em WebSocket e autoridade para required checks — seguem como carried-forward `[NEEDS CLARIFICATION:]`, não bloqueantes deste recorte.

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante.

## If go — Handoff to `/speckit.specify`

- **Problem**: O campo `app.cors_origins` é decorativo e expõe o Seed a requisições cross-origin arbitrárias, enquanto a documentação de governança permanece desatualizada.
- **Chosen approach**: Option A — origem efetiva com verificação determinística (`concept.md:9-12,29`).
- **In scope / out of scope**: In — `AppConfig.cors_origins` (4 origens dev/preview exatas, fail-fast para vazio/`*`/ininterpretável, aviso de loopback em produção), `cors_layer(&[String])` com `AllowOrigin::list`, métodos/headers explícitos (incl. `authorization`), sem credenciais, chamada única em `routes/mod.rs`, testes determinísticos (preflight permitido com ACAO/ACAM/ACAH/`Vary`/sem credenciais, origem negada sem ACAO, rejeição de curinga, default aceito), `.env.example` com sintaxe de array Figment e nota de export para o processo, drift residual (banner de supersessão no doc de release 031, ROADMAP, contagem de isenções em OBJETIVO, refs de ADR-0002). Out — afirmar ADR-0002 #2 atendido, promover cargo-vet a required, editar proteção de branch/CODEOWNERS/rulesets, criar tag/release, executar load/restore, introduzir OTLP, atribuir proteção de WebSocket à CORS, tratar bearer como credenciado e reabrir tenancy/RBAC/RLS ou FASE 7 (decision.md antigo:8-16).
- **Success metrics**: Testes de preflight permitido/negado; rejeição de allowlist vazia/curinga/ininterpretável; drift residual fechado em ROADMAP/OBJETIVO/refs de ADR; nenhuma mudança de proteção sem autorização registrada (baseline: nenhum teste, config ignorada, ROADMAP `001-019`, OBJETIVO "442", estado remoto [NV] — problem.md:30-35).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quais URLs de runtime do frontend compõem a allowlist fora dos 4 valores dev/preview]; [NEEDS CLARIFICATION: como verificar `Origin` em WebSocket]; [NEEDS CLARIFICATION: quem autoriza a promoção de cargo-vet e a alteração de required checks/proteção de branch]; [NEEDS CLARIFICATION: qual é a origem autorizada em produção, dado que loopback é apenas aviso]; [NV: estado remoto das proteções]; 13 isenções informacionais e contagem 455 permanecem inalteradas (decision.md antigo:18-19).
