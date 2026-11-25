# Problem Definition: Fronteira de origem e deriva de governança

- **Slug**: 032-next-vertical-slice
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

Em uma aplicação self-hosted multi-tenant, o campo `app.cors_origins` existe na configuração, mas é ignorado em tempo de execução: não há fronteira de origem efetiva no backend e valores inválidos (vazio, `*` ou não interpretável) não falham rápido (research.md:5; decision.md, "Scope (in)"). Isso deixa o Seed exposto a requisições cross-origin arbitrárias e retira do operador o controle sobre quais origens devem ser aceitas. Em paralelo, a documentação de governança permanece desatualizada (ROADMAP com intervalos `001-019`, OBJETIVO com "442", referências a ADR-0002), e a promoção de checks de segurança a required depende de autorização humana explícita (research.md:4). O problema é a combinação de uma lacuna de segurança executável e verificável com a incapacidade de afirmar honestamente o que ela cobre: CORS não protege WebSocket e não autoriza reivindicar o critério #2 de reentrada de ADR-0002 (decision.md, "Explicit non-goals" e "Risks/NVs").

## Affected Users & Stakeholders

- **Users**: operador de uma instalação self-hosted multi-tenant, que precisa limitar origens aceitas; membros da organização, que dependem da separação entre requisições legítimas e externas.
- **Stakeholders**: mantenedores e responsáveis pela segurança — precisam de uma checagem determinística e de limites de autorização claros; revisores de governança — precisam que os documentos reflitam o estado real.

## Goals

- Fazer a configuração de origens ser efetiva e rejeitar valores que não representem uma allowlist utilizável.
- Dispor de verificação determinística que distinga origem permitida de origem não permitida, incluindo o comportamento de preflight.
- Manter a documentação alinhada ao que foi implementado e ao que permanece fora de escopo.
- Respeitar a fronteira de autorização: pesquisa e rascunho são permitidos, promoção de required checks e alteração de proteções dependem de autorização humana.

## Non-Goals

- Não afirmar que o critério #2 de reentrada de ADR-0002 foi atendido — faltam URLs de runtime do frontend, prova ao vivo não-browser e verificação de `Origin` em WebSocket (decision.md).
- Não promover cargo-vet a required, não editar proteção de branch/CODEOWNERS/rulesets sem autorização, não criar tag/release, não executar load/restore e não introduzir OTLP.
- Não atribuir a CORS a proteção de WebSocket nem tratar bearer token como requisição credenciada.
- Não reabrir decisões de tenancy, RBAC/RLS ou Documentos FASE 7.

## Success Metrics

- Testes determinísticos cobrem preflight de origem permitida (ACAO/ACAM/ACAH, `Vary`, sem credenciais) e origem não permitida sem ACAO (baseline: nenhum teste; configuração ignorada).
- Configuração rejeita allowlist vazia, curinga `*` e valor não interpretável (baseline: ausência de fail-fast).
- Drift documental residual fechado em ROADMAP, OBJETIVO e referências a ADR-0002 (baseline: intervalos `001-019`, anotação "442", refs desatualizadas).
- Nenhuma mudança de proteção de branch ou required check sem autorização registrada (baseline: sob gestão humana; estado remoto [NV]).

## Cost of Inaction

Sem allowlist efetiva, uma instalação multi-tenant aceita requisições de origens não controladas e o operador não tem como limitá-las pela configuração; simultaneamente, afirmações documentais desatualizadas podem fazer o projeto reivindicar garantias que não possui. O custo é exposição de segurança e erosão de confiança na governança; nenhum impacto financeiro medido foi anexado [NV].

## Open Questions

- [NEEDS CLARIFICATION: quais URLs de runtime do frontend devem compor a allowlist fora dos quatro valores de dev/preview registrados?]
- [NEEDS CLARIFICATION: como será feita a verificação de `Origin` em WebSocket, hoje fora da proteção de CORS?]
- [NEEDS CLARIFICATION: quem autoriza a promoção de cargo-vet e a alteração de required checks/proteção de branch?]
- [NEEDS CLARIFICATION: qual é a origem autorizada da configuração em produção, dado que o loopback é apenas sinalizado como aviso?]
