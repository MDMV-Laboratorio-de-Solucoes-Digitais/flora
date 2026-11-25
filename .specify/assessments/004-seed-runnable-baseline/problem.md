# Problem Definition: Seed sem baseline local reproduzível

- **Slug**: 004-seed-runnable-baseline
- **Created**: 2026-11-25
- **Inputs used**: intake.md, research.md, decision.md

## Problem Statement

O Seed v0.1 da Flora tinha implementação e specs substanciais (001-003, assessment 003), mas nenhum baseline local reproduzível que permitisse verificar a jornada ponta a ponta. No início do ciclo não havia `OBJETIVO.md` nem `ROADMAP.md`, e o checkout não continha um compose de dependências para Postgres/Valkey/Meilisearch/OIDC — apenas `dev-seed.sql` para smoke manual. Embora verificações determinísticas pontuais tenham sido executadas em 2026-11-15 (`cargo check -p flora-api -p flora-core`, `cargo fmt --all -- --check`, `cargo test --workspace --no-run`, 8 testes `flora-core --lib` e 54 `flora-tests --lib`), a integração com Postgres, Valkey, Meilisearch e OIDC não foi exercitada neste ambiente.

A consequência é que correções incrementais não podem ser validadas de forma confiável: o que está "pronto" depende de quem executa e de qual ambiente, e o drift entre tasks/checklists/README e a realidade se reproduz. Intake e research mostram que o gargalo não era outra feature isolada (WS fanout, indexação/notificação, contrato de arquivos, RBAC, quota, health checks), mas a ausência de um caminho mínimo, documentado e repetível para subir e checar o Seed. Escolher entre perfis dev (apenas infraestrutura essencial vs. incluir mock OIDC/Zitadel) e garantir reprodutibilidade sem segredos são incógnitas que aumentam o risco de qualquer avanço posterior.

## Affected Users & Stakeholders

- **Users**: desenvolvedores que tentam subir o Seed a partir de um checkout limpo e precisam de um caminho repetível de verificação; operadores de VPS barata que dependem de um Seed operável antes de adotar a alternativa.
- **Stakeholders**: operador do repositório, responsável por consolidar direção e autorizar ações fora do escopo autônomo; mantenedores de CI e documentação, que precisam de critérios de smoke estáveis; futuros usuários PME, representados pela necessidade de um piloto confiável.

## Goals

- Tornar explícito o estado real do Seed em documento de objetivo e roadmap, incluindo gaps e não-objetivos, sem presumir verificação não executada.
- Caracterizar a ausência de ambiente dev reproduzível como pré-requisito bloqueador para fatias verticais posteriores.
- Definir o que conta como smoke mínimo verificável do Seed e quais dependências são realmente necessárias nesse perfil.
- Reduzir o custo de iteração e o risco de "falso pronto" com evidências que possam ser registradas e repetidas.
- Preservar o princípio de baixo risco: nada de segredos reais, deploy de produção ou serviços pagos no caminho de baseline.

## Non-Goals

- Não detalhar neste artefato a solução de infraestrutura, compose, configuração ou lista de tarefas do ciclo.
- Não entregar feature de usuário final como meta deste problema; o foco é enablement de verificação.
- Não afirmar que integrações com Postgres/Valkey/Meilisearch/OIDC funcionam sem execução observada (NV).
- Não cobrir deploy/infra de produção, proteções remotas, merges/releases, segredos/credenciais reais, serviços pagos/proprietários, RE não autorizada ou reuso incompatível com AGPL.

## Success Metrics

- Tempo e número de etapas para obter um Seed local funcional a partir de checkout limpo (baseline: unknown — ausente compose de dependências e documentação consolidada no início do ciclo).
- Serviços externos com integração executada e evidenciada localmente (baseline: 0/NV — research.md marca integrações runtime não exercitadas).
- Comandos determinísticos locais com resultado registrado (baseline: parcial — check/fmt e 8+54 testes ok em 2026-11-15; integração e frontend parcialmente inconclusivos).
- Documentação de direção (OBJETIVO/ROADMAP) alinhada ao estado observável do repositório (baseline: documentos inexistentes no início).

## Cost of Inaction

Sem baseline reproduzível, cada nova fatia vertical é validada apenas por inspeção, tornando regressões e drift inevitáveis. O time gasta esforço repetindo setup ad hoc, e a promessa de "qualquer VPS barata" perde credibilidade porque ninguém consegue demonstrar o fluxo central em condições limpas. A ausência de objetivo e roadmap consolidados agrava o problema: priorizações passam a depender de percepções individuais, não de evidência.

## Open Questions

- [NEEDS CLARIFICATION: qual perfil dev mínimo deve compor o baseline (infra essencial apenas ou inclusão de mock OIDC/Zitadel), e com que critério essa escolha é aceita?]
- [NEEDS CLARIFICATION: Docker, serviços locais e credenciais OIDC estão disponíveis no ambiente para executar o smoke integrado, ou o resultado deve permanecer NV?]
- [NEEDS CLARIFICATION: quais são os critérios de smoke considerados suficientes para declarar o Seed local verificável?]
